// Ownership and functions
fn main() {
    let i = 5;
    call_int(i);
    println!("AFTER CALLING THE FUNCTION, the value of i: {}", i);

    let s = String::from("Hello");
    call_string(s);

    println!("AFTER CALLING THE FUNCTION, the value of s: {}", s);

}

// call int function
fn call_int(i:i32){
    println!("call_int i: {}", i)
}

// call string function
fn call_string(s: String){
    println!("call_string s: {}", s)
}

--------------------------------

fn main(){
    let s1 = give_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("Hello from main");

    let s3 = take_and_give_ownership(s2);
    println!("s3: {}", s3);
}


// function to give ownership of a string to another function
fn give_ownership() -> String {
    let some_string = String::from("Hello from give_ownership");
    // return:
    some_string
}

// function to take ownership of a string
fn take_and_give_ownership(some_string: String) -> String {
    // return:
    some_string
} 

-------------------------

//Return Value and Scope
fn main(){
    let s1 = String::from("hellow");

    let (s2, len) = calculate_lenght(s1);
    print!("The lenght of '{}' is {}.", s2, len)
}

//calculate lengh of the string
fn calculate_lenght(s: String) -> (String, usize){
    let lenght = s.len();
    //Return
    (s, lenght)
}



--------------------------------

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

---------------------


fn main() {
    let s = String::from("Hello");
    change_borrowed_value(&mut s);
}

fn change_borrowed_value(s: &mut String){
    s.push_str(", world!");
}


----error-------

fn main(){
    let mut s = String::from("Hello");

    let s1 = &mut s;
    let s2 = &mut s;

    println!("{}, {}", s1, s2);
}

---------------------- This is amazing

fn main(){
    let mut s = String::from("Hello");
    {
        let s1 = &mut s;
        s1.push_str(", world")
    }
    let s2 = &mut s;
    s2.push_str(";");
    
    println!{"s2 : {}", s2}
}


