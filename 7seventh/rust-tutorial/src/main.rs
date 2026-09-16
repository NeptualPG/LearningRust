fn main() {
    let s = String::from("Hello");
    change_borrowed_value(&mut s);
}

fn change_borrowed_value(s: &mut String){
    s.push_str(", world!");
}