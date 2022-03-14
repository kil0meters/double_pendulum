use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::result::Result;
use std::thread;

const THREAD_COUNT: usize = 8;

pub struct PPM {
    width: usize,
    height: usize,
    buffer: Vec<Vec<(u8, u8, u8)>>,
}

impl PPM {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![(0, 0, 0); height]; width],
        }
    }

    pub fn color_with(&mut self, color_function: fn(usize, usize) -> (u8, u8, u8)) {
        let mut threads = Vec::new();

        for t in 0..THREAD_COUNT {
            let size = self.width / THREAD_COUNT;
            let mut vecs = vec![vec![(0, 0, 0); self.height]; size];
            let height = self.height;

            threads.push(thread::spawn(move || {
                for x in 0..size {
                    for y in 0..height {
                        vecs[x][y] = color_function(x + t * size, y);
                    }
                }
                return vecs;
            }));
        }

        let mut t = 0;
        let size = self.width / THREAD_COUNT;
        for thread in threads {
            let vecs = thread.join().unwrap();

            for x in 0..size {
                self.buffer[x + t * size] = vecs[x].clone();
            }

            t += 1;
        }
    }

    pub fn write(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

        let mut contents = format!("P3\n{} {}\n255\n", self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.buffer[x][y];
                contents.push_str(&format!("{} {} {} ", pixel.0, pixel.1, pixel.2));
            }
        }

        file.write(contents.as_bytes())?;
        Ok(())
    }
}
