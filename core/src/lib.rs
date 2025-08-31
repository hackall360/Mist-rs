pub fn greet() -> &'static str {
    "Hello from mist-core"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_message() {
        assert_eq!(greet(), "Hello from mist-core");
    }
}
