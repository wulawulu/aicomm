use crate::model::{CreateMessage, ListMessages};
use crate::{model::ChatFile, AppError, AppState, ErrorOutput};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    extract::{Multipart, Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Extension, Json,
};
use chat_core::{Message, User};
use tokio::fs;
use tracing::{info, warn};

/// List all messages in the chat.
#[utoipa::path(
    get,
    path = "/api/chats/{id}/messages",
    params(
         ("id" = u64, Path, description = "Chat id"),
    ),
    request_body(content = ListMessages, description = "获取消息", content_type = "application/json"),
    responses(
         (status = 200, description = "List of messages", body = Vec<Message>),
         (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    tag="message",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn list_message_handler(
    State(state): State<AppState>,
    Path(chat_id): Path<i64>,
    Query(input): Query<ListMessages>,
) -> Result<impl IntoResponse, AppError> {
    let messages = state.list_message(input, chat_id as _).await?;
    Ok(Json(messages))
}

/// Send a new message in the chat.
#[utoipa::path(
    post,
    path = "/api/chats/{id}/messages",
    params(
         ("id" = u64, Path, description = "Chat id"),
    ),
    request_body(content = CreateMessage, description = "创建消息", content_type = "application/json"),
    responses(
         (status = 201, description = "Message created", body = Message),
    ),
    tag="message",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn send_message_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(chat_id): Path<i64>,
    Json(input): Json<CreateMessage>,
) -> Result<impl IntoResponse, AppError> {
    let msg = state
        .create_message(input, chat_id as _, user.id as _)
        .await?;
    Ok((StatusCode::CREATED, Json(msg)))
}

#[utoipa::path(
    get,
    path = "/api/files/{ws_id}/{*path}",
    responses(
         (status = 200, description = "Chat users"),
    ),
    tag="message",
    security(
         ("token" = [])
    )
)]
#[allow(dead_code)]
pub(crate) async fn file_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((ws_id, path)): Path<(i64, String)>,
) -> Result<impl IntoResponse, AppError> {
    if user.ws_id != ws_id {
        return Err(AppError::NotFound(
            "File doesn't exist or you don't have permission".to_string(),
        ));
    }

    let base_url = state.config.server.base_url.join(ws_id.to_string());
    let path = base_url.join(path);
    if !path.exists() {
        return Err(AppError::NotFound("File doesn't exist".to_string()));
    }

    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    // TODO: streaming
    let body = fs::read(path).await?;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", mime.to_string().parse().unwrap());
    Ok((headers, body))
}

#[utoipa::path(
    post,
    path = "/api/upload",
    responses(
         (status = 200, description = "File created", body = Vec<String>),
    ),
    tag="message",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn upload_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let ws_id = user.ws_id as u64;
    let base_dir = &state.config.server.base_url;

    let mut files = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().map(|name| name.to_string());
        let (Some(filename), Ok(data)) = (filename, field.bytes().await) else {
            warn!("Failed to read multipart field");
            continue;
        };

        let file = ChatFile::new(ws_id as _, &filename, &data);
        let path = file.path(base_dir);
        if path.exists() {
            info!("File {} already exists: {:?}", filename, path);
        } else {
            fs::create_dir_all(path.parent().expect("file path parent should exists")).await?;
            fs::write(path, data).await?;
        }
        files.push(file.url());
    }

    Ok(Json(files))
}
