We already saw generics!

Option<T>
* Used to handle nullable values safely 
* Example: let some_number:Opiton<i32> = Some(5);
  
Vec<T>
* A resizable array type.
* Example: let numbers: Vec<i32> = vec![1,2,3];

HashMap<K,V>
* a key-value store for efficient data retrieval.
* Example: let mut score: HashMap<String, i32> = HashMap::new();

Result<T,E>
* Used for error handling
* Example: let result: Result<i32,String> = Ok(10);