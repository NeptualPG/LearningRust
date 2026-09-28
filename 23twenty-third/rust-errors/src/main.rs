use::std::fs::File;
use::std::io::ErrorKind;
use::std::io::{self, Read}; 
use::std::error::Error;



// Just with a result in the fucntion
fn main () -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt");
    // print the value of the file
    let mut greeting = String::new();
    greeting_file?.read_to_string(&mut greeting)?;
    println!("Greeting: {}", greeting);
    Ok(())
}
 
