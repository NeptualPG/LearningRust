use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();


    let config  = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    // let config = Config::build(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}


// fn parse_config(args: &[String]) -> Config {
//     let config = Config {
//         query: args[1].clone(),
//         file_path: args[2].clone(),
//     };
    
//     config
// }
