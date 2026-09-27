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

    let mut map4 = HashMap::new();

    map4.insert(String::from("Blue"), 10);
    map4.insert(String::from("Yellow"), 50); 
    // same key with a different value will overwrite the previous value for that key
    map4.insert(String::from("Blue"), 25); // this will overwrite the previous value of 10 for the key "Blue" 

    println!("{:?}", map4); // {"Blue": 25, "Yellow": 50}

    // Only inserting a value if the key has no value
    let mut map5 = HashMap::new();

    map5.insert(String::from("Blue"), 1);
    map5.entry(String::from("Yellow")).or_insert(50); // this will insert the key "Yellow" with the value 50 because it does not exist in the map5 HashMap.
    map5.entry(String::from("Blue")).or_insert(25); // this will not insert the key "Blue" with the value 25 because it already exists in the map5 HashMap.

    println!("{:?}", map5); // {"Blue": 1, "Yellow": 50}

    // Updting a value based on the old value
    let text = "Rust is a greate programming language. I love Rust!";
    let mut map6 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map6.entry(word).or_insert(0); // this will insert the key "word" with the value 0 if it does not exist in the map6 HashMap.
        *count += 1; // this will increment the value of the key "word" by 1.
    }

    println!("{:?}", map6); // {"Rust": 2, "is": 1, "a": 1, "greate": 1, "programming": 1, "language.": 1, "I": 1, "love": 1}
}
