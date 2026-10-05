// import the env module
use std::env;

fn main() {
    // Read the arguments passed to the program
    let args: Vec<String> = env::args().collect();
    // Print the arguments
    dbg!(&args);

    //save the arguments in variables
    let arg1 = &args[1];
    let arg2 = &args[2];

    // print the variables
    println!("Argument 1: {}", arg1);
    println!("Argument 2: {}", arg2);

}
