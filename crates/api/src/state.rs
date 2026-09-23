use backend::errors::AppError;
use backend::models::{EsQuery, EsResults};

#[derive(Clone, Default)]
pub struct AppState {
    pub elasticsearch_url: String,
}

impl AppState {
    pub async fn search(&self, query: EsQuery) -> Result<EsResults, AppError> {
        let _ = reqwest::get(&self.elasticsearch_url).await;

        Err(AppError::E)
    }
}
