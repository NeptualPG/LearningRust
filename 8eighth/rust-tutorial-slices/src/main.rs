// Exercise: get the last number of the first word of a String (with slice);
fn main (){
    let mut s = String::from("hello wuda");
    let word = first_word(&s);
    println!("The s is = {}", s);
    println!("The first word is = {}", word);

    s.clear(); // Clear the String 
    println!("The s is = {}", s);
    println!("The first word is = {}",word);
}


// Function to get the first word of a string
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // Iterate throught the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { // If the byte is a space (are you sick?)
            return &s[..i];
        }
    }
    // If no space is found, return the length of the string
    &s[..]
}


