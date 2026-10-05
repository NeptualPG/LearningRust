//  A simple CLI in rust 
// This is a simple command line interface (CLI) application written in Rust.
// AND RETURN THE STRING REVERSED

use std::env;

fn main() {
    // collect the arguments in a vector
    let args: Vec<String> = env::args().collect();

    // check if the has at least one argument
    // by cheking the number of args

    //args[0] is the name of the program

    if args.len() < 2 {
        println!("Please provide a string to reverse.");
        return;
    }

    // store the string in a variable
    let mut input = args[1].clone();

    // reverse the string 
    let reversed: String = input.chars().rev().collect::<String>();



}
