fn main(){
    let mut a = 5;
    let mut 10;

    println!("Before swap: a ={}, b = {}", a, b);
    // Swapping
    {
        let temp = a;
        a = b;
        b = temp;
    }
    println!("After swap: a = {}, b = {}", a, b);

    let mut x = "hello";
    let mut y = "world";

    println!("Before swap: x = {}, y = {}", x, y);
    // Swapping strings
    {
        let temp = x;
        x = y;
        y = temp;
    }

}


// Avoid code duplication: 

// We have two blocks of code that swap the values of two variables.
// The logic is duplicated, making the code harder to maintain.