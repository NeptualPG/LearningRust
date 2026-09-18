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



-------------------------------------


struct Rectangle {
    width: u32,
    height: u32,
}

//imp block is used to define methods
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    //method with the same name of a field
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 0,
        height: 50,
    }; 
    // print the area
    if rect1.width() {
        println!("The areaa of rectangle is {} square pixels.", rect1.area());
    } else {
        println!("ERROR The width of the rectangle is INVALID.");
    }
}

--------------------------


struct Rectangle {
    width: u32,
    height: u32,
}

//imp block is used to define methods
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    //method with the same name of a field
    fn width(&self) -> bool {
        self.width > 0
    }
    
    //getter for height
    fn height(&self) -> bool {
        self.height > 0
    }

    // check if a rectangle ccan hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }

    //associated function to define a square 
    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size,
        }
    }  
}

fn main(){
    let square = Rectangle::square(10);

    //calculate the area of the square
    println!("The area of the square is: {}", square.area());
}

---------------------