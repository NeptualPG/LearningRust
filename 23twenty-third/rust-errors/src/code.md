
use::std::fs::File;
use::std::io::ErrorKind; // This has diferent types of errors that can occur when working with files

fn main(){
    let greeting_from_file = File::open("hello.txt");
    let greenting_file = match greeting_from_file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}


// ALTERNATIVE WAY TO HANDLE ERRORS

// Use unwrap_or_else to handle errors in a more concise way

fn main() {
    let greeting_file = File::Open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error)
            })
        }else {
            panic!("Problem opening the file: {:?}", error)
        }
    })
}


-----------------------




/* 
Error Handling in Rust

- Most programming languages have one way to handle errors (exceptions)
- Rust does not have exceptions 
- Rust differentiates between unrecoverable errors and recoverable errors
- Unrecoverable errors are handle by the panic! macro
- Recoverable errors are handled by the result<T, E> enum

*/

use::std::fs::File;
use::std::io::ErrorKind; // This has diferent types of errors that can occur when working with files

// unwrap and expect


fn main(){
    // let greeting_from_file: File = File::open("hello.txt").unwrap();
    let greeting_from_file: File = File::open("hello.txt").expect("hello.txt should be included in this project");
    // unwrap is a shortcut method that retuns the value if Ok or panics if Err.  
    // expect is similar to unwrap but allows you to specify a custom panic message.
}

-----------------------


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