use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::JwtError(_) => StatusCode::FORBIDDEN,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
