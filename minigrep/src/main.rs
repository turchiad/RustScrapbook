use std::env; // To allow access of CLI arguments
use std::fs; // To allow for file I/O
use std::process; //So the program may be terminated early

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(x) => x,
        Err(x) => {
            println!("Problem parsing arguments: {}", x);
            process::exit(1);
        }
    };

    // Announce action
    println!(
        "Searching for {}\nIn file {}",
        config.query, config.filename
    );

    // Read file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong when reading the file.");

    // Announce successful read
    println!("With text {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        //Check if the arguments are formatted properly
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
