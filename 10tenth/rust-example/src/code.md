// With Tuples:

fn main() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {}", area(rect1));
}


fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} 

------------------------------


// Structure:

struct Rectangle {
    widht: u32,
    height:u32,
}

fn main() {
    let rect1 = Rectangle {
        widht: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.",area(rect1));
}

fn area(rectangle: Rectangle) -> u32{
    rectangle.widht * rectangle.height
}