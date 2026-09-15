fn main (){
    let a = 10;
    let b = 5; 
    let c = 20;

    //Using && (AND) to check if 'a' is > than 'b' AND 'a' is < than 'c'

    if a > b && a < c {
        println!("a is greater than b AND a is less than c");
    } else {
        println!("a is NOT greater than b AND a is NOT less than c");
    }

    //Using || (OR) to check if 'a' is > than 'b' OR 'a' is < than 'c'

    if a > b || a < c {
        println!("At least in codition with || is met");
    } else {
        println!("The codition with || is not met");
    }
    
}