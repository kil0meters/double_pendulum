use rand::Rng;
use std::f64::consts::PI;

use ppm::PPM;

mod ppm;

// SAFETY: it's safe trust me
static mut L1: f64 = 1.1;
static mut L2: f64 = 1.1;
static mut M1: f64 = 1.1;
static mut M2: f64 = 1.1;
static mut G: f64 = 1.1;

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
    l1: f64,  // Length of 1st pendulum
    l2: f64,  // Length of 2nd pendulum
    a1: f64,  // Angle of 1st pendulum
    a1v: f64, // Velocity of 1st pendulum
    a2: f64,  // Angle of 2nd pendulum with respect to 1st
    a2v: f64, // Velocity of 2nd pendulum
    m1: f64,  // Mass of 1st pendulum
    m2: f64,  // Mass of 2nd pendulum
    g: f64,   // Force of gravity
}

const SPEED: f64 = 0.05;

impl DoublePendulum {
    fn new() -> Self {
        unsafe {
            Self {
                l1: L1,
                l2: L2,

                a1: 0.0,
                a1v: 0.0,
                a2: 0.0,
                a2v: 0.0,

                m1: M1,
                m2: M2,
                g: G,
            }
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
        let a1a = self.dda1();
        let a2a = self.dda2();

        self.a1v += a1a * SPEED;
        self.a2v += a2a * SPEED;

        self.a1 += self.a1v * SPEED;
        self.a2 += self.a2v * SPEED;
    }

    fn get_position(&self) -> (Position, Position) {
        let x1 = self.l1 * self.a1.sin();
        let y1 = -self.l1 * self.a1.cos();

        let x2 = x1 + self.l2 * self.a2.sin();
        let y2 = y1 - self.l2 * self.a2.cos();

        return (Position { x: x1, y: y1 }, Position { x: x2, y: y2 });
    }
}

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn interp(x: f64, y: f64, part: f64) -> f64 {
    x + (y - x) * part
}

fn wow(x: usize, y: usize) -> (u8, u8, u8) {
    let mut double_pendulum = DoublePendulum::new();
    double_pendulum.a1 = interp(-PI / 2.0, PI / 2.0, x as f64 / WIDTH as f64);
    double_pendulum.a2 = interp(-PI / 2.0, PI / 2.0, y as f64 / HEIGHT as f64);

    let (initial_p1, initial_p2) = double_pendulum.get_position();

    // let initial_a1 = double_pendulum.a1;
    // let initial_a2 = double_pendulum.a2;

    for i in 0..(25555) {
        double_pendulum.advance();

        let (p1, p2) = double_pendulum.get_position();

        if p1.similar_to(&initial_p1, 0.1) && p2.similar_to(&initial_p2, 0.1) && i > 5 {
            let r = (i * 8) & 0xff;
            let g = (i * 9) & 0xff;
            let b = (0xff - (i * 4)) & 0xff;

            return (r as u8, g as u8, b as u8);
        }
    }

    return (0, 0, 0);
}

fn main() {
    let mut rng = rand::thread_rng();

    for i in 0..100 {
        println!("Generating image {}", i);

        // SAFETY: This is safe because these variables are only ever modified
        // when there are no other threads running. Threads are only creating
        // during the execution of the synchronous `color_with` runction.
        unsafe {
            L1 = rng.gen_range(0.0..100.0);
            L2 = rng.gen_range(0.0..100.0);
            M1 = rng.gen_range(0.0..100.0);
            M2 = rng.gen_range(0.0..100.0);
            G = rng.gen_range(0.0..20.0);

            let mut image = PPM::new(WIDTH, HEIGHT);

            image.color_with(wow);
            image
                .write(&format!(
                    "./out/{} L1={} L2={} M1={} M2={} G={}.ppm",
                    i, L1, L2, M1, M2, G
                ))
                .unwrap();
        }
    }
}
