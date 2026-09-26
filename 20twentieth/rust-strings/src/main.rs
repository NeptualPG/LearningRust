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

    // Concatenation with + operator

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Pinche rust does not allow to add two String values together with the + operator.
    // Instead, we can use the format! macro or the push_str method.
    // IT DOESN'T WORK:
    // let s3 = s1 + s2;
    // let s3 = &s1 + &s2;
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("s3: {}", s3); // s3:

    //sIGNATURE OF THE ADD METHOD
    // fn add(self, s: &str) -> String {

    // concatenate 3 strings
    let one = String::from("one");
    let two = String::from("two");
    let three = String::from("three");

    // Concatenate using the + operator
    // this does not work because the + operator only works with two strings at a time
    // let combined = one + "-" + &two + "-" + &three; // one-two-three
    // Instead, use the format! macro
    let combined = format!("{}-{}-{}", one, two, three); // one-two-three
    println!("combined: {}", combined); // combined: one-two-three

    let s1 = String::from("hello");

    // this is a error:
    // let h = s1[1]; // this is a slice of the string, it is a &str
    // different bytes can represent different characters, 
    // so indexing into a string is not allowed in Rust. 
    // pinche Rust does not allow indexing into a String.
    // let h = &s1[1];
    // intead we can use:
    let h = &s1[0..1]; // this is a slice of the string, it is a &str

    //Slicing Strings 
    // Namaste in Hindi
    let hello = String::from("नमस्ते");
    // we get the letter "न" by slicing the string from index 0 to 3
    let s = &hello[0..3]; // this is a slice of the string, it is a &str 
    println!("s: {}", s);

    // iterating over Strings
    // we can iterate over the string using the chars() method
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    /// iterate over string by bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);  
    } 
}
