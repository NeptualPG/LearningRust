// implementation with enum

#[derive(Debug)]
enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// method for the generic enum Message

impl Message {
    fn call(&self) {
        println!("Message is: {:?}", self);
    }
}

fn main(){
    let m = Message::Write(String::from ("Hello, World!"));
    let x = Message::Move { x: 3, y: 4 };
    let y = Message::ChangeColor(0,0,0);
    let z = Message::Quit;

    // println!("{:?}", m);
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    m.call();
    x.call();
    y.call();
    z.call();
}

