use std::process; //Provide a means of exiting the program
use std::io;
use std::collections::HashMap;

fn main() {
	menu();
}

//Print the main menu, wait for a reply, and direct the user to the necessary section
fn menu() {

	println!("Welcome to the Company Employee Interface\n");

	loop {

		println!("Please select one of following:\n\
		I/Insert\n\
		Q/Query\n\
		E/Exit\n");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line.");

		let input = input.trim().to_ascii_lowercase();

		match input.as_str() {
			"i" | "insert" => insert(),
			"q" | "query" => query(),
			"e" | "exit" | "quit" => process::exit(1),
			_ => {
				print!("Unexpected answer, please only use the options given.");
				continue;
			},
		}
	}
}

//Handle the user interface in terms of inserting data into the system
fn insert() {

	loop {

		println!("Please specify the insert in the following format:\n\
			Add <Employee> to <Department>");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line.");

		let input = input.trim().to_ascii_lowercase();
	}

}

//Handle the user interface in terms of reporting data into the system
fn query() {

}