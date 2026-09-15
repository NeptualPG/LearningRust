// fn main() {
    // DATA TYPES:
    // Scalar Types:
    //  - Integers
    //  - Floating-point numbers
    //  - Booleans
    //  - Characters
    //  Compound Types
    //  - Tuples
    //  - Arrays
    //  Custom Types
    //  - Structs
    //  - Enum
// }

// fn main() {
//     //INTEGERS:
//     // Length Signed Unsigned 
//     // 8-bit   i8     u8
//     // 16-bit  i16     u16
//     // 32-bit  i32     u32
//     // 64-bit  i64     u64 
//     // 128-bit i128    u128
//     // arch    isize   usize

//     // let small_number: u8 = 250;
//     // let small_number: i8 = -12;
//     // let big_number: u128 = 20112312312323;
//     // let big_number: i128 = -20112312312323;

//     // Numeral System Description example
//     // DECIMAL BASE-10, common form 98_222
//     // Hexadecimal BASE-16, prefix with 0x, common form 0xff
//     // Octal BASE-8, prefix with 0o, common form 0o77
//     // Binary BASE-2, prefix with 0b, common form 0b1111_0000
//     // Byte (u8 only), prefix with b, common form b'A'
    
//     let decimal = 98_222;
//     let hex = 0xff;
//     let oct = 0o77;
//     let byte = b'A';
    
    
//     println!("decimal: {}", decimal);
//     println!("hex: {}", hex);
//     println!("octal: {}", oct);
//     println!("byte: {}", byte);
// }

//INTEGER = i
//FLOATING POINT = fn 
// n = bits

// fn main() {
//     //FLOATING POINT NUMBERS:
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32

//     println!("x: {}, y: {}", x, y);
//     println!("x =  {}, y = {}", x, y);

//     // NUMERIC OPERATORS:
//     let sum = 5 + 10; // addition
//     let difference = 95.5 - 4.3; // subtraction
//     let product = 4 * 30; // multiplication
//     let quotient = 56.7 / 32.2; // division
//     let floored = 2 / 3; // integer division, result is 0

//     println!("sum: {}\ndifference: {} \nproduct: {} \nquotient: {} \nfloored: {} \n", sum, difference, product, quotient, floored);

// }

// fn main() {
//     // BOOLEAN TYPE:
//     let t = true; 
//     let f = true; // explicit type annotation
    
//     if t {
//         print!("t is true")
//     } else {        
//         print!("t is false")
//     }
    
//     let not_t  = !t; // logical NOT operator 

//     println!("not_t: {}", not_t);
    
// }


// fn main() {
//     let b:bool = false;
//     println!("b: {}", b);
// }
// fn main() {
//   // CHARACTER TYPE:
//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';

//     println!("{}, {}, {}", c, z, heart_eyed_cat);

//     for char in ['a', 'b', 'c'].iter() {
//         println!("{}", char);
//     }
// }
// fn main() {
//     let tup: (i32, f64, char) = (500, 6.4, 'c');

//     let (x, y , z) = tup;

//     println!("x: {}, y: {}, z: {}", x, y, z);

//     // Accessing tuple elements by index
//     println!("First element: {}", tup.0);
//     println!("Second element: {}", tup.1);
//     println!("Third element: {}", tup.2);
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
    
//     let first = arr[0];
//     let second = arr[1];

//     println!("First element: {}", first);
//     println!("Second element: {}", second);

//     for elemnt in arr.iter() {
//         println!("element {}", elemnt);
//     }
// }

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    // CUSTOM DATA TYPES
    // STRUCTS
    let mut user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        sign_in_count: 1,
        active: true,
    };
    println!("User1: {}, {}, {}, {}", user1.email, user1.username, user1.sign_in_count, user1.active);

    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }
}
