use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Match {
	None,
	Correct,
	Somewhere,
}

/// returns the matching pattern according to wordle
///
/// ```
/// use wordle::result::{*, Match::*};
/// let result = calculate_matches("xxxef", "abxxc");
/// assert_eq!(result, [Somewhere, None, Correct, None, None]);
/// ```
pub fn calculate_matches(guess: &str, solution: &str) -> [Match; 5] {
	let mut result = [
		Match::None,
		Match::None,
		Match::None,
		Match::None,
		Match::None,
	];
	for (result, (guess, solution_char)) in
		result.iter_mut().zip(guess.chars().zip(solution.chars()))
	{
		if guess == solution_char {
			*result = Match::Correct;
		}
	}

	let mut x: HashMap<char, u32> = result
		.iter()
		.zip(solution.chars())
		.filter(|(result, _)| **result != Match::Correct)
		.map(|(_, char)| char)
		.sorted()
		.group_by(|&x| x)
		.into_iter()
		.map(|(k, v)| (k, v.count() as u32))
		.collect();
	for (result, guess) in result.iter_mut().zip(guess.chars()) {
		if let Some(count) = x.get_mut(&guess) {
			if *count > 0 {
				*result = Match::Somewhere;
				*count -= 1;
			}
		}
	}
	result
}

#[cfg(test)]
mod tests {
	use super::Match::*;
	use super::*;

	#[test]
	fn it_has_no_match() {
		let result = calculate_matches("aaaaa", "bbbbb");
		assert_eq!(result, [None, None, None, None, None]);
	}

	#[test]
	fn it_has_one_match() {
		let result = calculate_matches("acccc", "abbbb");
		assert_eq!(result, [Correct, None, None, None, None]);
	}

	#[test]
	fn it_has_one_match_somewhere_else() {
		let result = calculate_matches("acccc", "bbbba");
		assert_eq!(result, [Somewhere, None, None, None, None]);
	}
}
