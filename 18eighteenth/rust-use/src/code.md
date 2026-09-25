use std::collections::HashMap;


fn main(){
    // here we create a new HashMap with a function
    // the method to not confuse with the new() 
    // is to use the function HashMap::new()
    let mut map = HashMap::new();

    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map);
} 


--------------------------

