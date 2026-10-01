// Both need to be the same type, so this will not compile. 
// The generic struct Point<T> allows us to create a Point 
// with different types for x and y coordinates, but in 
// this case, we are trying to create a Point with an integer and a floating-point number, which is not allowed.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// To allow for different types for x and y coordinates, we can define a generic struct Point<T, U> that takes two type parameters T and U. This way, we can create a Point with different types for x and y coordinates without causing a compilation error.
struct Point<T, U> {
    x: T,
    y: U,
}

//get x and y values
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// distance from origin
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// mixer method
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Generic struct Point<T> allows us to create a Point with different types for x and y coordinates. In this example, we create three instances of Point: one with integer coordinates, one with floating-point coordinates, and one with character coordinates. The main function demonstrates how to create these instances and print their values.
fn main(){
    let integer = Point {x : 5, y: 10};
    let float = Point { x: 1.0, y: 4.2};
    let char = Point { x: 'a', y: 'b'};

    println!("Integer Point: ({}, {})", integer.x, integer.y);
    println!("Float Point: ({}, {})", float.x, float.y);
    println!("Char Point: ({}, {})", char.x, char.y);

    let integer_and_float = Point { x: 5, y: 4.2}; // This line will cause a compilation error because the types of x and y are different.
    println!("Integer and Float Point: ({}, {})", integer_and_float.x, integer_and_float.y);

    let integer_char: Point<i32, char> = Point { x: 5, y: 'a' }; // This line will compile successfully because we are using different types for x and y.
    println!("Integer and Char Point: ({}, {})", integer_char.x, integer_char.y);

    // read the x calue of the integer_and_float point
    println!("The x value of the integer_and_float point is: {}", integer_and_float.x());

    // read the y value of the integer_and_float point
    println!("The y value of the integer_and_float point is: {}", integer_and_float.y());

    //Distance from origin for float point
    println!("Distance from origin for float point: {}", float.distance_from_origin());

    let p1_mix: Point<i32, i32> = Point { x: 2, y: 10 };
    let p2_mix: Point<f64, f64> = Point { x: 1.0, y: 2.0 };
    let p3_mix = p1_mix.mixup(p2_mix);
    println!("Mixed Point: ({}, {})", p3_mix.x, p3_mix.y);

}

