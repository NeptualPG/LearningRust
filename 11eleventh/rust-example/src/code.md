/*
Methods in rust are similar to functions, but they are 
defined within the context of a struct (or an enum or a trait).they are called by intacnces of the
struct, and their first parameter is always self, which represent the instance
of the struct, the following exaple demostrates how to define a method
for the rectabgle struct that calculates the area of a rectangle.
*/


struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the rectangle
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}