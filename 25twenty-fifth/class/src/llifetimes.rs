fn logest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main( {
    let string1 = String::from("long stringis longer");
    let string2 = "xyz";

    // as_str() converts a String to a string slice (&str)
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
})