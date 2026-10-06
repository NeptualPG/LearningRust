use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()>{
    let file = File::open("full.txt")?;
    let reader = io::BufReader::new(file);

    // line by line reading
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    ok(())
}

