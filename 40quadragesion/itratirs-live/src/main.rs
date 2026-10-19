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

    /*1 .iter() - creates an iterator that borrows each element of the collection, allowing you to read the values without taking ownership.

     */

}

pub trait iterator{
    type Item; // assiciated type - Item
    fn next(&mut self) -> Option<Self::Item>;

}