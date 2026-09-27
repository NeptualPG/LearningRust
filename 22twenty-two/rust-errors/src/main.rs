/* 
Error Handling in Rust

- Most programming languages have one way to handle errors (exceptions)
- Rust does not have exceptions 
- Rust differentiates between unrecoverable errors and recoverable errors
- Unrecoverable errors are handle by the panic! macro
- Recoverable errors are handled by the result<T, E> enum

*/


// Unrecoverable errors

fn main(){
    // panic!("crash and burn!"); // This will cause the program to terminate and print the error message // way to handle unrecoverable errors in Rust

    let v: Vec<i32> = vec![1, 2, 3];

    v[99]; // This will cause the program to terminate and print the error message
}

