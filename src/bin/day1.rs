use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../freq_deltas.txt");

fn main() {
    let input: Vec<i64> = INPUT
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    println!("Sum {}", input.iter().sum::<i64>());

    let mut cache = HashSet::new();
    cache.insert(0);

    let result = input
        .iter()
        .cycle()
        .scan(0, |frequency, num| {
            *frequency += num;
            Some(*frequency)
        })
        .find(|f| !cache.insert(*f));

    println!("Repeating: {:?}", result);
}
