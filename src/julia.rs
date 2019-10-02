use crate::color;
use crate::window;
use rayon::prelude::*;
use std::time::Instant;

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

pub struct Julia {
    pub pos: Pos,
    pub iter: usize,
    pub zoom: f64,
}

impl Julia {
    pub fn new(x: f64, y: f64, iter: usize, zoom: f64) -> Self {
        Julia {
            pos: Pos { x, y },
            iter,
            zoom,
        }
    }

    pub fn compute(&self, window: &mut window::Window) {
        let width = window.width();

        let x1 = self.pos.x;
        let y1 = self.pos.y;

        let now = Instant::now();

        window
            .buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, val)| {
                let x = index % width;
                let y = index / width;

                const c_r: f64 = -0.7;
                const c_i: f64 = 0.27015;
                let mut z_r = x as f64 / self.zoom + x1;
                let mut z_i = y as f64 / self.zoom + y1;
                let mut i = 0;

                while (z_r * z_r + z_i * z_i < 4.0) && i < self.iter {
                    let tmp = z_r;
                    z_r = z_r * z_r - z_i * z_i + c_r;
                    z_i = 2.0 * z_i * tmp + c_i;
                    i += 1;
                }

                if i == self.iter {
                    *val = 0x0000_0000;
                } else {
                    *val = color::hue_to_rgb(
                        i as f32 * (360.0 / self.iter as f32),
                        1.0,
                        i as f32 / self.iter as f32,
                    );
                }
            });

        let duration = now.elapsed();
        println!(
            "{:.5} seconds for whatever you did.",
            duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
        );
    }
}
