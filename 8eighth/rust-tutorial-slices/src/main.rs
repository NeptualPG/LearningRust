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