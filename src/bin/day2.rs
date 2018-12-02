use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../day2_input.txt");

fn main() {
    checksum();
    find_similar();
}

pub fn checksum() {
    let result = INPUT.lines().fold((0, 0), |counts, line| {
        let mut frequencies = HashMap::<char, u32>::new();

        line.chars().for_each(|c| {
            *frequencies.entry(c).or_default() += 1;
        });

        (
            counts.0 + frequencies.values().any(|&n| n == 3) as u32,
            counts.1 + frequencies.values().any(|&n| n == 2) as u32,
        )
    });

    println!("checksum: {:?}", result.0 * result.1);
}

pub fn find_similar() {
    let r: Option<String> = INPUT
        .lines()
        .cartesian_product(INPUT.lines())
        .map(|(id1, id2): (&str, &str)| {
            (
                id1.chars()
                    .zip(id2.chars())
                    .fold(0, |d, (c1, c2)| d + (c1 != c2) as u32),
                id1,
                id2,
            )
        })
        .filter(|(d, _, _)| *d == 1)
        .map(|(_, id1, id2)| {
            id1.chars()
                .zip(id2.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        })
        .nth(0);

    println!("{:?}", r);
}
