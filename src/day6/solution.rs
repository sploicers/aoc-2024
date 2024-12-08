use std::collections::HashSet;

use crate::grid::{Direction, Grid};
use crate::util::read_input_as_str;

pub fn part1() -> usize {
	let input_str = read_input_as_str();
	let grid = Grid::new(&input_str);
	traverse(&grid, &input_str, visit_noop).len()
}

pub fn part2() -> usize {
	let input_str = read_input_as_str();
	let mut grid = Grid::new(&input_str);
	let path = traverse(&grid, &input_str, visit_noop);
	let mut count = 0;

	for pos in path.visited_uniq().into_iter().skip(1) {
		grid.update_pos(*pos, '#');
		let new_path = traverse(&grid, &input_str, visit_if_not_seen);
		if new_path.has_cycle {
			count += 1;
		}
		grid.update_pos(*pos, '.');
	}

	count
}

fn traverse(grid: &Grid, input_str: &str, visit: fn(usize, Direction, &Path) -> bool) -> Path {
	let mut guard_direction = Direction::N;
	let mut guard_position = input_str
		.replace('\n', "")
		.find('^')
		.expect("Input string should contain a guard.");

	let mut path = Path::new();
	path.append(guard_position, guard_direction);

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
				if !visit(pos, guard_direction, &path) {
					path.has_cycle = true;
					break;
				}
				guard_position = pos;
				path.append(guard_position, guard_direction);
			}
			None => break,
		}
	}
	path
}

fn visit_noop(_: usize, _: Direction, _: &Path) -> bool {
	true
}

fn visit_if_not_seen(pos: usize, dir: Direction, path: &Path) -> bool {
	!path.contains(pos, dir)
}

struct Path {
	visited: HashSet<(usize, Direction)>,
	has_cycle: bool,
}

impl Path {
	pub fn new() -> Self {
		Self { visited: HashSet::new(), has_cycle: false }
	}

	pub fn len(&self) -> usize {
		self.visited_uniq().len()
	}

	pub fn visited_uniq(&self) -> HashSet<&usize> {
		self.visited
			.iter()
			.map(|(pos, _)| pos)
			.collect::<HashSet<_>>()
	}

	pub fn append(&mut self, pos: usize, dir: Direction) {
		self.visited.insert((pos, dir));
	}

	pub fn contains(&self, pos: usize, dir: Direction) -> bool {
		self.visited.contains(&(pos, dir))
	}
}
