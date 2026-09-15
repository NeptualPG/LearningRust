fn main( ){
    let a = [1, 2, 3, 4, 5];

    // iter() IS A FUCNTION TO RETURN AN ITERATOR OVER THE ARRAY 

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}