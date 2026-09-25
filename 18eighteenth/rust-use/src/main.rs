//main.rs
// use external packages

//dependencies are specified in Cargo.toml file
use rand; // for random number generation

// std is the stadard library of rust
// Include: 
// std::collections::HashMap; // for using HashMap
// std::io; // for input and output
// std::fs; // for file system operations
// std::env; // for environment variables
// std::thread; // for multithreading
// std::time; // for time operations
// std::sync; // for synchronization primitives
// std::net; // for networking
// std::process; // for process management
// std::path; // for path operations
// std::str; // for string operations
// etc...
use std::collections::HashMap; // for using HashMap

fn main(){
    // old way of generating a random number
    // let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number between 1 and 100
    let secret_number = rand::random_range(1..=100);
    println!("The secret number is: {}", secret_number);
    
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("The map is: {:?}", map);
}
