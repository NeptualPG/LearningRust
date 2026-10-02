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

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // panic attribute indicates that this test should panic 
    fn greater_than_100() {
        Guess::new(200);
    } 
}