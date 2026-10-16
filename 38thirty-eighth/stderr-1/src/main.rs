use std::fs::read_to_string;

fn main() {
    let filename = "non_existent_file.txt";

    if let Err(_e) = read_to_string(filename) {
        // print to stdout 
        println!("This is printend on STDOUT");

        // print to stderr
        eprint!("This is printend on STDERR");
        eprintln!();

        // print in bold red text to for eprintln
        eprintln!("\x1b[1;31mThis is printend on STDERR in bold red text\x1b[0m");
    }

}
