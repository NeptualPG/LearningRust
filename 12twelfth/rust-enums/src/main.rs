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