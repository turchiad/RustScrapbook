use std::fmt::Write;
use std::io;

fn main() {
    
    println!("Provide a sentence:");

    //Define input
    let mut word: String = String::new();

    //Parse input
    io::stdin()
    	.read_line(&mut word)
    	.expect("Failed to read line");

    let word = word.trim();

    let mut sentence = String::from("");

    //counter to skip first whitespace
    let mut i = 0;

    //iterate through each word and construct the pig latin sentence
    for w in word.split(" ") {
    	//Translate word w
    	let s = translate(w);

    	if i != 0 {
    		write!(sentence, " {}", s).unwrap();
    	}
    	else {
    		write!(sentence, "{}", s).unwrap();
    		i += 1
    	}
    }

    println!("{}", sentence);
}

fn translate(word: &str) -> String {
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
    	
    	//Defining the prefix

    	//In case the word should begin with a capital
    	let first_letter = &word.chars().next().unwrap();
    	if first_letter.is_uppercase() {
    		//Make the new first letter capital
    		let new_first_letter = &word[i..].chars().next().unwrap().to_ascii_uppercase();
    		prefix = format!("{}{}",new_first_letter,&word[i+1..]);
    	}
    	else {
    		prefix = (&word[i..]).to_string();
    	}

    	//Defining the postfix

    	//In case the last letter is punctuation
    	let last_letter = &word.chars().last().unwrap();
    	if last_letter.is_ascii_punctuation() {
    		prefix.pop();
    		postfix = format!("{}{}{}",&word[..i].to_ascii_lowercase(),"ay",last_letter)
    	}
    	else {
    		postfix = format!("{}{}",&word[..i].to_ascii_lowercase(),"ay");
    	}
    }

    format!("{}-{}",prefix,postfix)
}

fn is_vowel(c: char) -> bool {
	let c = c.to_ascii_lowercase();
	c == 'a' ||
	c == 'e' ||
	c == 'i' ||
	c == 'o' ||
	c == 'u'
}