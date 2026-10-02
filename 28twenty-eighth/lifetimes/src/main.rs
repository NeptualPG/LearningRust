// Life annotations in Method Definitions

struct ImporTantExcerpt<'a> {
    name: String,
}

impl <'a> ImporTantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl <'a> ImporTantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        &self.name
    }
}

fn main(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImporTantExcerpt {
        name: String::from(first_sentence),
    };
}