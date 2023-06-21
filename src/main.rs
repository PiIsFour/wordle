use itertools::Itertools;
use rand::seq::SliceRandom;
use std::{
	env, fs,
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
	let check = |guess: &str| result::calculate_matches(guess, solution);

	let args: Vec<String> = env::args().collect();

	match &args[..] {
		[_] => entropy_solver(&possible, rng, check),
		[_, a] if a == "hello" => {
			println!("hello");
			0
		}
		[_, a] if a == "random" => random_solver(&allowed, rng, check),
		[_, a] if a == "entropy" => entropy_solver(&allowed, rng, check),
		_ => panic!("parameters not recognized"),
	};
}

fn entropy_for(allowed: &[String], word: &str) -> f64 {
	let groups = allowed
		.iter()
		.into_group_map_by(|s| result::calculate_matches(word, s));
	let bars: Vec<f64> = groups.values().map(|list| list.len() as f64).collect();
	let sum: f64 = bars.iter().sum();
	let entropy = bars
		.iter()
		.map(|size| size / sum)
		.map(|prob| prob * -f64::log2(prob))
		.sum();
	// println!("{:#?}: {}", entropy, sum);
	entropy
}

fn entropy_solver(
	allowed: &[String],
	mut rng: rand::rngs::ThreadRng,
	check: impl Fn(&str) -> [result::Match; 5],
) -> u8 {
	let x = allowed.iter().map(|s| (s, entropy_for(allowed, s)));
	for (word, entropy) in x {
		println!("{}: {}", word, entropy);
	}
	99
}

fn random_solver(
	allowed: &[String],
	mut rng: rand::rngs::ThreadRng,
	check: impl Fn(&str) -> [result::Match; 5],
) -> u8 {
	let start = Instant::now();
	let mut choices: Vec<String> = allowed.to_vec();
	for i in 1..10 {
		let guess = choices
			.choose(&mut rng)
			.expect("allowed words where empty")
			.to_owned();
		let result = check(&guess);
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
