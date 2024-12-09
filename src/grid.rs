use std::{
	fmt::Display,
	ops::{Add, Sub},
};

pub struct Grid {
	pub n: usize,
	pub m: usize,
	data: Vec<char>,
}

pub struct DirectionalGridIterator<'g> {
	grid: &'g Grid,
	pos: usize,
	dir: Direction,
}

impl Grid {
	pub fn new(s: &str) -> Self {
		let lines = s.lines().collect::<Vec<_>>();
		let n = lines
			.first()
			.expect("grid can only be constructed from multi-line input")
			.len();

		let m = lines.len();

		let data = lines
			.into_iter()
			.flat_map(|line| line.chars().map(|c| c.into()))
			.collect::<Vec<_>>();

		Self { data, n, m }
	}

	pub fn next_in_direction(&self, pos: usize, dir: Direction) -> Option<(usize, &char)> {
		let x = pos % self.n;
		let y = pos / self.m;

		match dir {
			Direction::N => (y > 0).then(|| pos - self.n),
			Direction::E => (x < self.n - 1).then_some(pos + 1),
			Direction::S => (y < self.m - 1).then_some(pos + self.n),
			Direction::W => (x > 0).then(|| pos - 1),

			Direction::NE => self
				.next_pos_in_direction(pos, Direction::N)
				.and_then(|i| self.next_pos_in_direction(i, Direction::E)),

			Direction::SE => self
				.next_pos_in_direction(pos, Direction::S)
				.and_then(|i| self.next_pos_in_direction(i, Direction::E)),

			Direction::SW => self
				.next_pos_in_direction(pos, Direction::S)
				.and_then(|i| self.next_pos_in_direction(i, Direction::W)),

			Direction::NW => self
				.next_pos_in_direction(pos, Direction::N)
				.and_then(|i| self.next_pos_in_direction(i, Direction::W)),
		}
		.map(|i| (i, &self.data[i]))
	}

	pub fn next_char_in_direction(&self, pos: usize, dir: Direction) -> Option<&char> {
		self.next_in_direction(pos, dir).map(|(_, c)| c)
	}

	pub fn next_pos_in_direction(&self, pos: usize, dir: Direction) -> Option<usize> {
		self.next_in_direction(pos, dir).map(|(i, _)| i)
	}

	pub fn iter_in_direction(&self, pos: usize, dir: Direction) -> DirectionalGridIterator {
		DirectionalGridIterator { grid: self, pos, dir }
	}

	pub fn update_pos(&mut self, pos: usize, c: char) {
		self.data[pos] = c;
	}

	pub fn coords_1d(&self, p: Point) -> Option<usize> {
		self.contains(p)
			.then_some((p.y as usize * self.n) + p.x as usize)
	}

	pub fn coords_2d(&self, i: usize) -> Point {
		Point { x: (i % self.n) as isize, y: (i / self.n) as isize }
	}

	pub fn contains(&self, p: Point) -> bool {
		p.x >= 0 && p.y >= 0 && (p.x as usize) < self.n && (p.y as usize) < self.m
	}
}

impl Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, c) in self.data.iter().enumerate() {
			if (i + 1) % self.n == 0 {
				writeln!(f, "{}", c)?;
			} else {
				write!(f, "{}", c)?;
			}
		}
		Ok(())
	}
}

impl<'c> Iterator for DirectionalGridIterator<'c> {
	type Item = &'c char;

	fn next(&mut self) -> Option<Self::Item> {
		self.grid
			.next_in_direction(self.pos, self.dir)
			.map(|(i, c)| {
				self.pos = i;
				c
			})
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}

impl Direction {
	pub fn all() -> Vec<Direction> {
		vec![
			Self::N,
			Self::NE,
			Self::E,
			Self::SE,
			Self::S,
			Self::SW,
			Self::W,
			Self::NW,
		]
	}

	pub fn diags() -> Vec<(Direction, Direction)> {
		vec![(Self::NW, Self::SE), (Self::NE, Self::SW)]
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
	pub x: isize,
	pub y: isize,
}

impl Sub for Point {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

impl Add for Point {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
