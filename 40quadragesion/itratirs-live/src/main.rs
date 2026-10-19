/* 
Iterators in Rust provide a powerful and flexible way to process data efficiently by transforming, filtering 
and aggregating elements in a collection.

However, unlike traditional loops, Rust's iterators are lazy, meaning they don't do any work until explicitly instructed to 
This laziness makes iterators incredibly efficient, as they only evaluate elements when needed,
often combining multiple transformations into a single pass over the data.

In the lesson, we'll dive into the core of Rust's iterator system,
exploring how to use mehods like .map(), filter(), and .fold() to create expressive, functional code.

*/

fn main() {
    // Iterators - example 1
    let v1  = vec![1,2,3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Iterator demostration 
    let v1 = vec![10,20,30];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&10));
    assert_eq!(v1_iter.next(), Some(&20));
    assert_eq!(v1_iter.next(), Some(&30));
    assert_eq!(v1_iter.next(), None);

    // Types of iterators

    /*1 .iter() - 
      purpose: creates an iterator that borrows each element of the collection, 
      allowing you to read the values without taking ownership of them.
      Ownership: The iterator yields references (&T), no owned values: the original collection 
      is accessible and unaltered after iteration.
      use case:use iter() when you want to read or inspect elements without taking ownership
      or modifying the collection.
     */

    let numbers = vec![1, 2, 3];

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
    
    println!(".iter()");
    println!("Original numbers vector: {:?}", numbers); // The original vector is still accessible



    /*2 .into_iter() - 
      purpose: creates an iterator that takes ownership of the collection, 
      consuming it and yielding owned values.
      Ownership: The iterator yields owned values (T), and the original collection is no longer accessible after iteration.
      use case: use into_iter() when you want to consume the collection, transferring ownership of its elements.
     */

    let mut numbers = vec![1, 2, 3];

    for num in numbers.iter_mut() {
        *num += 1; // Modify the elements in place
        println!("Modified Number: {}", num);
    }

    println!(".iter_mut()");
    println!("Modified numbers vector: {:?}", numbers); // The original vector is still accessible and modified

}

pub trait iterator{
    type Item; // assiciated type - Item
    fn next(&mut self) -> Option<Self::Item>;

}