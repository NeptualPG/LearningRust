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

