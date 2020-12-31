use std::io;

fn main() {

    loop {

    	println!("Which Fibonacci number?");

	    let mut i = String::new();

	    io::stdin()
	    	.read_line(&mut i)
	    	.expect("Numbers only please!");

	    let i : u32 = match i.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => {
	    		println!("Positive numbers only please!");
	    		continue
	    	}
	    };

	    let result = fib(i);

	    println!("The result is: {}", result);

	    break;
	}
}

fn fib(i : u32) -> u32 {
	if i == 0 {0} else {fib_help(0, 1, 1, i)}	
}

fn fib_help(last : u32, this : u32, index : u32, end : u32) -> u32
{
	if index == end {this} else {fib_help(this, last + this, index + 1, end)}
}