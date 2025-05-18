use super::TokenVerify;
use axum::extract::Query;
use axum::{
    extract::Request,
    extract::State,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::headers::{HeaderMap, HeaderMapExt};
use axum_extra::{headers::Authorization, headers::authorization::Bearer};
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct Param {
    token: Option<String>,
}

pub async fn verify_token<T>(
    State(state): State<T>,
    headers: HeaderMap,
    Query(params): Query<Param>,
    mut req: Request,
    next: Next,
) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let option = headers.typed_get::<Authorization<Bearer>>();
    let token = if let Some(Authorization(bearer)) = option {
        bearer.token().to_string()
    } else if params.token.is_some() {
        params.token.unwrap()
    } else {
        let msg = "missing Authorization header";
        warn!(msg);
        return (StatusCode::UNAUTHORIZED, msg).into_response();
    };
    match state.verify(&token) {
        Ok(user) => {
            req.extensions_mut().insert(user);
        }
        Err(e) => {
            let msg = format!("verify token failed: {:?}", e);
            warn!(msg);
            return (StatusCode::FORBIDDEN, msg).into_response();
        }
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecodingKey, EncodingKey, User};
    use anyhow::{Context, Result};
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::middleware::from_fn_with_state;
    use axum::routing::get;
    use std::sync::Arc;
    use tower::ServiceExt;

    async fn handle() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }

    #[derive(Clone)]
    struct AppState(Arc<AppStateInner>);

    struct AppStateInner {
        ek: EncodingKey,
        dk: DecodingKey,
    }

    impl TokenVerify for AppState {
        type Error = ();

        fn verify(&self, token: &str) -> std::result::Result<User, Self::Error> {
            self.0.dk.verify(token).map_err(|_| ())
        }
    }

    impl AppState {
        fn build() -> Result<AppState> {
            let encoding_pem = include_str!("../../fixtures/encoding.pem");
            let decoding_pem = include_str!("../../fixtures/decoding.pem");
            let ek = EncodingKey::load(encoding_pem).context("ek load failed")?;
            let dk = DecodingKey::load(decoding_pem).context("dk load failed")?;
            Ok(AppState(Arc::new(AppStateInner { ek, dk })))
        }
    }
    #[tokio::test]
    async fn test_verify_token() -> Result<()> {
        let state = AppState::build()?;

        let user = User::new(1, "wu", "wu@github.org");
        let token = state.0.ek.sign(user)?;

        let app = Router::new()
            .route("/", get(handle))
            .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
            .with_state(state);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/?token={}", token))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Authorization", format!("Bearer {}", "bad token"))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/?token={}", "bad-token"))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        Ok(())
    }
}
