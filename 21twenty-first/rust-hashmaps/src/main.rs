/* Hash Maps in Rust 
The type HashMap<K, V> stores a mapping of 
keys of type k to value of V 

It does this via a hashing function, which determines how it places
these keys and values into memory 

this allows for very fast lookups of values by key.

Hash maps are useful when you want to look up values by a particular key.
*/

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    println!("scores: {:?}", scores); // scores: {"Blue": 10}

    let team_name = String::from("Blue"); 
    let score = scores.get(&team_name); // get() method returns an Option<&V> type


    let no_team_name = String::from("Yellow");
    let no_score = scores.get(&no_team_name); // get() method returns an Option

    println!("score: {:?}", score); // score: Some(10)
    println!("no_score: {:?}", no_score); // no_score: None

    // how to iterate over a HashMap

    let mut languages = HashMap::new();

    languages.insert(String::from("Rust"), 1);
    languages.insert(String::from("Python"), 2);
    languages.insert(String::from("Java"), 3);
    languages.insert(String::from("C++"), 4);
    languages.insert(String::from("JavaScript"), 5);

    for (key, value) in languages {
        println!("{}: {}", key, value);
    }

    // Ownership:

    let mut map2 = HashMap::new();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map2.insert(&field_name, &field_value);

    println!("{:?}", map2); // {}
    // I got an error here because I tried to use field_name after it was moved into the map2 HashMap.
    println!("field_name: {}", field_name); // field_name: Favorite color

    let mut map3 = HashMap::new();

    let number = 10;
    let text = String::from("Hello, World!");
    map3.insert(&text, number); // implemnt the copy trait for the number variable, so it can be copied into the map3 HashMap.

    println!("{:?}", map3); // {"Hello, World!": 10}

    println!("text: {}", text); // text: Hello, World!
    println!("number: {}", number); // number: 10

    // Updating a HashMap
}
