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