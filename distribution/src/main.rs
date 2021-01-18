use std::io;
use std::collections::HashMap;

fn main() {

    println!("Provide a list one number at a time (Q to stop):");

    let mut num_list: Vec<f64> = Vec::new();

    loop {

	    let mut input = String::new();

	    io::stdin()
	    	.read_line(&mut input)
	    	.expect("Failed to read line");

	    if input.trim() == "Q" {
	    	if num_list.len() <= 0 {
	    		println!("There needs to be at least one element. Try again.");
	    		continue;
	    	}
	    	else {
	    		break;
	    	}
	    }

	    let input: f64 = match input.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => {
	    		println!("Please provide only numbers or \"Q\"");
	    		continue;
	    	}
	    };

	    num_list.push(input);
    }

    println!(
    	"Statistics:\nMEAN: {}\nMEDIAN: {}\nMODE (First Encountered): {}\n",
    	mean(&num_list),
    	median(&num_list),
    	mode(&num_list)
    );
}

fn mean(v: &Vec<f64>) -> f64 {

	let mut sum: f64 = 0.0;
	let n = v.len() as f64;
	for e in v {
		sum += e;
	}

	sum / n
}

fn median(v: &Vec<f64>) -> f64 {
	
	let mut sorted = v.clone();
	sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

	let mid = (sorted.len() as f64) / 2.0;

	if mid % 1.0 == 0.0 {
		if mid as i32 - 1 < 0 {
			println!("WARNING: Unexpected error, median average has a negative index");
			0.0
		}
		else {
		(sorted[mid as usize] + sorted[mid as usize - 1]) / 2.0
		}
	}
	else {
		sorted[mid as usize]
	}
}

fn mode(v: &Vec<f64>) -> f64 {

	let mut map = HashMap::new();

	//Populate Hashmap with counts
	for e in v {
		let count = map.entry(e.to_string()).or_insert(0);
		*count += 1;
	}

	let mut result : (Option<f64>, u32) = (None, 0);

	//Iterate over Hashmap for max value
	for e in v {
		let n = map.get(&e.to_string());
		match n {
			Some(&i) => {
				if i > result.1 {
					result = (Some(*e), i);
				}
			}
			None => {}
		}
		
	}

	match result.0 {
		Some(i) => i, 
		None => {
			println!("WARNING: Unexpected error, no mode found.");
			404.0
		}
	}

}