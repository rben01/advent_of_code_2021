// tag::setup[]
use crate::Answer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Brace {
	Paren,
	Square,
	Curly,
	Angle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Orientation {
	Left,
	Right,
}

impl Orientation {
	fn flip(self) -> Self {
		use Orientation::*;
		match self {
			Left => Right,
			Right => Left,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Token {
	brace: Brace,
	orientation: Orientation,
}

impl Token {
	fn from_char(c: char) -> Option<Self> {
		use Brace::*;
		use Orientation::*;

		let (orientation, brace) = match c {
			'(' => (Left, Paren),
			')' => (Right, Paren),
			'[' => (Left, Square),
			']' => (Right, Square),
			'{' => (Left, Curly),
			'}' => (Right, Curly),
			'<' => (Left, Angle),
			'>' => (Right, Angle),
			_ => return None,
		};

		Some(Self { brace, orientation })
	}
	fn flip(self) -> Self {
		Self {
			orientation: self.orientation.flip(),
			..self
		}
	}
}

enum TokenizationErr {
	Corrupted(Token),
	Incomplete(Vec<Token>),
}

type ParseResult = Result<(), TokenizationErr>;

fn parse_line<T: std::borrow::Borrow<Token>>(line: impl Iterator<Item = T>) -> ParseResult {
	use Orientation::*;

	let mut token_stack = Vec::new();
	for curr in line {
		let curr = *curr.borrow();
		match token_stack.last() {
			None => {
				token_stack.push(curr);
			}
			Some(&prev) => {
				if prev.orientation == Left && curr.orientation == Right {
					if prev.brace != curr.brace {
						return Err(TokenizationErr::Corrupted(curr));
					}
					token_stack.pop();
				} else {
					token_stack.push(curr);
				}
			}
		}
	}

	if !token_stack.is_empty() {
		return Err(TokenizationErr::Incomplete(
			token_stack.iter().rev().map(|t| t.flip()).collect(),
		));
	}

	Ok(())
}

fn read_input(input: &str) -> Option<Vec<Vec<Token>>> {
	input
		.lines()
		.map(|line| {
			line.trim()
				.chars()
				.map(Token::from_char)
				.collect::<Option<Vec<_>>>()
		})
		.collect::<Option<Vec<_>>>()
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let tokens = read_input(input).unwrap();
	let parsed_lines = tokens
		.iter()
		.map(|v| parse_line(v.iter()))
		.collect::<Vec<_>>();
	(10, (pt1(parsed_lines.iter()), pt2(parsed_lines.iter()))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1<P: std::borrow::Borrow<ParseResult>>(prs: impl Iterator<Item = P>) -> usize {
	use Brace::*;
	prs.filter_map(|r| {
		let r = r.borrow();
		if let Err(TokenizationErr::Corrupted(t)) = r {
			Some(match t.brace {
				Paren => 3,
				Square => 57,
				Curly => 1197,
				Angle => 25137,
			})
		} else {
			None
		}
	})
	.sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2<P: std::borrow::Borrow<ParseResult>>(prs: impl Iterator<Item = P>) -> usize {
	use Brace::*;
	let mut scores = prs
		.filter_map(|r| {
			let r = r.borrow();
			if let Err(TokenizationErr::Incomplete(tokens)) = r {
				let mut score = 0_usize;
				for t in tokens {
					score *= 5;
					let token_score = match t.brace {
						Paren => 1,
						Square => 2,
						Curly => 3,
						Angle => 4,
					};
					score += token_score;
				}
				Some(score)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	scores.sort_unstable();
	scores[scores.len() / 2]
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("input.txt"), day: 10, ans: (167_379, 2_776_842_859));
	}
}
