fn main() {
    // literal string is stone you can't change that
    // let s1 = "hello";
    
    let s1 = String::from("hello"); 
    let s2 = s1; // assign s1 to s2

    //print s2
    println!("s2 = {}", s2);

    //print s1 
    println!("s1 = {}", s1);
}