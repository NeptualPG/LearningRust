// Structure:
#[derive(Debug)]
struct Rectangle {
    widht: u32,
    height:u32,
}

fn main() {
    let rect1 = Rectangle {
        widht: 30,
        height: 50,
    };

    // Print the rectangle
    println!("{:?}", rect1);

    println!("The area of the rectangle is {} square pixels.",area(rect1));
}

fn area(rectangle: Rectangle) -> u32{
    rectangle.widht * rectangle.height
}