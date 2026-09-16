fn main(){
    let mut s = String::from("Hello");
    {
        let s1 = &mut s;
        s1.push_str(", world")
    }
    let s2 = &mut s;
    s2.push_str(";");
    
    println!{"s2 : {}", s2}
}
