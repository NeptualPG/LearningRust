//Addcustom failure messages
pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        let result = greeting("Carol");
        // assert_eq!(result, "Hello, Carol!");
        assert!(result.contains("Hello"), "Greeting should contain 'Hello' but got: {}", result);
    }
} 