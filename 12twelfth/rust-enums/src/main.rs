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

#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
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