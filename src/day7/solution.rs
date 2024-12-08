use std::io::BufRead;

use crate::util::get_input_reader;

pub fn part1() -> usize {
	let mut sum: usize = 0;

	for line in get_input_reader().lines().flatten() {
		let (lhs, rhs) = line.split_once(": ").expect("Encountered malformed line.");

		let nums: Vec<usize> = rhs
			.split_ascii_whitespace()
			.flat_map(|s| s.parse())
			.collect();

		let target: usize = lhs.parse().expect("Failed to parse LHS as int");
		let ways = combinations(State { nums, operators: vec![], target });

		if ways > 0 {
			sum += target;
		}
	}
	sum
}

fn combinations(state: State) -> usize {
	if state.is_filled() {
		state.evaluates_to_target() as usize
	} else {
		combinations(state.fill_next('+')) + combinations(state.fill_next('*'))
	}
}

#[derive(Clone)]
struct State {
	nums: Vec<usize>,
	operators: Vec<char>,
	target: usize,
}

impl State {
	pub fn is_filled(&self) -> bool {
		self.operators.len() == self.nums.len() - 1
	}

	pub fn evaluates_to_target(&self) -> bool {
		let mut i = 0;
		let mut j = 1;
		let mut result = self.nums[i];

		while j < self.nums.len() {
			result = match self.operators[i] {
				'*' => result * self.nums[j],
				'+' => result + self.nums[j],
				_ => unreachable!(),
			};
			i += 1;
			j += 1;
		}
		result == self.target
	}

	pub fn fill_next(&self, op: char) -> Self {
		let mut clone = self.clone();
		clone.operators.push(op);
		clone
	}
}
