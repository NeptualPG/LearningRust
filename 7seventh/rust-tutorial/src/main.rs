//Return Value and Scope
fn main(){
    let s1 = String::from("hellow");

    // the & symbol is used to pass a reference to the string instead of passing the string itself. 
    // This is more efficient because it avoids copying the entire string.
    let len = calculate_lenght(&s1); // passing reference to the string
    print!("The lenght of '{}' is {}.", s1, len)
}

//calculate lengh of the string
                  // What are you sending?
fn calculate_lenght(s: &String) -> usize // result type of the function is usize
{
    let lenght = s.len();
    //Return
    lenght
}


