#[macro_use]
extern crate itertools;

#[macro_use]
extern crate lazy_static;

use gif::{Encoder, Repeat, SetParameter};
use itertools::Itertools;
use rusttype::point;
use rusttype::Point;
use rusttype::Scale;
use rusttype::{Font, FontCollection};
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::ops::Index;

lazy_static! {
    pub static ref FONT: Font<'static> = {
        let data = include_bytes!("../fonts/Roboto-Regular.ttf");
        let collection = FontCollection::from_bytes(data as &[u8]).unwrap();
        collection.into_font().unwrap()
    };
}

#[derive(Copy, Clone)]
pub struct VisualizerOptions {
    info_height: u16,
}

impl Default for VisualizerOptions {
    fn default() -> Self {
        VisualizerOptions { info_height: 200 }
    }
}

pub struct Visualizer {
    encoder: Encoder<File>,
    width: u16,
    height: u16,
    options: VisualizerOptions,
    pub info_rects: Vec<((u16, u16), (u16, u16))>,
    pub pixels: Vec<u8>,
}

impl Visualizer {
    pub fn new(
        file: File,
        width: u16,
        height: u16,
        pallete: &[u8],
        options: VisualizerOptions,
    ) -> Result<Self, Box<Error>> {
        let height = height + options.info_height;

        let mut vis = Self {
            encoder: Encoder::new(file, width, height, pallete)?,
            width,
            height,
            pixels: vec![0u8; width as usize * (height) as usize],
            info_rects: Vec::new(),
            options,
        };

        vis.clear_rect(((0, height - options.info_height), (width, 2)), 2);
        vis.clear_rect(
            (
                (0, height - options.info_height + 2),
                (width, options.info_height - 2),
            ),
            1,
        );
        Ok(vis)
    }

    #[inline(always)]
    pub fn pixel(&self, x: u16, y: u16) -> u8 {
        let (x, y) = (x as usize, y as usize);
        self.pixels[y * self.width as usize + x]
    }

    #[inline(always)]
    pub fn pixel_mut(&mut self, x: u16, y: u16) -> &mut u8 {
        let (x, y, width) = (x as usize, y as usize, self.width as usize);
        unsafe { self.pixels.get_unchecked_mut(y * width + x) }
    }

    #[inline(always)]
    pub fn pixel_set(&mut self, x: u16, y: u16, pixel: u8) {
        *self.pixel_mut(x, y) = pixel;
    }

    pub fn clear_rect(&mut self, ((x, y), (w, h)): ((u16, u16), (u16, u16)), color: u8) {
        for j in y..y + h {
            for i in x..x + w {
                self.pixel_set(i, j, color);
            }
        }
    }

    pub fn clear_info(&mut self) {
        while let Some(rect) = self.info_rects.pop() {
            self.clear_rect(rect, 0);
        }
    }

    pub fn end_frame(&mut self) {
        let mut frame = gif::Frame::default();
        frame.width = self.width;
        frame.height = self.height;
        frame.buffer = Cow::Borrowed(&*self.pixels);
        self.encoder.write_frame(&frame).unwrap();
    }

    pub fn text(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        color: u8,
        scale: f32,
    ) -> ((u16, u16), (u16, u16)) {
        //println!("Drawing {}", text);
        let y = y + (self.height - self.options.info_height) + 2;

        let scale = Scale::uniform(scale);
        let metrics = FONT.v_metrics(scale);

        //println!("matrics ascent {}", metrics.ascent);

        let glyphs: Vec<_> = FONT
            .layout(text, scale, point(x as f32, y as f32 + metrics.ascent))
            .collect_vec();

        let glyphs_height = (metrics.ascent - metrics.descent).ceil() as u32;

        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();

            (max_x - min_x) as u32
        };

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|rx, ry, v| {
                    if v > 0.5 {
                        let x = (rx + bounding_box.min.x as u32) as u16;
                        let y = (ry + bounding_box.min.y as u32) as u16;

                        self.pixel_set(x, y, color);
                    }
                });
            }
        }
        self.info_rects
            .push(((x, y), (glyphs_width as u16, glyphs_height as u16)));
        ((x, y), (glyphs_width as u16, glyphs_height as u16))
    }
}
