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