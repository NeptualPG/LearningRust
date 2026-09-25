
// lib
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
            // Function implementation goes here
    } 
}

// This use is just in this scope, so it is not available outside of this module.
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant(){
    // hosting::add_to_waitlist();
    add_to_waitlist();
}

