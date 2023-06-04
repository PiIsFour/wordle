use rand::seq::SliceRandom;
use std::{
	fs,
	io::{self, BufRead},
	time::Instant,
};
use wordle::result;

fn main() {
	println!("Hello, wordle!");
	let start = Instant::now();
	let possible = load_word_list("./data/possible_words.txt")
		.expect("could not find file possible_words.txt");
	let allowed =
		load_word_list("./data/allowed_words.txt").expect("could not find file allowed_words.txt");
	println!(
		"word lists {}/{} in {:#?}",
		possible.len(),
		allowed.len(),
		start.elapsed()
	);

	let mut rng = rand::thread_rng();
	let solution = possible
		.choose(&mut rng)
		.expect("possible words where empty");
	println!("{:?}", solution);
	let check = |guess: String| result::calculate_matches(&guess, solution);

	random_solver(&allowed, rng, check);
}

fn random_solver<F>(allowed: &[String], mut rng: rand::rngs::ThreadRng, check: F) -> u8
where
	F: Fn(String) -> [result::Match; 5],
{
	let start = Instant::now();
	let mut choices: Vec<String> = allowed.to_vec();
	for i in 1..10 {
		let guess = choices
			.choose(&mut rng)
			.expect("allowed words where empty")
			.to_owned();
		let result = check(guess.clone());
		if result
			== [
				result::Match::Correct,
				result::Match::Correct,
				result::Match::Correct,
				result::Match::Correct,
				result::Match::Correct,
			] {
			println!(
				"solved after {} tries({:#?}): {}",
				i,
				start.elapsed(),
				guess
			);
			return i;
		}
		choices.retain(|word| result == result::calculate_matches(&guess, word));
		println!("{} {:?}: {:?} -> {}", i, guess, result, choices.len());
	}
	99
}

fn load_word_list(path: &str) -> Result<Vec<String>, std::io::Error> {
	let file = fs::File::open(path)?;
	let words = io::BufReader::new(file).lines().flatten().collect();
	Ok(words)
}
