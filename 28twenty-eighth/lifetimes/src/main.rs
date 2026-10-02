fn main(){
    let string1 = String::from("Hello");
    let string2 = "World";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

}


// if I am using anotation into the return type, I have to use the same lifetime annotation in the parameters of the function. This is because the return type is a reference that is tied to the lifetimes of the input parameters. By using the same lifetime annotation, we ensure that the returned reference is valid for as long as the input references are valid.
fn longest <'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("longest string");
    result.as_str()
}