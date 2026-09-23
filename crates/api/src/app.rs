use axum::routing::get;
use tokio::net::TcpListener;

use crate::{
    routes::{download, health_check, search, stats},
    state::AppState,
};

pub struct App {
    listener: TcpListener,
    router: axum::Router,
}

impl App {
    pub fn new(state: AppState, listener: TcpListener) -> Self {
        let router = axum::Router::new()
            .route("/download", get(download))
            .route("/health", get(health_check))
            .route("/search", get(search))
            .route("/stats", get(stats))
            .with_state(state);

        Self { listener, router }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router.into_make_service()).await
    }
}
