/* Smart Pointers in Rust

Apointer si a general concept for a variable that contains a memory address (example: references in Rust).

Smart pointers are data structures that act like pointers but have additional metadata and capabilities. They are often used to manage memory and resources in a safe and efficient manner.

Example of smart pointers we have seen so far:
- String 
- Vec<T>

The smart pointer pattern is a implemented using structs

They implement the Deref and Drop traits:
- Deref trait allows an instance of the smart pointe struct to be treated leke a regular reference.
- Drop trait allows to customize the behavior when an instance of the smart pointer goes out of scope.

Main smart pointers in Rust:
- The Box<T> for allocating values on the heap is a smart pointer.
- The Rc<T> type is a reference conting smart pointer and anables multiple ownership of data.
- The RefCell<T> type is a smart pointer that allows for mutable borrows checked at runtime.

We'll also cover
- Interior mutability pattern: a design pattern in Rust that allows you to mutate data even when
there are immutable references to that data.
- Reference cycles: a scenario in which two references contain each other, preventing either value from being dropped 

*/

fn main() {
    // Box<T> for allocating values on the heap

    // Creating a Box<T> to store an i32 value on the heap insstead of the stachQ

    let b = Box::new(5);

    println!("b = {}", b);
}
