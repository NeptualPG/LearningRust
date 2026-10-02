// Trats in rust 

// A trait defines functionallity a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way

// They are similar to Interfaces!

// Define a Trait

// pub trait Summary {
//     fn summarize(&self) -> String; // funcrtion declaration
// }


pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


// Implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // article with the default implementation of the summarize_author method
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("John Doe"),
        content: String::from("The Pittsburgh Penguins have won the Stanley Cup Championship!"),
    };

    println!("1 New article available!: {}", article.summarize());
    
}

------------------------------

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

fn main() {
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("This is a test tweet."),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("The sky is blue"),
        location: String::from("New York"),
        author: String::from("Jane Smith"),
        content: String::from("This is another test article."),
    };

    println!("{}", article.summarize());
    println!("{}", article.summarize_author());
}

--------------------------------


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





// Traits as Parameters
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// now with impl two items that implement the Summary trait

// pub fn notify(item1: &impl Summary + , item2: &impl Summary) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }

pub fn notify<T:Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
//     println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
// }

use std::fmt::Display;
use std::fmt::Debug;


//define multiple trait bounds with the + syntax
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T, U>(item1: &T, item2: &U)    


// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     0
// }

// Cleaner syntax for multiple trait bounds with where clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Debug {
    0
}


fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Debug {
    0
}

//Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user123"),
        content: String::from("This is a test tweet."),
        reply: false,
        retweet: false,
    }
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