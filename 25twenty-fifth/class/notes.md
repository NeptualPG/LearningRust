GENERICS
We already saw generics!

Option<T>
* Used to handle nullable values safely 
* Example: let some_number:Opiton<i32> = Some(5);
  
Vec<T>
* A resizable array type.
* Example: let numbers: Vec<i32> = vec![1,2,3];

HashMap<K,V>
* a key-value store for efficient data retrieval.
* Example: let mut score: HashMap<String, i32> = HashMap::new();

Result<T,E>
* Used for error handling
* Example: let result: Result<i32,String> = Ok(10);


TRAITS

Trait definition: Summary trait defines a summarize method that types must implement.

Trait implementation: NewsArticle and Tweet structs each provide their own summary implementation.

Trait use: Create instances of NewsArticle and Tweet, then call summarize to print.

Benefits
* Abstraction: Defines shared behavior in an abstract way.
* Reusability: Allows multiple types to implement the same behavior.
* Polymorphism: Enables functions to operate on any type that implements the trait.


Lifetimes:

Lifetimes ensure that references are valid for as long
as ther are needed to prevent dangling references 

fn longest<'a> (x:'a str, y:&'a str) -> &'a str: This defines a function named longest with a lifetime parameter'a 
* <'a>: this specifies a generic lifetime parameter named 'a.
* x: &'a str: The first parameter is a string slice that must live at least as long as the lifetime 'a
* y: &'a str: The secound parameter is a string slice with the same lifetime 'a
* -> &'a str: The return type is a string slice that lives at least as long as the lifetime 'a

Summary 

Generics
* Definition: Write flexible and reusable code with type placeholder
* Benefits: Flexibility and reusability.

Trait
* Definition: Define share behavior that types must implement
* Benefits: Abstraction and polymorphism

Lifetime
* Definition: Ensure references are valid, prevent dangling references.
* Benefits: safety and clarity