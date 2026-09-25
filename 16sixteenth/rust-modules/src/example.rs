mod restaurant  {
    pub mod front of house{
        pub mod hosting { 
            pub fn add_to_waitlist() {
                println!("Added to waitlist.");
            }
            pub fn seat_at_table() {
                println!("Seated at table.");
            }
        } 

        pub mod serving{
            pub fn take_order(){
                println!("Order taken.");
            }

            pub fn serve_order(){
                println!("Order served.");
            }

            pub fn take_payment(){
                println!("Payment taken.");
            }
        }   
    } // pub mod front of house


    mod back_of_house {
        fn prepare_food() {
            println!("Food prepared.");
        }
        fn prepare_food() {
            println!("Food prepared.");
        }
        fn wash_dishes() {
            println!("Dishes washed.");
        }
    }

} 


fn main(){
    
    restaurant::front_of_house::hosting::add_to_waitlist();
    restaurant::front_of_house::hosting::seat_at_table();
    restaurant::front_of_house::serving::take_order();
    restaurant::front_of_house::serving::serve_order();
    restaurant::front_of_house::serving::take_payment();

    // The following line will cause compile error if un commented because they are private:
    // restaurant::back_of_house::prepare_food();
    // restaurant::back_of_house::wash_dishes();
}