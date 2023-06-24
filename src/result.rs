use core::ops::Range;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum Match {
	None,
	Correct,
	Somewhere,
}

impl Hash for Match {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Match::None => 0,
			Match::Correct => 1,
			Match::Somewhere => 2,
		}
		.hash(state);
	}
}

/// returns a hash for the matches array
///
/// ```
/// use wordle::result::{*, Match::*};
/// let matches = [Somewhere, None, Correct, None, None];
/// let hash_value = hash(&matches);
/// assert_eq!(hash_value, 2 * 3_usize.pow(4) + 1 * 3_usize.pow(2));
/// ```
pub fn hash(matches: &[Match; 5]) -> usize {
	let mut hash = 0;
	for m in matches {
		let digit = match m {
			Match::None => 0,
			Match::Correct => 1,
			Match::Somewhere => 2,
		};
		hash = hash * 3 + digit;
	}
	hash
}

/// returns the matching pattern according to wordle
///
/// ```
/// use wordle::result::{*, Match::*};
/// let guess = ['x', 'x', 'x', 'e', 'f'];
/// let solution = ['a', 'b', 'x', 'x', 'c'];
/// let result = calculate_matches(&guess, &solution);
/// assert_eq!(result, [Somewhere, None, Correct, None, None]);
/// ```
pub fn calculate_matches(guess: &[char; 5], solution: &[char; 5]) -> [Match; 5] {
	let mut result = [
		Match::None,
		Match::None,
		Match::None,
		Match::None,
		Match::None,
	];
	let mut characters: [u8; 26] = [0; 26];

	for i in 0..5 {
		let result = &mut result[i];
		let guess = guess[i];
		let solution = solution[i];
		if guess == solution {
			*result = Match::Correct;
		} else {
			characters[(solution as usize) - ('a' as usize)] += 1;
		}
	}

	for i in 0..5 {
		let result = &mut result[i];
		if *result == Match::Correct {
			continue;
		}
		let guess = guess[i];
		let count = &mut characters[(guess as usize) - ('a' as usize)];
		if *count > 0 {
			*result = Match::Somewhere;
			*count -= 1;
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
		let guess = ['a', 'a', 'a', 'a', 'a'];
		let solution = ['b', 'b', 'b', 'b', 'b'];
		let result = calculate_matches(&guess, &solution);
		assert_eq!(result, [None, None, None, None, None]);
	}

	#[test]
	fn it_has_one_match() {
		let guess = ['a', 'c', 'c', 'c', 'c'];
		let solution = ['a', 'b', 'b', 'b', 'b'];
		let result = calculate_matches(&guess, &solution);
		assert_eq!(result, [Correct, None, None, None, None]);
	}

	#[test]
	fn it_has_one_match_somewhere_else() {
		let guess = ['a', 'c', 'c', 'c', 'c'];
		let solution = ['b', 'b', 'b', 'b', 'a'];
		let result = calculate_matches(&guess, &solution);
		assert_eq!(result, [Somewhere, None, None, None, None]);
	}

	#[test]
	fn it_correct_has_precedence() {
		let guess = ['a', 'x', 'x', 'x', 'x'];
		let solution = ['a', 'a', 'y', 'y', 'y'];
		let result = calculate_matches(&guess, &solution);
		assert_eq!(result, [Correct, None, None, None, None]);
	}
}
