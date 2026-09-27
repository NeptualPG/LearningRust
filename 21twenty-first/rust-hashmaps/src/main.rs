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
}
