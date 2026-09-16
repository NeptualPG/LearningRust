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