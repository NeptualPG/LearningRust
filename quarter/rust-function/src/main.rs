//1st: Hello World
//2nd: Function
//3rd: Data Types
//4th: Function}

// fn another_function(num: i32, letter: char) {
//     println!("The value of num is {}", num);
//     println!("The value of letter is {}", letter);
// }

// fn main() {
//     println!("Hello, World!");
//     another_function(42, 'a');
// }


// fn main() {
//     let x = 5;
//     let y = 6; 
//     let z = x + y;
    
//     println!("The value of z is: {}", z);
// }




fn sum(num1: i32, num2: i32) -> (i32, i32) {
    (num1 + num2, num1 - num2)
}

fn main() {
    let x = sum(5, 7);
    println!("sum = {}, diff = {}", x.0, x.1);
    println!("The sum and diff is (:?)", x);
    // (:?) is used to print the tuple in a debug format
}