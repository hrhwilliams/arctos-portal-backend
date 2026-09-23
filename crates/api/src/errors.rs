use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use backend::errors::AppError;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
    detail: String,
}

pub enum ApiError {
    Internal(String),
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        match value {
            AppError::E => Self::Internal("guh!".into()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal(s) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    error: "Internal server error".into(),
                    detail: s,
                }),
            )
                .into_response(),
        }
    }
}
