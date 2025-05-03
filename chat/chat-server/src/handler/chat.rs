use crate::model::{CreateChat, UpdateChat};
use crate::{AppError, AppState, ErrorOutput};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chat_core::{Chat, User};

/// List all chats in the workspace of the user.
#[utoipa::path(
    get,
    path = "/api/chats",
    responses(
         (status = 200, description = "List of chats", body = Vec<Chat>),
    ),
    tag="chat",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.fetch_chats(user.id as _, user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(chat)))
}

/// Create a new chat in the workspace of the user.
#[utoipa::path(
    post,
    path = "/api/chats",
    responses(
         (status = 201, description = "Chat created", body = Chat),
    ),
    tag="chat",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn create_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state
        .create_chat(input, user.id as _, user.ws_id as _)
        .await?;
    Ok((StatusCode::CREATED, Json(chat)))
}

/// Get the chat info by id.
#[utoipa::path(
    get,
    path = "/api/chats/{id}",
    params(
         ("id" = u64, Path, description = "Chat id")
    ),
    responses(
         (status = 200, description = "Chat found", body = Chat),
         (status = 404, description = "Chat not found", body = ErrorOutput),
    ),
    tag="chat",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn get_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.get_chat_by_id(id).await?;
    match chat {
        None => Err(AppError::NotFound(format!("chat id {id}"))),
        Some(chat) => Ok(Json(chat)),
    }
}

#[utoipa::path(
    patch,
    path = "/api/chats/{id}",
    params(
         ("id" = u64, Path, description = "Chat id"),
    ),
    request_body(content = UpdateChat, description = "update chat", content_type = "application/json"),
    responses(
         (status = 200, description = "Chat found", body = Chat),
         (status = 404, description = "Chat not found", body = ErrorOutput),
    ),
    tag="chat",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn update_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.update_chat_by_id(id, input).await?;
    Ok(Json(chat))
}

#[utoipa::path(
    delete,
    path = "/api/chats/{id}",
    params(
         ("id" = u64, Path, description = "Chat id")
    ),
    responses(
         (status = 200, description = "Chat deleted"),
    ),
    tag="chat",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn delete_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_chat_by_id(id).await?;
    Ok(StatusCode::OK)
}
