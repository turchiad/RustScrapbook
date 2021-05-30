use std::env; // To allow access of CLI arguments
use std::process; //So the program may be terminated early

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Problem parsing arguments: {}", x);
            process::exit(1);
        }
    };

    // // Announce action
    // println!(
    //     "Searching for {}\nIn file {}",
    //     config.query, config.filename
    // );

    // Program logic
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
