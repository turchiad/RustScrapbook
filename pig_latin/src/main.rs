use std::io;

fn main() {
    
    println!("Provide a word:");

    //Define input
    let mut word: String = String::new();

    //Parse input
    io::stdin()
    	.read_line(&mut word)
    	.expect("Failed to read line");

    let word = word.trim();

    //index of first vowel
    let mut i = 0;

    //Look for first vowel
    for c in word.chars() {
    	if !is_vowel(c) {
    		i += 1;
    	}
    	else {
    		break;
    	}
    }

    //Define the default prefix
    let mut prefix = word.to_string();
    //Define the default postfix
    let mut postfix = "yay".to_string();
    
    //In case the first letter is not vowel
    if i > 0 {
    	prefix = (&word[i..]).to_string();
    	postfix = format!("{}{}",&word[..i],"ay");
    }

    let new_word = format!("{}{}",prefix,postfix);

    println!("{}", new_word);
}

fn is_vowel(c: char) -> bool {
	let c = c.to_ascii_lowercase();
	c == 'a' ||
	c == 'e' ||
	c == 'i' ||
	c == 'o' ||
	c == 'u'
}