//main.rs
// use external packages

use rand; // for random number generation

fn main(){
    // old way of generating a random number
    // let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number between 1 and 100
    let secret_number = rand::random_range(1..=100);
    println!("The secret number is: {}", secret_number);
}
