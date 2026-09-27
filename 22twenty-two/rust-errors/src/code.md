
// Unrecoverable errors

fn main(){
    // panic!("crash and burn!"); // This will cause the program to terminate and print the error message // way to handle unrecoverable errors in Rust

    let v: Vec<i32> = vec![1, 2, 3];

    v[99]; // This will cause the program to terminate and print the error message
}

