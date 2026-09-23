initial code :

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


------------------------------------
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

------------------------------------
// using super to access parant module

// ../ to go up one module

fn deliver_order(){}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        // if you want to access the parent private module, you can use super 
        super::deliver_order();
    }

    fn cook_order(){}
}