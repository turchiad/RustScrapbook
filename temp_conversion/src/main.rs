use std::io;

fn main() {

	let mut exit = false;
	let mut unit : String = String::new();
	let mut temp : f64 = 0.0;

	while !exit{

		exit = true;

		println!("What would you like to convert to? (C/F)");

		unit = String::new();

		io::stdin()
			.read_line(&mut unit)
			.expect("Failed to read line!");

		match unit.trim() {
			"C" => println!("You chose Fahrenheit to Celsius."),
			"F" => println!("You chose Celsius to Fahrenheit."),
			_ => {
				println!("I didn't understand that.");
				exit = false;
				continue
			}
		};

		unit = unit.trim().into();

		println!("Is that okay? (Y/N)");

		let mut ans = String::new();

		io::stdin()
			.read_line(&mut ans)
			.expect("Failed to read line!");

		if ans.trim().to_lowercase() != "y" {exit = false;}
	}

	exit = false;

	while !exit{

		exit = true;

		println!("Please type in the number in {}", if unit == "C" {"Fahrenheit"} else {"Celsius"});

		let mut ans = String::new();

		io::stdin()
			.read_line(&mut ans)
			.expect("Failed to read line!");

		temp = match ans.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please type a number!");
				exit = false;
				continue
			}
		};
	}

	let result = if unit == "C" {fahrenheit_to_celsius(temp)} else {celsius_to_fahrenheit(temp)};

	println!("{:.2} °{:.2} => {:.2} °{:.2}", temp, if unit == "C" {"F"} else { "C" }, result, unit);
}


fn celsius_to_fahrenheit(temp : f64) -> f64 {
   	(9.0/5.0) * temp + 32.0
}
fn fahrenheit_to_celsius(temp : f64) -> f64 {
   	(temp - 32.0) * (5.0 / 9.0)
}