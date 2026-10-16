use std::fs::read_to_string;

fn main() {
    let filename = "non_existent_file.txt";

    if let Err(_e) = read_to_string(filename) {
        println!("This is an error");
    }

}
