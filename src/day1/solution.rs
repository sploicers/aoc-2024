use std::{collections::HashMap, i32};

use crate::util::read_input_lines;

pub fn part1() -> i32 {
	let (mut left, mut right) = split_input();
	left.sort();
	right.sort();

	left.iter()
		.zip(right)
		.map(|(left, right)| (right - left).abs())
		.sum()
}

pub fn part2() -> i32 {
	let (left, right) = split_input();
	let mut scores: HashMap<i32, i32> = HashMap::new();

	for x in right.into_iter() {
		scores.insert(x, scores.get(&x).map(|existing| existing + 1).unwrap_or(1));
	}
	left.iter().map(|n| n * scores.get(n).unwrap_or(&0)).sum()
}

fn split_input() -> (Vec<i32>, Vec<i32>) {
	read_input_lines()
		.map(|line| {
			let (left, right) = line
				.split_once("   ")
				.expect("Fatal - encounted malformed line.");
			return (
				left.parse::<i32>().expect("Couldn't parse LHS as i16"),
				right.parse::<i32>().expect("Couldn't parse RHS as i16"),
			);
		})
		.unzip()
}
