
#[derive(Debug)]
struct QuitMessage;

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String);

#[derive(Debug)]
struct ChangeColorMessage(i32,i32,i32);

impl QuitMessage {
    fn call(self) {
        println!("self is {:?}", self);
    }
}

impl MoveMessage {
    fn call(self) {
        println!("self is {:?}", self);
    }
}

impl WriteMessage {
    fn call(self){
        println!("self is {:?}", self);
    }
}

impl ChangeColorMessage {
    fn call(self){
        println!("self is {:?}", self);
    }
}

fn main(){
    let q =  QuitMessage;
    let m = MoveMessage{x: 10, y: 20};
    let w = WriteMessage(String::from("Hello"));
    let c = ChangeColorMessage(255, 0, 0);


    // call the methods:
    q.call();
    m.call();
    w.call();
    c.call();
    
}