use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Host not found: {0}")]
    HostNotFound(String),

    #[error("Route path not found: {0}")]
    RoutePathNotFound(String),

    #[error("Route method not allowed: {0}")]
    RouteMethodNotAllowed(Method),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = match self {
            AppError::HostNotFound(_) => StatusCode::NOT_FOUND,
            AppError::RoutePathNotFound(_) => StatusCode::NOT_FOUND,
            AppError::RouteMethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            AppError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, self.to_string()).into_response()
    }
}
