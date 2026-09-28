/* 
Error Handling in Rust

- Most programming languages have one way to handle errors (exceptions)
- Rust does not have exceptions 
- Rust differentiates between unrecoverable errors and recoverable errors
- Unrecoverable errors are handle by the panic! macro
- Recoverable errors are handled by the result<T, E> enum

*/

use::std::fs::File;
use::std::io::ErrorKind; // This has diferent types of errors that can occur when working with files

// unwrap and expect


fn main(){
    // let greeting_from_file: File = File::open("hello.txt").unwrap();
    let greeting_from_file: File = File::open("hello.txt").expect("hello.txt should be included in this project");
    // unwrap is a shortcut method that retuns the value if Ok or panics if Err.  
    // expect is similar to unwrap but allows you to specify a custom panic message.
}