// Performance of code using Generics in Rust

#![allow(unused)]
fn main(){
    let integer = Some(5);
    let float = Some(5.0);
}


/*
When Rust compiles this code, it performs monomorphization.
During monomorphization, the Rust compiler reads the values that have been used
in the code and identifies the types
It then generates the code for the specific types used in the code.
This means that the code that is generated is specific to the types used in the code.
This process is what makes Rust code using generics fast at runtime
*/

// at compile tume
enum option_i32 {
    Some(i32),
    None,
}

enum option_f64 {
    Some(f64),
    None,
}

// Monomorphization is the process of generating specific code for each 
// type used in a generic function or data structure. In this case, the Rust 
// compiler generates two separate enums: `option_i32` for `Some(i32)` and `option_f64` for `Some(f64)`. 
// This allows the compiler to optimize the code for each specific type, resulting in better performance at runtime.


fn main () {
    let integer = option_i32::Some(5);
    let float = option_f64::Some(5.0);
}

// The generic Type T is replaced with the specific type used in the code.