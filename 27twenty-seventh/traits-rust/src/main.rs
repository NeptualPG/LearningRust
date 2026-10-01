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