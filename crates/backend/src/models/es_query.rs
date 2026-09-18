use serde::Deserialize;

use crate::{errors::AppError, models::ArctosSearch};

#[derive(Debug, Deserialize)]
pub struct EsQuery {
    pub page: Option<usize>,
}

impl TryFrom<ArctosSearch> for EsQuery {
    type Error = AppError;

    fn try_from(value: ArctosSearch) -> Result<Self, Self::Error> {
        Err(AppError::E)
    }
}
