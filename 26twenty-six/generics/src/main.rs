/* Generic Data Types in Rust

In Rust, we can define generic data types for: 

- Functions
- Structs
- Enums
- Methods

Generic data types are useful when we want to write code that can work with different types without duplicating code for each type.
 */

 // Generic function

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest: &i32 = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    *largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest: &char = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    *largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    
}
