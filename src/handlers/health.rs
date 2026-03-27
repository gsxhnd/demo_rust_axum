use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

pub async fn health_check() -> impl IntoResponse {
    info!("Health check requested");
    (StatusCode::OK, "Server is running ✓")
}
