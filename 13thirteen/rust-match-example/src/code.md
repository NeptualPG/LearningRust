// Match: first example 
// this works to avoid the warning of unused code so that 
// we can focus on the match example but whent isn't used it will give a warning telling us that it is unused code and 
// unused code is when you have a variable or function that is not being used in the code.
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}


#[derive(Debug)]
#[allow(dead_code)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(rarity) => {
            println!("Quarter is a {:?} coin!", rarity);
            25
        },
    }
}

fn main (){
    let coin = Coin::Quarter(Rarity::Rare);
    println!("The value into the coin is: {}", value_in_cents(coin));
}


--------------------------------------

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