
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
