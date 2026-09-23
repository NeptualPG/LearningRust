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