/* Enviroment Variables They are a set of key-value
pairs that are stored in the operating system
and are used to store configuration settings
and other information that is used by the system and
other applications.
 */

use std::env; // import the env module from the standard library

fn main() {
    let key = "BB";
    std::env::set_var(key, "Hello World"); // set the environment variable BB to Hello World

    match env::var(key) { // match the value of the environment variable BB
        Ok(val) => println!("{}: {:?}", key, val), // if the variable exists, print its value
        Err(e) => println!("couldn't interpret {}: {}", key, e), // if the variable doesn't exist, print an error message
    }
}
