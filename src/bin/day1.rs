use std::collections::BTreeSet;

const INPUT: &'static str = include_str!("../../freq_deltas.txt");

fn main() {
    let input: Result<Vec<i64>, _> = INPUT.lines().map(str::parse).collect();
    let input = input.unwrap();

    println!("Sum {}", input.iter().sum::<i64>());

    let mut cache = BTreeSet::new();
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
