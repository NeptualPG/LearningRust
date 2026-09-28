use::std::fs::File;
use::std::io::ErrorKind;
use::std::io::{self, Read}; 


// use the ? operator to return the error if it occurs
fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // The ? operator can be used to simplify the code above. 
    // It will return the error if it occurs, otherwise it will continue executing the function.

    // let mut username_from_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_from_file.read_to_string(&mut username)?;
    // Ok(username)

    // let mut username = String::now();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    fs::read_to_string("hello.txt")
}

fn main () {
    // call the function 
    let result = read_username_from_file();
    match result {
        Ok(s) => println!("Username: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}