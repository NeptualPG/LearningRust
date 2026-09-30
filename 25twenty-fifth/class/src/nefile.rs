// Define a function using generics and the partialOrd
// trait to find the largest item in a slice
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // Initialize the largest item as the first item in the list
    let mut largest = &list[0];
    // Iterate through the list to find the largest element
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    // Return a reference to the largest item found
    largest
}


fn main(){
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}