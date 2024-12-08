use std::io::BufRead;

use crate::util::get_input_reader;

#[derive(Clone)]
enum Operator {
	Add,
	Mul,
	Cat,
}

pub fn part1() -> usize {
	evaluate(combinations)
}

pub fn part2() -> usize {
	evaluate(combinations2)
}

fn evaluate(count_combinations: fn(State) -> usize) -> usize {
	let mut sum: usize = 0;

	for line in get_input_reader().lines().flatten() {
		let (lhs, rhs) = line.split_once(": ").expect("Encountered malformed line.");

		let nums: Vec<usize> = rhs
			.split_ascii_whitespace()
			.flat_map(|s| s.parse())
			.collect();

		let target: usize = lhs.parse().expect("Failed to parse LHS as int");
		let ways = count_combinations(State { nums, operators: vec![], target });

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
		combinations(state.fill_next(Operator::Add)) + combinations(state.fill_next(Operator::Mul))
	}
}

fn combinations2(state: State) -> usize {
	if state.is_filled() {
		state.evaluates_to_target() as usize
	} else {
		combinations2(state.fill_next(Operator::Add))
			+ combinations2(state.fill_next(Operator::Mul))
			+ combinations2(state.fill_next(Operator::Cat))
	}
}

#[derive(Clone)]
struct State {
	nums: Vec<usize>,
	operators: Vec<Operator>,
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
				Operator::Mul => result * self.nums[j],
				Operator::Add => result + self.nums[j],
				Operator::Cat => (result.to_string() + &self.nums[j].to_string())
					.parse()
					.expect("Failed to parse concatenated nums as int"),
			};
			i += 1;
			j += 1;
		}
		result == self.target
	}

	pub fn fill_next(&self, op: Operator) -> Self {
		let mut clone = self.clone();
		clone.operators.push(op);
		clone
	}
}
