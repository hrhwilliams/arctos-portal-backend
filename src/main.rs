use arctos_portal_backend::app::App;
use tokio::net::TcpListener;

#[must_use]
pub const fn one() -> i32 {
    1
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;
    let app = App::new(listener);

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
