//structs in rust: used to create custom data types
//similar to tuples, but with named fields
// used to create more complex data types 
// structs are immutable by default
// struct with name, email, is_active, age

struct User {
    name: String, 
    email: String,
    is_active: bool,
    age:u8
}

fn main(){
    // every instances are mutable or a can just select them into the structure
    let mut user1 = User {
        name: String::from("John Doe"),
        email: String::from("doe@mail.com"),
        is_active: true,
        age: 25
    };

    user1.name = String::from("Francesco");

    println!("User 1 name: {}", user1.name);
}
