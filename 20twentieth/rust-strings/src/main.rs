/*  String and UTF-8
UTF-8 is a variable-width encoding: represents unicode code points.
it's used to represent different characters: alphabets, emojis, etc.

What is a String?
A String in Rust is a collection of bytes.
More specifically, a String is a wrapper over a Vec<u8> buffer.
Rust has only one string type in the core language: str
it's usually seen in its borrowed form &str

We will discuss
1. Creating a new String
2. Updating a String
3. Concatenation
4. Indexxing into String
5. Slicing Strings
6. Iterating over Strings

*/

#![allow(unused)]
fn main() {
    // Creating a new String
    let mut s = String::new(); // similar to Vec<T>

    // Create a String with initial data
    let Data  = "initial contents"; // data is a &str
    let s = Data.to_string(); // to_string() method is used to create a new String


    // The method also works on a literal directly
    let s = "initial contents".to_string(); // type of s is String

    let s = String::from("initial contents"); // type of s is String
    println!("s: {}", s); 

    //UTF-8 encoded STRING 
    let hello = String::from("السلام عليكم"); // Arabic
    let hello = String::from("Dobrý den"); // Czech
    let hello = String::from("Hello"); // English
    let hello = String::from("שָׁלוֹם"); // Hebrew
    let hello = String::from("नमस्ते"); // Hindi
    let hello = String::from("こんにちは"); // Japanese
    let hello = String::from("안녕하세요"); // Korean
    let hello = String::from("你好"); // Chinese
    let hello = String::from("Olá"); // Portuguese
    let hello = String::from("Здравствуйте"); // Russian
    let hello = String::from("Hola"); // Spanish
    let hello = String::from("👋"); // Emoji

    // Update a String
    let mut s = String::from("hello");

    s.push_str(", world"); // push_str() method appends a string slice to a String

    println!("s: {}", s); // s: hello, world!

    // push a single character to a String using the push() method
    s.push('!'); // push() method appends a single character to a String
    println!("s: {}", s); // s: hello, world!!
}
