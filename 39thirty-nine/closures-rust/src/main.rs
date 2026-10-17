/*Closure in Rust 

Closures are block of code that can be :

- stored in variables
- passed as arguments to functions
- returned from functions

They are similar to functions but with one main difference:

THey can capture variables from their surrounding scope.

key features:

 - Anonymous Functions:
    Closure are unnamed functions that can be stored in variables or passed to other functions.

 - Capturing Enviroment:
    Closures can capture values from their surrounding scope by borrowing, mutably borrowing, or taking ownershop of them.

 - Type Inferemce: 
    Rust interface the types of parameters and return types in most closures, so explicit type annotation are often 

 - Flexibility:
    Closures can be stored as function pointer or as traits like `Fn`, `FnMut`, and `FnOnce, depending on how they capture variables.


*/

fn main() {
     
     // The syntax for closure is similar to function sytax
     // || { code }

     // example1 
     let closure = || "Hello World"; // closure that takes no parameters and returns a string
     println!("{}", closure()); // call the closure and print the result

     // example2

     let add = |x: i32, y: i32| -> i32 { x + y }; // closure that takes two parameters and returns their sum
     println!("The result of adding 5 and 10 is: {}", add(5, 10)); // call the closure and print the result
     
     let hello = String::from("Hello"); 
     let world = String::from("World");
     let add = |x, y| x + y;

    println!("The result of adding 5 and 10 is: {}", add(5, 10));

    // Capturing Cariables with closures
    // There can 3 diffrent ways to capture variables with closures:
    // 1. By borrowing a variable immutably
    // 2. By borrowing a variable mutably
    // 3. By taking ownership
 
    // example 3 capturing by borrowing
    let x = 60;
    let print_x = || println!("The value of x is: {}", x); // closure that captures x by borrowing it immutably

    print_x(); // call the closure and print the value of x

    //example5 - capturing by mutable borrowing
    let mut y = 100;

    let mut print_y = || {
        y += 1; // closure that captures y by borrowing it mutably
        println!("The value of y is: {}", y);
    };

    print_y(); // call the closure and print the value of y}

    // example -capturing by taking ownership
    let z = String::from("Hello");

    let print_z = move || {
        println!("The value of z is: {}", z)
        drop(z); // closure that captures z by taking ownership of it
    }; 

    print_z();
}