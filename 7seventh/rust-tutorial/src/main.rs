fn main (){

    // But that isn't existing at the moment when the code is here:
    let ref_value = no_dangle();
    println!("{}", ref_value)
}

fn no_dangle() -> String {
    let s = String::from("helllo");
    // return the reference 
    s
} // drop s