#[macro_use]
extern crate lazy_static;

use regex::Regex;
use itertools::Itertools;
use std::collections::{HashMap, BinaryHeap };
use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../day7_input.txt");

lazy_static! {
	static ref STEP_REGEX: Regex = {
		Regex::new("Step (:?[A-Z]) must be finished before step (:?[A-Z]) can begin.").unwrap()
	};
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
struct Value {
	c: char
}

impl Ord for Value {
	fn cmp(&self, other: &Self) -> Ordering {
		other.c.cmp(&self.c)
	}
}

impl PartialOrd<Value> for Value {
	fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Default, Debug, Clone)]
struct Worker {
	current_value: Option<Value>,
	time_left: usize
}

fn main() {
	let mut triggers = HashSet::new();

	let mut run_map: HashMap<char, (usize, Vec<char>)> = INPUT
		.lines()
		.filter_map(|line| {
			let matches = STEP_REGEX.captures(line)?;

			let trigger = matches[1].chars().next().unwrap();
			triggers.insert(trigger);

			let value = matches[2].chars().next().unwrap();

			Some((value, trigger))
		})
		.into_group_map()
		.into_iter()
		.map(|a| { (a.0, (0, a.1)) })
		.collect::<HashMap<_, _>>();

	let mut call_stack = BinaryHeap::new();

	{
		let keys = run_map.keys().map(|c| *c).collect::<HashSet<_>>();
		for trigger in triggers {
			if keys.contains(&trigger) {
				continue;
			}
			call_stack.push(Value {c: trigger});
		}
	}

	let mut workers = vec![Worker::default(); 5];
	let mut time = 0;

	let setup_time = 60;

	loop {
		let mut stalled = true;

		for (i, worker) in &mut workers.iter_mut().enumerate() {
			if let Some(worker_value) = worker.current_value.clone() {
				if worker.time_left == 0 {
					for (key, (trig_count, values)) in run_map.iter_mut() {
						if values.contains(&worker_value.c) {
							*trig_count += 1;

							if *trig_count == values.len() {
								call_stack.push(Value { c: *key });
							}
						}
					}

					if let Some(value) = call_stack.pop() {
						stalled = false;
						worker.current_value = Some(value.clone());
						worker.time_left = setup_time + (value.c as usize - 'A' as usize );
					} else {
						worker.current_value = None;
					}
				} else {
					stalled = false;
					worker.time_left -= 1;
				}
			} else {
				if let Some(value) = call_stack.pop() {
					stalled = false;
					worker.current_value = Some(value.clone());
					//println!("{}", value.c);
					worker.time_left = setup_time + (value.c as usize - 'A' as usize);
				}
			}
		}
		if stalled {
			break;
		}

		time += 1;
	}

	println!("Total Time: {}", time - 1);
}

