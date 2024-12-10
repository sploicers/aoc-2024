use crate::{grid::Grid, util::read_input_as_str};
use std::collections::{HashMap, HashSet};

struct State {
	grid: Box<Grid>,
	antennae_by_pos: HashMap<char, Vec<usize>>,
}

pub fn part1() -> usize {
	let State { grid, antennae_by_pos } = parse();
	let mut antinodes = HashSet::new();

	for positions in antennae_by_pos.values() {
		for i in positions {
			for j in positions {
				if i == j {
					continue;
				}

				let p1 = grid.coords_2d(*i);
				let p2 = grid.coords_2d(*j);

				let a1 = p1 - (p2 - p1);
				let a2 = p2 + (p2 - p1);

				if grid.contains(a1) {
					antinodes.insert(a1);
				}
				if grid.contains(a2) {
					antinodes.insert(a2);
				}
			}
		}
	}
	antinodes.len()
}

pub fn part2() -> usize {
	let State { grid, antennae_by_pos } = parse();
	let mut antinodes = HashSet::new();

	for positions in antennae_by_pos.values() {
		for i in positions {
			for j in positions {
				if i == j {
					continue;
				}

				let mut p1 = grid.coords_2d(*i);
				let mut p2 = grid.coords_2d(*j);
				let dist = p2 - p1;
				antinodes.insert(p1);
				antinodes.insert(p2);

				loop {
					let a = p1 - dist;
					if grid.contains(a) {
						antinodes.insert(a);
						p1 = a;
					} else {
						break;
					}
				}
				loop {
					let a = p2 + dist;
					if grid.contains(a) {
						antinodes.insert(a);
						p2 = a;
					} else {
						break;
					}
				}
			}
		}
	}
	antinodes.len()
}

fn parse() -> State {
	let mut antennae_by_pos: HashMap<char, Vec<usize>> = HashMap::new();
	let input_str = read_input_as_str();
	let grid = Box::new(Grid::new(&input_str));

	for (i, c) in input_str.replace('\n', "").chars().enumerate() {
		if c.is_alphanumeric() {
			if let Some(positions) = antennae_by_pos.get_mut(&c) {
				positions.push(i);
			} else {
				antennae_by_pos.insert(c, vec![i]);
			}
		}
	}
	State { grid, antennae_by_pos }
}
