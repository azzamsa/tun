pub mod app;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match self {
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
            Error::NotFound(_) => StatusCode::NOT_FOUND,             // 404
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,    // 400
        };

        let err_response = ErrorResponse {
            message: format!("{}", &self),
        };

        (status_code, Json(err_response)).into_response()
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => Error::NotFound("env var not found".into()),
            _ => Error::Internal(err.to_string()),
        }
    }
}

impl std::convert::From<tracing_subscriber::filter::ParseError> for Error {
    fn from(err: tracing_subscriber::filter::ParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<tracing_subscriber::filter::FromEnvError> for Error {
    fn from(err: tracing_subscriber::filter::FromEnvError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}
