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
 
}