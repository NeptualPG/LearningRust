use create::garden::vegetables::Asparagus;

pub mod garden;

fn main (){
    let plant: Aspargus = Asparagus {
        name: String::from("Asparagus"),
        stalks: 5,
    };

    println!("This plant is a {:?}", plant);
}