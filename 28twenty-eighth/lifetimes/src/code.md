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