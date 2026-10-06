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


```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    match read_file_to_string("full.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}


fn readt_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

```

---------------------
