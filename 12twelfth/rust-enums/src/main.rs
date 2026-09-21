// An example of Opiton enum
fn main() {
    let x: Option<i8> = Some(5);
    let y: Option<i8> = Some(5);

    // try to sum them
    // unwrap_or() is a method that returns the value inside the option<>
    // if it is Some(T)
    let sum = x.unwrap_or(0) + y.unwrap_or(0); 
    println!("The sum of x and y is: {}", sum);
} 