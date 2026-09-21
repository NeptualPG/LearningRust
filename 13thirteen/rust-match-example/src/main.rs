// Match with Option<T>

// enum Option<T> {
//     Some(T),
//     None,
// }



fn main () {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // the match needs express all the possible cases of the Option<T> enum 
        // GOSSSHHHH THIS IS SO COOL IS LIKE A SWITCH STATEMENT BUT BETTER WITH CORRECTION AND SAFETY BUILT IN.
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);

    let none = plus_one(None);
    println!("{:?}", none);

}