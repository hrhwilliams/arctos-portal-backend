use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EsResults {
    pub page: Option<usize>,
}

impl EsResults {
    #[must_use]
    pub const fn to_csv(&self) -> String {
        String::new()
    }
}
