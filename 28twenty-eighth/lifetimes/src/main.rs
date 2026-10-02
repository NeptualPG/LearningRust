
// Static Lifetime
// The 'static lifetime is the entire duration of the program'. 

#![allow(unused)]
fn main(){
    let s: &'static str = "I have a static lifetime.";
}