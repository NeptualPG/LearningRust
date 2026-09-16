// Mutable and immutable references 
// The problem is we have mutable reference (just one) and immutable too
// If we only use the immutable we can use several references
fn main() {
    let mut s = String::from("Helouda");

    let s1 = &s;
    let s2 = &s;

    println!("{}, {}", s1, s2);

    let s3 = &mut s;

    println!("{}", s3);


}