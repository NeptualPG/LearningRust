```rust
/* How to write test in Rust

Tests in Rust are written within the same file as the code they are testing.
The bodies of tests are marked with the #[test] attribute, 
and they are run using the cargo test command.

1. set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect them to be.
*/

// Anatomy of a test function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn testing_function(){
        let result: usize = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_will_fail(){
        panic!("Make this test fail");
    }
}


fn main() {
    
}
```
-------------------------------
```rust

// checking results with the assert! macro

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height || self.width > other.height && self.height > other.width
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // if the test isn't true is going to fail
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 2,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 2,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

```

-----------------


```rust
// Testing Equality with assert_eq! and assert_ne! Macros

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        let result = add_two(3);
        assert_eq!(result, 5); // This will pass
    }

    #[test]
    fn test_add_two_not_equal() {
        let result = add_two(3);
        assert_ne!(result, 6); // This will pass
    }
}
```

---------------


```rust
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

```