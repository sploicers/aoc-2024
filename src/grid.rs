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
			Direction::N => (y != 0).then(|| pos - self.n),
			Direction::E => (x != 0).then(|| pos + 1),
			Direction::S => (y != self.m - 1).then_some(pos + self.n),
			Direction::W => (x != self.n - 1).then_some(pos - 1),

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
		DirectionalGridIterator {
			grid: self,
			pos,
			dir,
		}
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

#[derive(Clone, Copy)]
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
