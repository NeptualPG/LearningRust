// Generics Enums

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main(){
    let some_number: Option<i32> = Option::Some(5);
    let no_number: OOption<i32> = Option::None;

    println!("{:?}", some_number);
    println!("{:?}", no_number);
    
    let success: Result<i32, String> = Result::Ok(5);
    let failure: Result<i32, String> = Result::Err(String::from("An error occurred"));

    match success {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(err) => println!("Error: {}", err),
    }

    match failure {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(err) => println!("Error: {}", err),
    }
}

