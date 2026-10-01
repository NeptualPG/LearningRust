// A trait defines  functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.

// They are similar to Interfaces!

// Multiple methods can be defined in a trait.

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

// Traits as parameters 
// pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize());
// }


// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }

// instead, we can use the following syntax
// pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
//     // just item1:
//     println!("Breaking news! {}", item1.summarize());
//     // and item2:
//     println!("Breaking news! {}", item2.summarize());
// }

use std::fmt::Display;

// define multiple trait bounds with the + syntax
pub fn notify(item: &(impl Summary + Display)) {
   println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T:Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("This is a test tweet."),
        reply: false,
        retweet: false,
    };

    // println!("{}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("The sky is blue"),
        location: String::from("New York"),
        author: String::from("Jane Smith"),
        content: String::from("This is another test article."),
    };

    // println!("{}", article.summarize());
    // println!("{}", article.summarize_author());

    notify(&tweet);
}