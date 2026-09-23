use axum::{
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::{IntoResponse, Response},
};

pub async fn berkeleymapper() -> Response {
    (
        [
            (CONTENT_TYPE, "application/xml;charset=utf-8"),
            (CACHE_CONTROL, "public, max-age=3600"),
        ],
        include_str!("berkeleymapper.xml"),
    )
        .into_response()
}
