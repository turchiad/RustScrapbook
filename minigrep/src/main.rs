use std::env; // To allow access of CLI arguments
use std::fs; // To allow for file I/O

fn main() {
    let args: Vec<String> = env::args().collect();

    //println!("{:?}", args);

    // String we're looking for
    let query = &args[1];

    // Filename we're looking in
    let filename = &args[2];

    // Announce action
    println!("Searching for {}\nIn file {}", query, filename);

    // Read file
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");

    // Announce successful read
    println!("With text {}", contents);
}
