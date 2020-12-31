use std::io;

fn main() {
    
	let lyrics = [
		"a partridge in a pear tree",
		"two turtle doves",
		"three french hens",
		"four calling birds",
		"five golden rings",
		"six geese a-laying",
		"seven swans a-swimming",
		"eight maids a-milking",
		"nine ladies dancing",
		"ten lords a-leaping",
		"eleven pipers piping",
		"twelve drummers drumming"
		];

	let ordinals = [
		"first",
		"second",
		"third",
		"fourth",
		"fifth",
		"sixth",
		"seventh",
		"eighth",
		"ninth",
		"tenth",
		"eleventh",
		"twelfth"
	];

	for i in 1..lyrics.len()+1
	{
		println!("On the {} day of Christmas, my true love gave to me!...", ordinals[i-1]);
		
		for lyric in (lyrics[0..i]).iter().rev()
		{
			if i > 1 && *lyric == lyrics[0]
			{
				println!("and a {}!", lyric)
			}
			else {
				println!("{}...", lyric)
			}
		}

		println!("<Press enter to continue>");

		let _ = io::stdin().read_line(&mut String::new());
	}
}
