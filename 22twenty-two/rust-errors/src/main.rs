/* 
Error Handling in Rust

- Most programming languages have one way to handle errors (exceptions)
- Rust does not have exceptions 
- Rust differentiates between unrecoverable errors and recoverable errors
- Unrecoverable errors are handle by the panic! macro
- Recoverable errors are handled by the result<T, E> enum

*/

// Unrecoverable errors

enum Reslt<T, E> {
    Ok(T),
    Err(E),
} // generics concrete value of the enum can be any type, and the error type can be any type as well.

fn main(){

}

