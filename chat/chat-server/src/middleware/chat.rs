use crate::{AppError, AppState};
use axum::extract::Path;
use axum::{
    extract::Request,
    extract::State,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use chat_core::User;

pub async fn verify_chat(
    State(state): State<AppState>,
    Path(chat_id): Path<u64>,
    user: Extension<User>,
    req: Request,
    next: Next,
) -> Response {
    if !state
        .is_chat_member(chat_id, user.id as _)
        .await
        .unwrap_or_default()
    {
        let err = AppError::CreateMessageError(format!(
            "User {} are not as member of chat {chat_id}",
            user.id
        ));
        return err.into_response();
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::middleware::from_fn_with_state;
    use axum::routing::get;
    use axum::Router;
    use chat_core::verify_token;
    use tower::{ServiceBuilder, ServiceExt};

    async fn handle() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }

    #[tokio::test]
    async fn test_verify_chat() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let user = state
            .find_user_by_id(1)
            .await
            .expect("user not found")
            .expect("user not found");
        let token = state.ek.sign(user).expect("signing failed");

        let app = Router::new()
            .route("/chat/{id}/messages", get(handle))
            .layer(
                ServiceBuilder::new()
                    .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
                    .layer(from_fn_with_state(state.clone(), verify_chat)),
            )
            .with_state(state);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/chat/1/messages")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/chat/10/messages")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        Ok(())
    }
}
