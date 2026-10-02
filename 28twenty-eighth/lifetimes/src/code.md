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

-------------


// Lifetime annotation syntax
fn main (){
    let string1 = String::from("abcd");
    let result: &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is: {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


-------------------------------


fn main(){
    let string1 = String::from("Hello");
    let string2 = "World";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

}


// if I am using anotation into the return type, I have to use the same lifetime annotation in the parameters of the function. This is because the return type is a reference that is tied to the lifetimes of the input parameters. By using the same lifetime annotation, we ensure that the returned reference is valid for as long as the input references are valid.
fn longest <'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("longest string");
    result.as_str()
}


---------------------------------------------------


//Life tiem Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn first_word(s: &'a str) -> &str {

fn main() {
    let my:string = String::from("Hello World");

    let word: &str = first_word(&my);

    let my_literal: &str = "Hello World";
    
    let word: &str = first_word(&my_literal[..]);

    let word: &str = first_word(my_literal);
}

// Three rules for lifetimes
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all
// that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, 
// but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.


----------------------------------


// Life annotations in Method Definitions

struct ImporTantExcerpt<'a> {
    name: String,
}

impl <'a> ImporTantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl <'a> ImporTantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        &self.name
    }
}

fn main(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImporTantExcerpt {
        name: String::from(first_sentence),
    };
}