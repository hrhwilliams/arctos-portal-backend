#[must_use]
pub const fn one() -> i32 {
    1
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        assert!(one() == 1)
    }
}
