use std::collections::HashSet;

use crate::grid::{Direction, Grid};
use crate::util::read_input_as_str;

pub fn part1() -> usize {
	let input_str = read_input_as_str();
	let grid = Grid::new(&input_str);
	let mut visited: HashSet<usize> = HashSet::new();

	let mut guard_direction = Direction::N;
	let mut guard_position = input_str
		.replace('\n', "")
		.find('^')
		.expect("Input string should contain a guard.");

	visited.insert(guard_position);

	loop {
		match grid.next_in_direction(guard_position, guard_direction) {
			Some((_, '#')) => {
				guard_direction = match guard_direction {
					Direction::N => Direction::E,
					Direction::E => Direction::S,
					Direction::S => Direction::W,
					Direction::W => Direction::N,
					_ => unreachable!(),
				}
			}
			Some((pos, _)) => {
				guard_position = pos;
				visited.insert(pos);
			}
			None => break,
		}
	}
	visited.len()
}

pub fn part2() -> i32 {
	todo!()
}
