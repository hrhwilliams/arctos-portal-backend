use axum::response::IntoResponse;

/// Poll to check the status of the backend service
#[tracing::instrument]
pub async fn health_check() -> impl IntoResponse {}
