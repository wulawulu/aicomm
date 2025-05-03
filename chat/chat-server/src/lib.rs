mod config;
mod error;
mod handler;
mod middleware;
mod model;
mod openapi;

use crate::middleware::verify_chat;
use crate::openapi::OpenApiRouter;
use anyhow::Context;
use axum::http::Method;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use chat_core::{set_layer, verify_token, DecodingKey, EncodingKey, TokenVerify, User};
pub use config::AppConfig;
pub use error::AppError;
pub use error::ErrorOutput;
use handler::*;
pub use model::{CreateChat, CreateMessage, CreateUser, ListMessages, SigninUser};
use sqlx::PgPool;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use tower_http::cors::{self, CorsLayer};

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        })
    }
}

impl TokenVerify for AppState {
    type Error = AppError;

    fn verify(&self, token: &str) -> Result<User, Self::Error> {
        self.dk.verify(token).map_err(AppError::from)
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let chat = Router::new()
        .route(
            "/{id}",
            get(get_chat_handler)
                .patch(update_chat_handler)
                .delete(delete_chat_handler),
        )
        .route(
            "/{id}/messages",
            get(list_message_handler).post(send_message_handler),
        )
        .layer(from_fn_with_state(state.clone(), verify_chat))
        .route("/", get(list_chat_handler).post(create_chat_handler));

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);
    let api = Router::new()
        .route("/users", get(list_chat_users_handler))
        .nest("/chats", chat)
        .route("/upload", post(upload_handler))
        .route("/files/{ws_id}/{*path}", get(file_handler))
        .route_layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
        .route("/signup", post(signup_handler))
        .route("/signin", post(signin_handler))
        .layer(cors);

    let app = Router::new()
        .openapi()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);
    Ok(set_layer(app))
}

#[cfg(feature = "test-util")]
mod test_util {
    use crate::{AppConfig, AppError, AppState, AppStateInner};
    use anyhow::Context;
    use chat_core::{DecodingKey, EncodingKey};
    use sqlx::{Executor, PgPool};
    use sqlx_db_tester::TestPg;
    use std::sync::Arc;

    impl AppState {
        pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
            let config = AppConfig::load()?;
            let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
            let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
            let post = config.server.db_url.rfind('/').expect("invalid db_url");
            let server_url = &config.server.db_url[..post];
            let (tdb, pool) = get_test_pool(Some(server_url)).await;
            let state = Self {
                inner: Arc::new(AppStateInner {
                    config,
                    dk,
                    ek,
                    pool,
                }),
            };
            Ok((tdb, state))
        }
    }

    pub async fn get_test_pool(url: Option<&str>) -> (TestPg, PgPool) {
        let url = match url {
            None => "postgres://postgres:postgres@localhost:5432".to_string(),
            Some(url) => url.to_string(),
        };
        let tdb = TestPg::new(url, std::path::Path::new("../migrations"));
        let pool = tdb.get_pool().await;

        let sql = include_str!("../fixtures/test.sql").split(";");
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");
        (tdb, pool)
    }
}
