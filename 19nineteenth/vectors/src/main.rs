/*Collections are data structures that store multiple values

they are stored on the heap:
the data does not need to be known at compile time
and can grow or shrink as the program runs.

A vector allow you to store a variable number of values next to each other in memory.
 */

fn main() {
    //crate a vector
    let vec1: Vec<i32> = Vec::new();
    println!("vec1: {:?}", vec1);   

    let vec2 = vec![1,2,3,4,5];
    println!("vec2: {:?}", vec2);

    // push and pop and mut into a vector
    let mut vec3: Vec<i32> = Vec::new();

    vec3.push(1);
    vec3.push(2);
    vec3.push(3);

    // println!("vec3: {:?}", vec3);00000

    vec3.pop();
    // vec3 after pop: [1, 2]
    // println!("vec3 after pop: {:?}", vec3);

    let  vec4 = vec!['a', 'b', 'c', 'd', 'e'];
    let fourth: &char = &vec4[3];
    
    println!("The fourth element is: {}", fourth);
    // is d

    // using get method to access elements with optional
    let fifth: Option<&char> = vec4.get(4);

    match fifth {
        Some(&fifth) => println!("The fifth element is: {}", fifth),
        None => println!("There is no fifth element."),
    }

    let mut vec6 = vec![10, 20, 30, 40, 50];
    

    for i in &mut vec6 {
        // * before i to dereference the mutable reference and get the value it points to
        *i += 1;
    }

    println!("vec6 after incrementing each element: {:?}", vec6);
    

    // if you wanna store different types in a vector using enums
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // is the same type but contains different types of data
    // vec! is for creating a vector with different types of data using the enum SpreadsheetCell
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("row: {:?}", row);


} // all the vectors go out of scope and are freed when the function ends