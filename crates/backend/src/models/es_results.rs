use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EsResults {
    pub page: Option<usize>,
}

impl EsResults {
    pub fn to_csv(&self) -> String {
        "".into()
    }
}
