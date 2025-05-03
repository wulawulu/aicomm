use axum::response::{IntoResponse, Response};
use axum::{http, http::StatusCode, Json};
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
    JwtError(#[from] jwt_simple::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] http::header::InvalidHeaderValue),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("{0}")]
    ChatFileError(String),

    #[error("create message error: {0}")]
    CreateMessageError(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("update chat error: {0}")]
    UpdateChatError(String),

    #[error("Not found: {0}")]
    NotFound(String),
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
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ChatFileError(_) => StatusCode::BAD_REQUEST,
            AppError::CreateMessageError(_) => StatusCode::BAD_REQUEST,
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
