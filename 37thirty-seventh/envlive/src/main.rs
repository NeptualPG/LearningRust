
use std::env; // import the env module from the standard library
use std::env::VarError;
use dotenv::dotenv; // import the dotenv crate to read .env files

fn main() {
    let key = "BB";
    unsafe {
       std::env::set_var(key, "Hello World");
       env::remove_var(key); // remove the environment variable BB
    } // set the environment variable BB to Hello World

    match env::var(key) { // match the value of the environment variable BB
        Ok(val) => println!("{}: {:?}", key, val), // if the variable exists, print its value
        Err(e) => println!("couldn't interpret {}: {}", key, e), // if the variable doesn't exist, print an error message
    }

    // example 2 read env variables from command line

    let cli_arg: Result<String, VarError> = env::var("CLI_ARG"); // read the environment variable CLI_ARG

    match cli_arg { // match the value of the environment variable CLI_ARG
        Ok(val) => println!("CLI_ARG: {:?}", val), 
        Err(e) => println!("couldn't interpret CLI_ARG: {}", e),
    }

    // example 3 read env variables from .env file

    dotenv().ok(); // load the .env file 

    let api_key: Result<String, VarError> = env::var("API_KEY"); // read the environment variable

    // match api_key { // match the value of the environment variable API_KEY
    //     Ok(val) => println!("API_KEY: {:?}", val), // if the variable exists, print its value into variable val
    //     Err(e) => println!("couldn't interpret API_KEY: {}", e), // if the variable doesn't exist, print an error message
    // }   
    println!("API_KEY: {:?}", api_key.unwrap_or("No API_KEY found".to_string())); 

    println!("Program completed successfully!");
}

/* Enviroment Variables They are a set of key-value
pairs that are stored in the operating system
and are used to store configuration settings
and other information that is used by the system and
other applications.
 */
