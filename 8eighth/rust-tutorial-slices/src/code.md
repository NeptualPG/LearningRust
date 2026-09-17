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