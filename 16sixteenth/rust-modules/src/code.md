// Public and private control where a function can be called from.

// Visibility:	                Who can call it?
// Public	                    Code outside the class or module can call it.
// Private	                    Only code within its permitted scope can call it. The exact rules depend on the language.


// Declaring a public module
pub mod network {
    //Private function within the public module, used to establish a connection
    fn connect(){
        println!("Connection established");
    }

    // Public function that initiates a network connection 
    pub fn initiate_connection() {
        connect(); // Calls the private function 
        println!("Initiate connection...");
    }
}


fn main() {
    // Initiating a network connection - this is possible because the function 
    network::initiate_connection(); // Calls the public function

    // The following line will cause a compile error if a uncommented because '' 
    // eror: function `connect` is private-
    // network::connect(); 
}
