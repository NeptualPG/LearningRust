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

fn main(){
    let rect1 = Rectangle {
        width: 31,
        height: 50,
    }; 
    let rect2 = Rectangle {
        width: 30,
        height: 40,
    }; 
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    }; 
    // print the area

    // println!("The height of the rectangle is {}", rect1.height());

    println!("Can rect1 hold react2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold react3? {}", rect1.can_hold(&rect3));

}

