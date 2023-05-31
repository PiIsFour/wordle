use std::{
	fs,
	io::{self, BufRead},
};

use wordle::result;

fn main() {
	println!("Hello, wordle!");
	let possible = load_word_list("./data/possible_words.txt")
		.expect("could not find file possible_words.txt");
	let allowed =
		load_word_list("./data/allowed_words.txt").expect("could not find file allowed_words.txt");
	println!("word lists {}/{}", possible.len(), allowed.len());
	let solution = "nails";
	let r = result::calculate_matches("hello", solution);
	println!("{:?}", r);
	for word in possible {
		if r == result::calculate_matches(&word, solution) {
			println!("{word}: {:?}", r)
		}
	}
}

fn load_word_list(path: &str) -> Result<Vec<String>, std::io::Error> {
	let file = fs::File::open(path)?;
	let words = io::BufReader::new(file).lines().flatten().collect();
	Ok(words)
}
