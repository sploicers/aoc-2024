use crate::util::read_input_lines;

pub fn part1() -> u16 {
	let mut safe_count: u16 = 0;

	for report in read_lines_as_vecs() {
		let pairs: Vec<(u8, u8)> = report.windows(2).map(|pair| (pair[0], pair[1])).collect();

		let (first, second) = *pairs
			.first()
			.expect("Fatal - each report ought to have at least two entries");

		if !safe_difference(first.abs_diff(second)) {
			continue;
		}

		let mut safe_report = true;
		let current_direction = if first < second { 1 } else { -1 };

		for (a, b) in pairs.into_iter().skip(1) {
			let difference = a.abs_diff(b);
			let next_direction = if a < b { 1 } else { -1 };
			if !(safe_difference(difference) && next_direction == current_direction) {
				safe_report = false;
				break;
			}
		}
		if safe_report {
			safe_count += 1;
		}
	}
	safe_count
}

pub fn part2() -> u16 {
	let mut safe_count: u16 = 0;

	for mut report in read_lines_as_vecs() {
		let cloned_report = report
			.clone()
			.iter()
			.map(|x| x.to_string())
			.collect::<Vec<_>>()
			.join(" ");

		if is_safe(&mut report, false) {
			safe_count += 1;
		} else {
			println!("{}", cloned_report);
		}
	}
	safe_count
}

#[derive(PartialEq)]
enum Direction {
	Increasing,
	Decreasing,
}

fn is_safe(report: &mut Vec<u8>, have_skipped: bool) -> bool {
	let mut previous_direction: Option<Direction> = None;

	for i in 1..report.len() {
		let left = report[i - 1];
		let right = report[i];

		if !safe_difference(left.abs_diff(right)) {
			return reevaluate_report(report, i, have_skipped);
		}

		let current_direction = if left < right {
			Direction::Increasing
		} else {
			Direction::Decreasing
		};

		if let Some(ref dir) = previous_direction {
			if *dir != current_direction {
				return reevaluate_report(report, i, have_skipped);
			}
		} else {
			previous_direction = Some(current_direction);
		}
	}
	return true;
}

fn safe_difference(diff: u8) -> bool {
	diff > 0 && diff < 4
}

fn reevaluate_report(report: &mut Vec<u8>, i: usize, have_skipped: bool) -> bool {
	if have_skipped {
		false
	} else {
		let mut cloned_report = report.clone();
		report.remove(i);
		cloned_report.remove(i - 1);

		is_safe(report, true) || is_safe(&mut cloned_report, true)
	}
}

fn read_lines_as_vecs() -> impl Iterator<Item = Vec<u8>> {
	read_input_lines().map(|line| line.split_whitespace().flat_map(|n| n.parse()).collect())
}
