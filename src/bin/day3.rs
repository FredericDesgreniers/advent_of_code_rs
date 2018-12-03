#[macro_use]
extern crate lazy_static;

use itertools::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use gif::Encoder;
use std::borrow::Cow;
use rand::Rng;

const INPUT: &'static str = include_str!("../../day3_input.txt");

const SIZE: usize = 1001;

lazy_static! {
    pub static ref parser: Regex =
        { Regex::new("#(:?[0-9]+) @ (:?[0-9]+),(:?[0-9]+): (:?[0-9]+)x(:?[0-9]+)").unwrap() };
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Filled(u32),
    Intersected,
}

fn main() {

    let input = INPUT
        .lines()
        .map(|line| {
            let caps = parser.captures(line).unwrap();

            let id = caps[1].to_string().parse::<u32>().unwrap();

            let pos: (usize, usize) = (caps[2].parse().unwrap(), caps[3].parse().unwrap());
            let dim: (usize, usize) = (caps[4].parse().unwrap(), caps[5].parse().unwrap());

            (id, pos, (pos.0 + dim.0, pos.1 + dim.1))
        })
        .collect_vec();

    let mut image = File::create("day3_timelapse.gif").unwrap();

    let mut pallete = vec![0; 256*3];

    let mut rng = rand::thread_rng();

	//rng.fill(&mut pallete);
    rng.fill(&mut pallete[..]);

    pallete[0] = 0xFF;
    pallete[1] = 0xFF;
    pallete[2] = 0xFF;

    pallete[3] = 0;
    pallete[4] = 0;
    pallete[5] = 0;

    let mut encoder = Encoder::new(&mut image, 1001, 1001, &pallete).unwrap();

    let mut tiles = vec![Tile::Empty; SIZE * SIZE];
    let mut pixels = vec![0u8; SIZE*SIZE];



    let mut overlap = 0;
    let mut no_overlap = HashSet::new();

    for (id, l, r) in input.into_iter() {
        no_overlap.insert(id);

        let color_id = (id % 254 + 2) as u8;

        for (i, j) in iproduct!((l.0..r.0), (l.1..r.1)) {
            let tile= &mut tiles[i*SIZE+j];
            let pixel= &mut pixels[i*SIZE+j];

            match *tile {
                Tile::Empty => {
                    *tile = Tile::Filled(id);
                    *pixel = color_id;
                },
                Tile::Filled(last_claim) => {
                    *tile = Tile::Intersected;

                    no_overlap.remove(&id);
					no_overlap.remove(&last_claim);

                    overlap += 1;
                    *pixel = 1;
                }
                _ => {
                    no_overlap.remove(&id);
                }
            }

        }

        let mut frame = gif::Frame::default();
        frame.width = SIZE as u16;
        frame.height = SIZE as u16;
        frame.buffer = Cow::Borrowed(&*pixels);

        encoder.write_frame(&frame).unwrap();
    }



    println!("Overlap of {} tiles", overlap);
    println!("No Overlap: {:?}", no_overlap);
}
