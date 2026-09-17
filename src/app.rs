use axum::routing::get;
use tokio::net::TcpListener;

use crate::routes::health_check;

pub struct App {
    listener: TcpListener,
    router: axum::Router,
}

impl App {
    pub fn new(listener: TcpListener) -> Self {
        let router = axum::Router::new().route("/health", get(health_check));

        Self { listener, router }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router.into_make_service()).await
    }
}
