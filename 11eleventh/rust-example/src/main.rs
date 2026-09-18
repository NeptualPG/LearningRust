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
}


// We can have multiple implementations to split methods

impl Rectangle {
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

