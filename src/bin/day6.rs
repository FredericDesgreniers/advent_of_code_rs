use itertools::Itertools;
use num_traits::real::Real;

const INPUT: &'static str = include_str!("../../day6_input.txt");

const SIZE: usize = 1000;

fn main() {
	let points = INPUT.lines().filter_map(|line| {
		if let Some(comma) = line.find(',') {
			let (left, right) = line.split_at(comma);
			let x = left.trim().parse::<usize>().unwrap();
			let y = right[1..].trim().parse::<usize>().unwrap();

			Some((x,y))
		} else {
			None
		}
	}).collect_vec();

	//let mut grid = vec![None; SIZE * SIZE];

	let mut region_count = 0;

	for y in 0..SIZE {
		'x: for x in 0..SIZE {
			let mut total_distance = 0;

			for point in points.iter() {
				let distance = (x as i32 - point.0 as i32).abs() + (y as i32 - point.1 as i32).abs();

				total_distance += distance;
			}

			if total_distance < 10000 {
				region_count += 1;
			}
		}
	}

	println!("{}", region_count);
}