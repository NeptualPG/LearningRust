

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

fn main(){
    let coin: Coin = Coin::Quarter(Rarity::Epic);

    if let Coin::Quarter(rarity) = coin {
        println!("This quarter is a {:?}", rarity);
    }else{
        println!("This coin is not rare, it is a {:?}", coin);
    }
}