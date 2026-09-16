#[must_use]
pub const fn one() -> i32 {
    1
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        assert_eq!(one(), 1);
    }
}
