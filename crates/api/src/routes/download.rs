use axum::{extract::State, response::IntoResponse};

use crate::{errors::ApiError, state::AppState};

#[tracing::instrument(skip(app_state))]
pub async fn download(State(app_state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    Ok(())
}
