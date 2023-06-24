use itertools::Itertools;
use rand::seq::SliceRandom;
use std::{
	env, fs,
	io::{self, BufRead},
	time::Instant,
};
use wordle::result::{self, Match};

trait ArrayAble {
	fn to_array_5(&self) -> [char; 5];
}

impl ArrayAble for &String {
	fn to_array_5(&self) -> [char; 5] {
		let mut iter = self.chars();
		[
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
		]
	}
}

fn main() {
	println!("Hello, wordle!");
	let start = Instant::now();
	let possible: Vec<[char; 5]> = load_word_list("./data/possible_words.txt")
		.expect("could not find file possible_words.txt")
		.iter()
		.map(|word| word.to_array_5())
		.collect();
	let allowed: Vec<[char; 5]> = load_word_list("./data/allowed_words.txt")
		.expect("could not find file allowed_words.txt")
		.iter()
		.map(|word| word.to_array_5())
		.collect();
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
	println!("{:#?}", solution);
	let check = |guess: &[char; 5]| result::calculate_matches(guess, solution);

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

fn entropy_for(allowed: &[[char; 5]], word: &[char; 5]) -> f64 {
	let sum: f64 = allowed.len() as f64;
	let mut groups: [u16; 243] = [0; 243];
	for s in allowed {
		let matches = result::calculate_matches(word, s);
		let hash = result::hash(&matches);
		groups[hash] += 1;
	}

	groups
		.iter()
		.filter(|list| **list != 0)
		.map(|list| *list as f64)
		.map(|size| size / sum)
		.map(|prob| prob * -f64::log2(prob))
		.sum()
	// println!("{:#?}: {}", entropy, sum);
}

fn entropy_solver(
	allowed: &[[char; 5]],
	mut rng: rand::rngs::ThreadRng,
	check: impl Fn(&[char; 5]) -> [result::Match; 5],
) -> u8 {
	let x: Vec<(&[char; 5], f64)> = allowed
		.iter()
		.map(|s| (s, entropy_for(allowed, s)))
		.collect();
	println!("Ninja Turtle Power: {}", x.len());
	// for (word, entropy) in x {
	// 	println!("{:#?}: {}", word, entropy);
	// }
	99
}

fn random_solver(
	allowed: &[[char; 5]],
	mut rng: rand::rngs::ThreadRng,
	check: impl Fn(&[char; 5]) -> [result::Match; 5],
) -> u8 {
	let start = Instant::now();
	let mut choices: Vec<[char; 5]> = allowed.to_vec();
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
				"solved after {} tries({:#?}): {:#?}",
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
