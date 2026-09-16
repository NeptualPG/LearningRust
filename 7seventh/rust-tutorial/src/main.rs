//Return Value and Scope
fn main(){
    let s1 = String::from("hellow");

    let (s2, len) = calculate_lengh(s1);
    print("The lenght of '{}' is {}.", s2, len)
}

//calculate lengh of the string
fn calculate_lenght(s: String) -> (String, usize){
    let lenght = s.len();
    (s, lenght)
}

