use crate::util::read_input_as_str;

use super::grid::{Direction, SquareGrid};

pub fn part1() -> i32 {
	let input_str = read_input_as_str();
	let grid = SquareGrid::new(&input_str);
	let mut xmas_count = 0;

	for (i, char) in input_str.chars().filter(|c| !c.is_whitespace()).enumerate() {
		if char == 'X' {
			for d in Direction::all() {
				let next_three_chars: String = grid.iter_in_direction(i, d).take(3).collect();

				if next_three_chars == "MAS" {
					xmas_count += 1;
				}
			}
		}
	}
	xmas_count
}

pub fn part2() -> i32 {
	let input_str = read_input_as_str();
	let grid = SquareGrid::new(&input_str);
	let mut xmas_count = 0;

	for (i, char) in input_str.chars().filter(|c| !c.is_whitespace()).enumerate() {
		if char == 'A' {
			let mut diag_matches = 0;

			for (d1, d2) in Direction::diags() {
				match (
					grid.next_char_in_direction(i, d1),
					grid.next_char_in_direction(i, d2),
				) {
					(Some('M'), Some('S')) | (Some('S'), Some('M')) => {
						diag_matches += 1;
					}
					_ => continue,
				}
			}
			if diag_matches == 2 {
				xmas_count += 1;
			}
		}
	}
	xmas_count
}
