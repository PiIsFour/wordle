use rand::seq::SliceRandom;
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
	let mut rng = rand::thread_rng();
	let solution = possible
		.choose(&mut rng)
		.expect("possible words where empty");
	println!("{:?}", solution);
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("no user input");
	let result = result::calculate_matches(&guess, solution);
	println!("{:?}", result);
}

fn load_word_list(path: &str) -> Result<Vec<String>, std::io::Error> {
	let file = fs::File::open(path)?;
	let words = io::BufReader::new(file).lines().flatten().collect();
	Ok(words)
}
