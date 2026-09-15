fn main(){
     enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
     }

     // we can use the enum to create a varaible

     fn value_in_cents(coin: Coin) -> u8{ //u8 is an unsigned 8-bit integer
         match coin {
             Coin::Penny => 1,
             Coin::Nickel => 5,
             Coin::Dime => 10, // you can't delete the Dime here because that is part of the enum, if you delete it, the code will not compile for pattern matching
             Coin::Quarter => 25,
         }
     }

     let conin = Coin::Penny;

     println!("The value of the coin is: {}", value_in_cents(conin));

}