use axum::response::{IntoResponse, Response};
use axum::{http, http::StatusCode, Json};
use chat_core::AgentError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] josekit::JoseError),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] http::header::InvalidHeaderValue),

    #[error("ai agent error: {0}")]
    AiAgentError(#[from] AgentError),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("create agent error: {0}")]
    CreateAgentError(String),

    #[error("update agent error: {0}")]
    UpdateAgentError(String),

    #[error("{0}")]
    ChatFileError(String),

    #[error("create message error: {0}")]
    CreateMessageError(String),

    #[error("user {user_id} is not member of chat {chat_id}")]
    NotChatMemberError { user_id: u64, chat_id: u64 },

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("update chat error: {0}")]
    UpdateChatError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            AppError::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::CreateChatError(_) => StatusCode::BAD_REQUEST,
            AppError::UpdateChatError(_) => StatusCode::BAD_REQUEST,
            AppError::CreateAgentError(_) => StatusCode::BAD_REQUEST,
            AppError::UpdateAgentError(_) => StatusCode::BAD_REQUEST,
            AppError::NotChatMemberError { .. } => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ChatFileError(_) => StatusCode::BAD_REQUEST,
            AppError::CreateMessageError(_) => StatusCode::BAD_REQUEST,
            AppError::AiAgentError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}
impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}
