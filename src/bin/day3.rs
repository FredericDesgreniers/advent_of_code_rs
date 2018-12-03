#[macro_use]
extern crate lazy_static;

use itertools::*;
use regex::Regex;

const INPUT: &'static str = include_str!("../../day3_input.txt");

lazy_static! {
    pub static ref parser: Regex =
        { Regex::new("#(:?[0-9]+) @ (:?[0-9]+),(:?[0-9]+): (:?[0-9]+)x(:?[0-9]+)").unwrap() };
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Filled,
    Intersected,
}

fn main() {
    let mut tiles = vec![Tile::Empty; 1001 * 1001];

    let input = INPUT
        .lines()
        .map(|line| {
            let caps = parser.captures(line).unwrap();

            let id = caps[1].to_string();

            let pos: (usize, usize) = (caps[2].parse().unwrap(), caps[3].parse().unwrap());
            let dim: (usize, usize) = (caps[4].parse().unwrap(), caps[5].parse().unwrap());

            (id, pos, (pos.0 + dim.0, pos.1 + dim.1))
        })
        .collect::<Vec<_>>();

    let mut overlap = 0;

    for (_, l, r) in input.iter() {
        for (i, j) in iproduct!((l.0..r.0), (l.1..r.1)) {
            let tile = &mut tiles[i*1001 + j];

            match *tile {
                Tile::Empty => *tile = Tile::Filled,
                Tile::Filled => {
                    *tile = Tile::Intersected;
                    overlap += 1;
                }
                _ => (),
            }
        }
    }

    println!("Overlap of {} tiles", overlap);

    'main: for (id, l, r) in input.iter() {
        for (i, j) in iproduct!((l.0..r.0), (l.1..r.1)) {
            let tile = tiles[i*1001 + j];

            match tile {
                Tile::Intersected => {
                    continue 'main;
                }
                _ => (),
            }
        }
        println!("{} did not intersect", id);
    }
}
