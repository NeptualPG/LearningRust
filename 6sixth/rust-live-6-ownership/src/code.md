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


fn main() {
    // literal string is stone you can't change that
    // let s1 = "hello";
    
    let s1 = String::from("hello"); 
    let s2 = s1.clone(); // assign s1 to s2

    //print s2
    println!("s2 = {}", s2);

    //print s1 
    println!("s1 = {}", s1);
}