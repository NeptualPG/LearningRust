// GENERICS 
// THEY ALLOW YOU TO WRITE FLEXIBLE AND REUSABLE FUNCTIONS AND TYPES THAT CAN
// OPERATE ON DIFFERENT TYPES WITHOUT SACRIFICING PERFORMANCE

fn swap<T>(x: &mut T, y: &mut T) {
    let temp = std::mem::replace(x, std::mem::replace(y, std::mem::replace(x, temp)));
}

fn main(){
    let mut a = 5;
    let mut b = 10;

    println!("Before swap: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap: a = {}, b = {}", a, b);

    let mut x = "hello";
    let mut y = "world";

    let mut x = "hello";
    let mut y = "world";
    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);
}

// T is for type parameter, it can be any type, and it will be determined when the function is called.