use std::fs::read_to_string;

fn main() {
    let filename = "non_existent_file.txt";

    if let Err(_e) = read_to_string(filename) {
        // print to stdout 
        println!("This is printend on STDOUT");

        // print to stderr
        eprint!("This is printend on STDERR");
    }

}
