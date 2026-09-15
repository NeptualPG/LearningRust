fn main( ){
    let a = [1, 2, 3, 4, 5];

    // iter() IS A FUCNTION TO RETURN AN ITERATOR OVER THE ARRAY 
    // the element is auto assigned as let and mut to the type of the array, in this case i32, that is crazy
    for element in a.iter() {
        println!("the value is: {}", element);   
    };

    let s = "hellouda";

    // chars() String function returns an iterator over the characters of the string

    for c in s.chars() {
        println!("the value is: {}", c);
    };


    for number in 1..4{
        println!("the value is: {}", number);
    };


}