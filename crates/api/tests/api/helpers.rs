use api::{app::App, state::AppState};
use std::sync::LazyLock;
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
});

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let listener = TcpListener::bind("localhost:0")
        .await
        .expect("failed to bind random port");
    let port = listener
        .local_addr()
        .expect("failed to retrieve port")
        .port();

    let elasticsearch_url =
        std::env::var("ELASTICSEARCH_URL").unwrap_or("http://localhost:9200".into());

    let state = AppState { elasticsearch_url };
    let app = App::new(state, listener);

    tokio::spawn(async move {
        app.run().await.unwrap();
    });

    TestApp {
        address: format!("http://localhost:{}", port),
    }
}
