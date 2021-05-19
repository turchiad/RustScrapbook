use std::io;
use std::collections::HashMap;

fn main() {

	let mut database = HashMap::<String,Vec<String>>::new();

	let instructions = "To add to the database type: \"Add <employee> to <department>\"\n\
		To retrieve a list of employees in a department type: \"Show <department>\"\n\
		To retrieve a list of all employees by department type: \"Showfull\"\n\
		To exit the program type: \"q\" or \"Quit\"";

	println!("Welcome to the Employee Database.\n");

	println!("{}", instructions);

	loop {

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line.");

		let input = input.trim();
		let input_lowercase = input.to_ascii_lowercase();

		match input_decider(&input_lowercase) {
			-1 => println!("Invalid input, please try again."),
			0 => break,
			1 => handle_add(&input_lowercase),
			2 => handle_show(&input_lowercase),
			3 => handle_showfull(&input_lowercase),
			_ => println!("Unexpected error, please try again.")
		}
	}
}

// This function determines what action the user is attempting given the input string
// and returns an int representing the action
//	outputs:
//		-1	-> invalid input
//		0	-> quit
//		1	-> add
//		2	-> show
//		3	-> showfull
fn input_decider(s: &String) -> i8
{
	if s == "q" || s == "quit" {
		0
	}
	else if s.starts_with("add ") {
		1
	}
	else if s == "showfull" {
		3
	}
	else if s.starts_with("show ") {
		2
	}
	else {
		-1
	}
}

// This function determines whether the action the user is attempting has been formatted
// correctly and returns true if so and false if not.
//	inputs:
//	i8 d -> indicates which decision has been chosen, consistent with input_decider()
//  &String s -> the raw input from the user
// 
//  correct formats:
//  1 => add <employee> to <department>
//  2 => show <department>
//  3 => showfull

fn is_valid_input(d: i8, s: &String) -> bool {
	match d {
		1 => {
			// Split input
			let words = s.split_whitespace();
			// Determine validity
			
			let mut counter = 0;
			// keyword_1 is detection of "add"
			let mut keyword_1 = false;
			// keyword 2 is detection of "to"
			let mut keyword_2 = false;
			// final_check is detection of a word after "to"
			let mut final_check = false;

			for word in words {
				// If we haven't found "add"
				if !keyword_1{
					// If it's not the first word and we haven't found add,
					// it's not formatted correctly
					// We should never arrive here, but it doesn't hurt to check.
					if counter != 0 {
						break;
					}
					else if word == "add" {
						keyword_1 = true;
						counter += 1
					}
					// If the first word isn't add
					else {
						break;
					}
				}
				// If we haven't found the "to"
				else if !keyword_2 {
					// If it's the second word, it can't be "to"
					if counter == 1 {
						counter += 1;
						continue;
					} 
					// If it's not the second word, it can be "to"
					else if word == "to"
					{
						keyword_2 = true;
					}
					// If neither of these are the case, than "to" is never found
				}
				// If we have found keyword_2 (and therefore both keywords)
				// And if "to" is not the last word.
				else if keyword_2
				{
					final_check = true;
				}
			}

			//debug
			//println!("counter: {}\nkeyword_1: {}\nkeyword_2: {}\nfinal_check: {}\n", counter, keyword_1, keyword_2, final_check);

			return keyword_1 && keyword_2 && final_check
		},
		_ => {
			println!("Unexpected error, please try again.");
			return false;
		}
	}
}

// This function handles additions to the database.
fn handle_add(s: &String) {
	if(is_valid_input(1, s))
	{
		println!("Success");
	}
	else {
		println!("Add attempt not formatted properly, please try again.");
	}
}

// This function handles requests to view departments from the database.
fn handle_show(s: &String) {
	unimplemented!()
}

// This function handles requests to view the full database
fn handle_showfull(s: &String) {
	unimplemented!()
}