use itertools::Itertools;
use num_traits::real::Real;

const INPUT: &'static str = include_str!("../../day5_input.txt");

fn main() {
	let mut input = INPUT.chars().filter(|c| c.is_alphabetic()).collect_vec();

	let units = (0..26).map(|x| (x + 'a' as u8) as char);

	let mut bestLen = ::std::usize::MAX;

	for unit in units {
		let mut input = input.iter().filter(|&&c| {
			c.to_ascii_lowercase() != unit
		}).map(|&c| c).collect_vec();

		let mut i = 0;

		while i < input.len() - 1 {
			let leftC = input[i];
			let rightC = input[i + 1];

			let distance = (leftC as i32 - rightC as i32).abs();

			if distance == 32 {
				input.remove(i);
				input.remove(i);

				if i > 0 {
					i -= 1;
				}
				continue;
			}

			i += 1;
		}

		let len = input.len();

		if len < bestLen {
			bestLen = len;
		}
	}
	println!("{}", bestLen);
}