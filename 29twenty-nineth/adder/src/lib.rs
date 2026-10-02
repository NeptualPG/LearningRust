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
