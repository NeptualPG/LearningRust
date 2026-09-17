/*
Slice in Rust are a reference to a contiguous
sequence of elements in a collection.
They are a view into the original collection 
and do not store any data themselves.
Slices are used to give a part of a collection to a function
or to iterate over a part of a collection.
 */

 //Slice syntac: &[T]
 //T is the type of  the elements in the collection
 //& is a reference to the collection.
fn main() {
    //first example: slice of an array of characters

    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];

    println!("{:?}", slice);

    //second example: slice of a vector of integers   
    //vectors are resezable arrays
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let slice: &[i32] = &vec[3..4];
    println!("{:?}", slice);

    let s: String = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("{:?}", hello);
    println!("{:?}", world);
}

----------------------------------

fn main(){
    let s= String::from("Francesco");

    // Same output

    let slice = &s[0..3];
    println!("{}", slice);

    let slice = &s[..3];
    println!("{}", slice);

    // Same output

    let len = s.len();
    let slice = &s[4..len];
    println!("{}", slice);

    let slice = &s[4..];
    println!("{}", slice);


    // Shortcut for both initial and final index
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}


---------------------------------


//Exercise: get the first word of a string();
fn main(){
    let s = String::from("anyword and more");
    get_word(s);
}

fn get_word(s: String) {
    let len = s.len();
    let mut cha: &str = "";
    let mut i = 1;
    
    while i <= len{
        cha = &s[i-1..i];
        if cha == " "{
            let shot: &str = &s[..i];
            println!("{}",shot);
            i = len;
        }else{
            i += 1;
        }
    }
}

-----------------------------


    //Exercise: get the first word of a string(slices);
fn main(){
    let mut s = String::from("hellowda man");
    let word = &s[..first_word(&s)];

    println!("The s is = {}", s);
    println!("The first word is = {}", word);

    s.clear(); // This empties the string, makin it equal to ""
    println!("The s is = {}", s);
    println!("The first word is = {}", word);
}
--------------------------------------------------------

//Exercise: get the first word of a string(slices);
fn main(){
    let mut s = String::from("hellowda man");
    let word = &s[..first_word(&s)];

    println!("The s is = {}", s);
    println!("The first word is = {}", word);

    s.clear(); // This empties the string, makin it equal to ""
    println!("The s is = {}", s);
    println!("The first word is = {}", word);
}

// Function to get the first word of a string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // Iterate throught the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { // If the byte is a space (are you sick?)
            return i; // Return the index 
        }
    }
    // If no space is found, return the length of the string
    s.len()
}

--------------------------------------------------------


// Exercise: get the last number of the first word of a String (with slice);
fn main (){
    let s = String::from("hellowuda");
    let word = first_word(&s);
    println!("The s is = {}", s);
    println!("The first word is = {}", word)
}


// Function to get the first word of a string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // Iterate throught the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { // If the byte is a space (are you sick?)
            return i; // Return the index 
        }
    }
    // If no space is found, return the length of the string
    s.len()
}


----------------------------------------


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


