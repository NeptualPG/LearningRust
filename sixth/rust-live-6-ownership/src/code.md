fn main() {
    //s is not valid here, it's not yet declared
    let s = "Hello";// s is valid from this point forward

    // do ..... // s is still valid 

} // the scope is now over, and s is no longer valid


fn main(){

    // s is not valid 
    let mut s = String::from("hello");//s is valid


    // allocated on the heapp and not on the stack 
    s.push_str(", world!");

    println!("{}", s);
    
} //s is not valid anymore, we call drop