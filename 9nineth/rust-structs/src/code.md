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

-----------------------------------------

struct User {
    name: String, 
    email: String,
    is_active: bool,
    age:u8
}

fn main(){
    let user1 = build_user(String::from("John Doe"), String::from("doe@mail.com"));
    //print all the values:
    println!("User name: {}, email: {}, is_active: {}, age: {}", user1.name, user1.email, user1.is_active, user1.age);

}

fn build_user(name: String, email:String) -> User{
    User {
        name,
        email,
        is_active: true,
        age: 21
    }
}

-----------------------------------

struct User {
    name: String, 
    email: String,
    is_active: bool,
    age:u8
}

fn main(){
    let user1 = User {
        name: String::from("Wuda"),
        email: String::from("doe@mail"),
        is_active: false,
        age: 20
    };

    // print all the values of user1
    
    let user2 = User {
        name: String::from("Francesco"),
        email: user1.email.clone(),
        is_active: user1.is_active,
        age: user1.age
    };
    
    // print all the values of user2
    println!("Name: {}, Email: {}, Is Active: {}, Age: {}", user2.name, user2.email, user2.is_active, user2.age);

    println!("Name: {}, Email: {}, Is Active: {}, Age: {}", user1.name, user1.email, user1.is_active, user1.age);


}


--------------------

// Tuple Structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);
}