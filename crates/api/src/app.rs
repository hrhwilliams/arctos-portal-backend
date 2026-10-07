use axum::routing::get;
use tokio::net::TcpListener;

use crate::{routes, state::AppState};

pub struct App {
    listener: TcpListener,
    router: axum::Router,
}

impl App {
    pub fn new(state: AppState, listener: TcpListener) -> Self {
        let router = axum::Router::new()
            .route("/download", get(routes::download))
            .route("/healthy", get(routes::health_check))
            .route("/relations", get(routes::relations))
            .route("/schema", get(routes::schema))
            .route("/search", get(routes::search))
            .route("/stats", get(routes::stats))
            .route("/summary", get(routes::summary))
            .route("/taxa", get(routes::taxa))
            .with_state(state);

        Self { listener, router }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router.into_make_service()).await
    }
}
