// Import the replace function from the std::mem module
use std::mem::replace;

// Define a generic function to swap two values
fn swap<T>(x: &mut T, y: &mut T) {
    let temp = replace(x, replace(y, replace(x, temp)));
}

fn main() {
    let mut a = 5;
    let mut b = 10;

    println!("Before swap: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap: a = {}, b = {}", a, b);

    let mut x = "hello";
    let mut y = "world";

    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);
}

// Use generics

// we import the replace function from the std::mem module at the top of the file
// using use std::mem::replace;.

// this allows us to call replace directly within the swap function
// without needing the full path std::mem::replace.