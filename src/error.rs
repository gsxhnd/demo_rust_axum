use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    ValidationError(String, Option<Vec<FieldError>>),
    DatabaseError(String),
    InternalServerError(String),
}

#[derive(Debug, serde::Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::ValidationError(msg, _) => write!(f, "Validation error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::ValidationError(msg, _) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let mut body = json!({
            "error": error_message
        });

        if let AppError::ValidationError(_, Some(field_errors)) = &self {
            body["fields"] = json!(field_errors);
        }

        (status, Json(body)).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        tracing::warn!("Database error: {}", err);
        match &err {
            DbErr::RecordNotFound(msg) => AppError::NotFound(msg.clone()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}
