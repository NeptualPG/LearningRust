//Cath - all Pattern and _


fn main() {
    let dice_roll = 9;
    
    // usefull to handle errors, and to handle different cases of a value
    match dice_roll {
        1 => println!("You rolled a one!"),
        2 => println!("You rolled a two!"),
        3 => println!("You rolled a three!"),
        4 => println!("You rolled a four!"),
        5 => println!("You rolled a five!"),
        6 => println!("You rolled a six!"),
        7 => (), // if you want do nothing, you can use an empty tuple
        _ => println!("Invalid roll!"), // catch all pattern, if the value is not matched, it will be caught here 
    }
}