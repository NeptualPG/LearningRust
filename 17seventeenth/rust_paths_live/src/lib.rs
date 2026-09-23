// Paths 

mod front_of_house {
    pub mod hosting {
        // pub to both
        pub fn add_to_waitlist(){}
    }
}

pub fn eat_at_resyaurant(){
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative path
    front_of_house::hosting::add_to_waitlist();
}