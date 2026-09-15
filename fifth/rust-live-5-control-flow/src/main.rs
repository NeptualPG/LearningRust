fn main(){
    let mut counter  = 3;

    while counter !=0  {
        println!("{}!", counter);
        counter -= 1;
        
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println! ("LIFTOFF!!!");
}