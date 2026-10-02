/* LIFETIMES

Lifetimes in Rust are:
- a tool to ensure memory safety.
- a way to tell the compiler that references are valid for a certain amount of time.
- a type of generic, but they are not like the other generics we have seen so far.

This is not a concept that is present in other languages,
so it can be confusing at first.
*/


// Preventing Dangling References with Lifetimes

// borrow checker example: 

fn main() {
    let r: &i32;
    {
        let x = 5;
        r = &x; // ERROR: `x` does not live long enough
    }
    println!("r: {}", r);
}

// solution

fn main() {
    let r: &i32;
    let x = 5;
    r = &x; 
    println!("r: {}", r);
}

