#[macro_use]
extern crate lazy_static;
extern crate advent_of_code;

use itertools::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../day4_input.txt");

lazy_static! {
    static ref BEGIN_REGEX: Regex =
        { Regex::new("\\[(:?.+) (:?[0-9]+):(:?[0-9]+)\\] (:?.*)").unwrap() };
}

#[derive(Debug, Clone)]
enum Msg {
    WakesUp,
    FallsAsleep,
    Begin(String),
}

fn main() {
    let mut inputs = INPUT
        .lines()
        .map(|line| {
            let caps = BEGIN_REGEX.captures(line).unwrap();
            let date = caps[1].to_string();
            let hour = caps[2].parse::<u32>().unwrap();
            let minute = caps[3].parse::<u32>().unwrap();
            let message = caps[4].to_string();

            let mut msgs = message.split(' ');

            let msg = match msgs.nth(1).unwrap().trim() {
                "asleep" => Msg::FallsAsleep,
                "up" => Msg::WakesUp,
                id => Msg::Begin(id.to_string()),
            };

            (date, hour, minute, msg)
        })
        .sorted_by(|a, b| match String::cmp(&a.0, &b.0) {
            Ordering::Equal => match u32::cmp(&a.1, &b.1) {
                Ordering::Equal => u32::cmp(&a.2, &b.2),
                o => o,
            },
            o => o,
        });

    let mut guard_map = HashMap::<_, HashMap<_, _>>::new();
    let mut current_guard = String::new();
    let mut last_action = (0u32, 0u32);

    for input in inputs {
        //println!("{}:{} {:?}", input.1, input.2, input.3);
        match input.3 {
            Msg::Begin(id) => {
                current_guard = id.clone();
                guard_map.entry(id.clone()).or_default();
            }
            Msg::FallsAsleep => {
                last_action = (input.1, input.2);
                if input.1 == 23 {
                    println!("{} falls asleep at {}:{}", input.0, input.1, input.2);
                }
            }
            Msg::WakesUp => {
                guard_map
                    .entry(current_guard.clone())
                    .and_modify(|g: &mut HashMap<_, _>| {
                        for h in last_action.0..=input.1 {
                            for m in last_action.1..input.2 {
                                *g.entry((h, m)).or_insert(1) += 1;
                            }
                        }
                    });
            }
        }
    }

    //println!("{:?}", guard_map);

    let results = guard_map
        .iter()
        .map(|(k, v)| {
            (
                k,
                v.values().sum::<u32>(),
                v.iter().sorted_by(|(_, m1), (_, m2)| u32::cmp(m2, m1)),
            )
        })
        .sorted_by(|(_, _, a), (_, _, b)| {
            u32::cmp(&b.get(0).map(|v| *v.1).unwrap_or(Default::default()),
                     &a.get(0).map(|v| *v.1).unwrap_or(Default::default()))
        });

    println!("{:?}", results[0]);
    println!("{:?}", results[1]);
    println!("{:?}", results[2]);
}
