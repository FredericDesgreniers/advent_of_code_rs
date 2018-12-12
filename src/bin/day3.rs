#[macro_use]
extern crate lazy_static;
extern crate advent_of_code;

use gif::Encoder;
use itertools::*;
use rand::Rng;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::File;

use advent_of_code::Visualizer;
use std::cmp;

const INPUT: &'static str = include_str!("../../day3_input.txt");

const SIZE: u16 = 1001;

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

            let pos: (u16, u16) = (caps[2].parse().unwrap(), caps[3].parse().unwrap());
            let dim: (u16, u16) = (caps[4].parse().unwrap(), caps[5].parse().unwrap());

            (id, pos, (pos.0 + dim.0, pos.1 + dim.1))
        })
        .collect_vec();

    let mut image = File::create("day3_timelapse.gif").unwrap();

    let mut pallete = vec![0; 256 * 3];

    let mut rng = rand::thread_rng();

    rng.fill(&mut pallete[..]);

    pallete[0] = 0xFF;
    pallete[1] = 0xFF;
    pallete[2] = 0xFF;

    pallete[3] = 0;
    pallete[4] = 0;
    pallete[5] = 0;

    pallete[6] = 0xcc;
    pallete[7] = 0;
    pallete[8] = 0;

    pallete[9] = 0;
    pallete[10] = 0x33;
    pallete[11] = 0;

    let mut visualizer = Visualizer::new(
        image,
        SIZE as u16,
        SIZE as u16,
        &pallete,
        Default::default(),
    ).unwrap();

    let mut tiles = vec![Tile::Empty; SIZE as usize * SIZE as usize];

    let mut overlap = 0;
    let mut total_claimed = 0;

    let mut no_overlap = HashSet::new();

    let progress_pos = visualizer.text(10, 50, "progress", 0, 32.0);

    let overlap_pos = visualizer.text(10, (progress_pos.1).1 + 100, "overlap", 0, 32.0);

    let info_text_width = cmp::max(
        (progress_pos.0).0 + (progress_pos.1).0,
        (overlap_pos.0).1 + (overlap_pos.1).0,
    );

    let claim_num = input.len();

    for (id, l, r) in input.into_iter() {
        no_overlap.insert(id);

        let color_id = (id % 252 + 4) as u8;

        let mut update_info = false;

        for (i, j) in iproduct!((l.0..r.0), (l.1..r.1)) {
            let tile = &mut tiles[i as usize * (SIZE as usize) + j as usize];
            let pixel = visualizer.pixel_mut(i, j);

            total_claimed += 1;

            match *tile {
                Tile::Empty => {
                    *tile = Tile::Filled(id);
                    *pixel = color_id;
                }
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

        {
            let progress_percent = id as f32 / claim_num as f32;

            let start_x = info_text_width;
            let start_y = (progress_pos.0).1;

            let width = 500;
            let height = (progress_pos.1).1;

            let middle_width = (width as f32 * progress_percent) as u16;

            visualizer.clear_rect(((start_x, start_y), ((middle_width) as u16, (height))), 2);

            visualizer.clear_rect(
                (
                    (start_x + middle_width, start_y),
                    ((width - middle_width) as u16, (height)),
                ),
                3,
            );

            visualizer.text(
                start_x + 100,
                50,
                &format!(
                    "{} / {} - {}%",
                    id,
                    claim_num,
                    (100.0 * progress_percent) as u32
                ),
                0,
                32.0,
            );
        }

        {
            let overlap_percent = overlap as f32 / total_claimed as f32;

            let start_x = info_text_width;
            let start_y = (overlap_pos.0).1;

            let width = 500;
            let height = (overlap_pos.1).1;

            let middle_width = (width as f32 * overlap_percent) as u16;

            visualizer.clear_rect(((start_x, start_y), ((middle_width) as u16, (height))), 2);

            visualizer.clear_rect(
                (
                    (start_x + middle_width, start_y),
                    ((width - middle_width) as u16, (height)),
                ),
                3,
            );

            visualizer.text(
                start_x + 100,
                (progress_pos.1).1 + 100,
                &format!(
                    "{} / {} - {}%",
                    overlap,
                    total_claimed,
                    (100.0 * overlap_percent) as u32
                ),
                0,
                32.0,
            );
        }

        visualizer.end_frame();

        println!("frame {} done", id);
    }

    println!("Overlap of {} tiles", overlap);
    println!("No Overlap: {:?}", no_overlap);
}
