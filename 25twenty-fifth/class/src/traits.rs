// is like generical function but for types
trait Summary {
    fn summarize(&self) -> String;
}

struct NewArticle {
    headline: String,
    content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format !("{}: {}", self.headline, self.content)
    }
}


struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format !("@{}: {}", self.username, self.content)
    }
}

fn main() {
    Let article = NewArticle {
        headline: String::from("Breaking News"),
        content: String::from("Rust is awesome!"),
    };

    let tweet = Tweet {
        username: String::from("Diago"),
        content String::from("Rust is awesome!"),
    }

    println!("Article summary: {}" article.summarize();)
    println!("Tweet summary: {}" tweet.summarize();)
}