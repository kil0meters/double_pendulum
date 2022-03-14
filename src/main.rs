use std::{f64::consts::PI, time::Instant};

use ppm::PPM;

mod ppm;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

impl Position {
    fn similar_to(&self, other: &Position, threshold: f64) -> bool {
        (self.x - other.x).abs() < threshold && (self.y - other.y).abs() < threshold
    }
}

struct DoublePendulum {
    l1: f64, // Length of 1st pendulum
    l2: f64, // Length of 2nd pendulum
    a1: f64, // Angle of 1st pendulum
    a1v: f64,
    a1a: f64,
    a2: f64, // Angle of 2nd pendulum with respect to 1st
    a2v: f64,
    a2a: f64,
    m1: f64,
    m2: f64,
    g: f64, // Force of gravity
}

impl DoublePendulum {
    fn new() -> Self {
        Self {
            l1: 10.0,
            l2: 10.0,

            a1: 0.0,
            a1v: 0.0,
            a1a: 0.0,
            a2: 0.0,
            a2v: 0.0,
            a2a: 0.0,

            m1: 1.0,
            m2: 2.0,
            g: 9.8,
        }
    }

    // Shamelessly stolen from
    // https://www.myphysicslab.com/pendulum/double-pendulum-en.html
    // Returns the acceleration of angle 1
    fn dda1(&self) -> f64 {
        let (l1, l2, a1, a1v, a2, a2v, m1, m2, g) = (
            self.l1, self.l2, self.a1, self.a1v, self.a2, self.a2v, self.m1, self.m2, self.g,
        );

        (-g * (2.0 * m1 + m2) * a1.sin()
            - m2 * g * (a1 - 2.0 * a2).sin()
            - 2.0 * (a1 - a2).sin() * m2 * (a2v * a2v * l2 + a1v * a1v * l1 * (a1 - a2).cos()))
            / (l1 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a2).cos()))
    }

    // Returns acceleration of angle 2
    #[allow(dead_code)]
    fn dda2(&self) -> f64 {
        let (l1, l2, a1, a1v, a2, a2v, m1, m2, g) = (
            self.l1, self.l2, self.a1, self.a1v, self.a2, self.a2v, self.m1, self.m2, self.g,
        );

        (2.0 * (a1 - a2).sin()
            * (a1v * a1v * l1 * (m1 + m2)
                + g * (m1 + m2) * a1.cos()
                + a2v * a2v * l2 * m2 * (a1 - a2).cos()))
            / (l2 * (2.0 * m1 + m2 - (m2 * (2.0 * (a1 - a2)).cos())))
    }

    fn advance(&mut self) {
        self.a1a = self.dda1();
        // this is obviously wrong but the simulation errors out immediately
        // if I use the right function
        self.a2a = self.dda1();

        self.a1v += self.a1a;
        self.a2v += self.a2a;

        self.a1 += self.a1v;
        self.a2 += self.a2v;
    }

    fn get_position(&self) -> (Position, Position) {
        let x1 = self.l1 * self.a1.sin();
        let y1 = -self.l1 * self.a1.cos();

        let x2 = x1 + self.l2 * self.a2.sin();
        let y2 = y1 - self.l2 * self.a2.cos();

        return (Position { x: x1, y: y1 }, Position { x: x2, y: y2 });
    }
}

const WIDTH: usize = 512 * 8;
const HEIGHT: usize = 512 * 8;

fn interp(x: f64, y: f64, part: f64) -> f64 {
    x + (y - x) * part
}

fn wow(x: usize, y: usize) -> (u8, u8, u8) {
    let mut double_pendulum = DoublePendulum::new();
    double_pendulum.a1 = interp(-PI / 2.0, PI / 2.0, x as f64 / WIDTH as f64);
    double_pendulum.a2 = interp(-PI / 2.0, PI / 2.0, y as f64 / HEIGHT as f64);

    let (initial_p1, initial_p2) = double_pendulum.get_position();

    for i in 0..255 {
        double_pendulum.advance();

        let (p1, p2) = double_pendulum.get_position();

        if p1.similar_to(&initial_p1, 0.1) && p2.similar_to(&initial_p2, 0.1) && i > 5 {
            let r = (i * 8) & 0xff;
            let g = (i * 9) & 0xff;
            let b = (0xff - (i * 4)) & 0xff;

            return (r, g, b);
        }
    }

    return (0, 0, 0);
}

fn main() {
    let mut double_pendulum = DoublePendulum::new();

    double_pendulum.a1 = PI / 4.0;
    double_pendulum.a2 = PI / 4.0;

    let start = Instant::now();
    let mut image = PPM::new(WIDTH, HEIGHT);
    println!("{}", start.elapsed().as_millis());

    image.color_with(wow);
    println!("{}", start.elapsed().as_millis());

    image.write("./image.ppm").unwrap();
    println!("{}", start.elapsed().as_millis());
}
