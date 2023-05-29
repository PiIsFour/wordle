use std::{
	fs,
	io::{self, BufRead},
};

fn main() {
	println!("Hello, wordle!");
	print_file("./data/possible_words.txt");
	print_file("./data/allowed_words.txt");
}

fn print_file(path: &str) {
	let file = fs::File::open(path).expect("could not find file possible_words.txt");
	let mut i = 0;
	for line in io::BufReader::new(file).lines().flatten() {
		println!("{line}");
		i += 1;
	}
	println!("possible word count: {i}");
}
