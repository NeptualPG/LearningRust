// Exercise: get the last number of the first word of a String (with slice);
fn main (){
    //String
    let mut s = String::from("hello wuda");
    let word = first_word(&s);
    println!("The s is = {}", s);
    println!("The first word is = {}", word);


    // String literals are slices
    let s2 = "Second world"; 
    let word2 = first_word(&s2);
    println!("The s2 is = {}", s2);
    println!("The first word is = {}", word2);

}


// Function to get the first word of a string
fn first_word(s: &str) -> &str {
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

