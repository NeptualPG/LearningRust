// Enums in rust 

// Enums are a way to define a type bu enumerating its possible variants. 
//  Enums defined using the "enum" keyword.
// they can contain data as well

// first example with IP adress

enum IpAddrKind{
    V4,
    V6
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    

    print("IP address kind: {:?} and {:?}", four, six);
}


-----------------------------


// Enums in rust 

// Enums are a way to define a type bu enumerating its possible variants. 
//  Enums defined using the "enum" keyword.
// they can contain data as well

// first example with IP adress

#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind){
    // do something with the ip address
    println!("Routing IP address of kind: {:?}", ip_kind); 
}   


-----------------------------------


// Enums in rust 

// Enums are a way to define a type bu enumerating its possible variants. 
//  Enums defined using the "enum" keyword.
// they can contain data as well

// first example with IP adress

#[derive(Debug)]
enum IpAddrKind{
    V4(String),
    V6(String)
}


fn main() {

    //create instances of strct
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    }

    println!("{:?}", home);

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    }
    
    println!("{:?}", loopback);

}

-----------------------------------------


#[derive(Debug)]
enum IpAddrKind{
    V4(String),
    V6(String)
}


fn main() {

    
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback); 

}


---------------------------------

#[derive(Debug)]
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {

    // Create instances of the enum
    let home = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:?}", home);
    
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", loopback); 
}

-------------------------------

#[derive(Debug)]
struct QuitMessage;

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String);

#[derive(Debug)]
struct ChangeColorMessage(i32,i32,i32);

impl QuitMessage {
    fn call(self) {
        println!("self is {:?}", self);
    }
}

impl MoveMessage {
    fn call(self) {
        println!("self is {:?}", self);
    }
}

impl WriteMessage {
    fn call(self){
        println!("self is {:?}", self);
    }
}

impl ChangeColorMessage {
    fn call(self){
        println!("self is {:?}", self);
    }
}

fn main(){
    let q =  QuitMessage;
    let m = MoveMessage{x: 10, y: 20};
    let w = WriteMessage(String::from("Hello"));
    let c = ChangeColorMessage(255, 0, 0);


    // call the methods:
    q.call();
    m.call();
    w.call();
    c.call();
    
}

--------------------------


// implementation with enum

#[derive(Debug)]
enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// method for the generic enum Message

impl Message {
    fn call(&self) {
        println!("Message is: {:?}", self);
    }
}

fn main(){
    let m = Message::Write(String::from ("Hello, World!"));
    let x = Message::Move { x: 3, y: 4 };
    let y = Message::ChangeColor(0,0,0);
    let z = Message::Quit;

    // println!("{:?}", m);
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    m.call();
    x.call();
    y.call();
    z.call();
}

-----------------------------------------

//In rust there is no "Null" value, instead we have "Option" enum

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
}

