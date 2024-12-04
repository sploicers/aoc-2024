use crate::util::{get_input_reader, regex_or_panic};
use regex::Captures;
use std::io::Read;

pub fn part1() -> u32 {
	let mut sum = 0;
	let regex = regex_or_panic(r"(mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)|don't\(\)|do\(\))");

	for cap in regex.captures_iter(&get_input_as_string()) {
		match Instruction::from(cap) {
			Instruction::Mul(x, y) => {
				sum += x * y;
			}
			_ => continue,
		}
	}
	sum
}

pub fn part2() -> u32 {
	let mut sum = 0;
	let regex = regex_or_panic(r"(mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)|don't\(\)|do\(\))");
	let mut multiplication_enabled = true;

	for cap in regex.captures_iter(&get_input_as_string()) {
		match Instruction::from(cap) {
			Instruction::Mul(x, y) => {
				if multiplication_enabled {
					sum += x * y;
				}
			}
			Instruction::Enable => {
				multiplication_enabled = true;
			}
			Instruction::Disable => {
				multiplication_enabled = false;
			}
		}
	}
	sum
}

fn get_input_as_string() -> String {
	let mut s = String::new();
	get_input_reader()
		.read_to_string(&mut s)
		.expect("Failed to read input.");
	s
}

#[derive(Debug)]
enum Instruction {
	Mul(u32, u32),
	Enable,
	Disable,
}

impl From<Captures<'_>> for Instruction {
	fn from(value: Captures<'_>) -> Self {
		match (&value.name("x"), &value.name("y")) {
			(Some(lhs), Some(rhs)) => {
				let x = lhs.as_str().parse().expect("Failed to parse left operand");
				let y = rhs.as_str().parse().expect("Failed to parse right operand");
				Instruction::Mul(x, y)
			}
			_ => match value.get(0).map(|m| m.as_str()) {
				Some("do()") => Instruction::Enable,
				Some("don't()") => Instruction::Disable,
				_ => unreachable!(),
			},
		}
	}
}
