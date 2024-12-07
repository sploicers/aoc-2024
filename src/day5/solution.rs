use crate::util::read_input_lines;
use std::{cmp::Ordering, collections::HashMap};

struct Puzzle {
	rules: HashMap<u8, Vec<u8>>,
	updates: Vec<Vec<u8>>,
}

pub fn part1() -> u16 {
	let Puzzle { rules, updates } = parse();
	let mut sum: u16 = 0;

	for update in updates {
		let mut in_order = true;
		for (n, m) in update.windows(2).map(|pair| (pair[0], pair[1])) {
			if let Some(requirements) = rules.get(&n) {
				if requirements.contains(&m) {
					in_order = false;
					break;
				}
			}
		}
		if in_order {
			let mid = update[update.len() / 2];
			sum += mid as u16;
		}
	}
	sum
}

pub fn part2() -> u16 {
	let Puzzle { rules, updates } = parse();
	let mut sum: u16 = 0;

	for mut update in updates {
		let mut in_order = true;
		for (n, m) in update.windows(2).map(|pair| (pair[0], pair[1])) {
			if let Some(requirements) = rules.get(&n) {
				if requirements.contains(&m) {
					in_order = false;
					break;
				}
			}
		}

		if !in_order {
			update.sort_by(|n, m| {
				if let Some(requirements) = rules.get(&n) {
					if requirements.contains(&m) {
						Ordering::Less
					} else {
						Ordering::Greater
					}
				} else {
					Ordering::Equal
				}
			});

			let mid = update[update.len() / 2];
			sum += mid as u16;
		}
	}
	sum
}

fn parse() -> Puzzle {
	let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
	let mut updates: Vec<Vec<u8>> = vec![];
	let mut in_rules_section = true;

	for line in read_input_lines() {
		if line.is_empty() {
			in_rules_section = false;
			continue;
		}
		if in_rules_section {
			let (lhs, rhs): (u8, u8) = line
				.split_once('|')
				.map(|(a, b)| {
					(
						a.parse().expect("Failed to parse LHS as int"),
						b.parse().expect("Failed to parse RHS as int"),
					)
				})
				.expect("Fatal - rules expected to be of the form x|y.");

			if let Some(entries) = rules.get_mut(&rhs) {
				entries.push(lhs);
			} else {
				rules.insert(rhs, vec![lhs]);
			}
		} else {
			updates.push(line.split(',').flat_map(|s| s.parse()).collect())
		}
	}

	Puzzle { rules, updates }
}
