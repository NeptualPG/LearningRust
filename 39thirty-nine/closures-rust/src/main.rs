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