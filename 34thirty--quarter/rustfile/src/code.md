```rust

use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open the file
    let mut file = File::open("full.txt")?;

    // initialize a string 
    let mut content = String::new();

    // read the file content
    file.read_to_string(&mut content)?;

    // print the content 
    println!("File content:\n{}", content);

    Ok(())
}


```

-----------------


