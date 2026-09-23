use axum::{
    Form, Json,
    extract::State,
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::IntoResponse,
};
use backend::{
    models::{ArctosSearch, Format},
    utils::timestamp,
};

use crate::{errors::ApiError, state::AppState};

#[tracing::instrument(skip(app_state))]
pub async fn search(
    State(app_state): State<AppState>,
    Form(arctos_search): Form<ArctosSearch>,
) -> Result<impl IntoResponse, ApiError> {
    let format = arctos_search.format;
    let results = app_state.search(arctos_search.try_into()?).await?;

    Ok(match format {
        Format::Json => Json(results).into_response(),
        Format::Csv => (
            [
                (CONTENT_TYPE, "text/csv;charset=utf-8".to_owned()),
                (
                    CONTENT_DISPOSITION,
                    format!("attachment;filename=\"search_{}.csv\"", timestamp()),
                ),
            ],
            results.to_csv(),
        )
            .into_response(),
    })
}
