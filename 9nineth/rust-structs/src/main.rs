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
