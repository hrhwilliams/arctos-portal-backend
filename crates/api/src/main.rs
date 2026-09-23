use api::{app::App, state::AppState};
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[must_use]
pub const fn one() -> i32 {
    1
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be in range 0-65535");

    let elasticsearch_url =
        std::env::var("ELASTICSEARCH_URL").expect("ELASTICSEARCH_URL must be set");

    let state = AppState { elasticsearch_url };
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    let app = App::new(state, listener);
    app.run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        assert_eq!(one(), 1);
    }
}
