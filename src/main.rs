use nannou::prelude::*;

use std::f32::consts::PI;

use ppm::PPM;

mod ppm;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn similar_to(&self, other: &Position, threshold: f32) -> bool {
        (self.x - other.x).abs() < threshold && (self.y - other.y).abs() < threshold
    }
}

struct DoublePendulum {
    l1: f32,  // Length of 1st pendulum
    l2: f32,  // Length of 2nd pendulum
    a1: f32,  // Angle of 1st pendulum
    a1v: f32, // Velocity of 1st pendulum
    a1a: f32, // Acceleration of 1st pendulum
    a2: f32,  // Angle of 2nd pendulum with respect to 1st
    a2v: f32, // Velocity of 2nd pendulum
    a2a: f32, // Acceleration of 2nd pendulum
    m1: f32,  // Mass of 1st pendulum
    m2: f32,  // Mass of 2nd pendulum
    g: f32,   // Force of gravity
}

impl DoublePendulum {
    fn new(a1: f32, a2: f32) -> Self {
        Self {
            l1: 100.0,
            l2: 100.0,

            a1,
            a1v: 0.0,
            a1a: 0.0,
            a2,
            a2v: 0.0,
            a2a: 0.0,

            m1: 2.0,
            m2: 2.0,
            g: 9.8,
        }
    }

    // Shamelessly stolen from
    // https://www.myphysicslab.com/pendulum/double-pendulum-en.html
    // Returns the acceleration of angle 1
    fn dda1(&self) -> f32 {
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
    fn dda2(&self) -> f32 {
        let (l1, l2, a1, a1v, a2, a2v, m1, m2, g) = (
            self.l1, self.l2, self.a1, self.a1v, self.a2, self.a2v, self.m1, self.m2, self.g,
        );

        (2.0 * (a1 - a2).sin()
            * (a1v * a1v * l1 * (m1 + m2)
                + g * (m1 + m2) * a1.cos()
                + a2v * a2v * l2 * m2 * (a1 - a2).cos()))
            / (l2 * (2.0 * m1 + m2 - (m2 * (2.0 * (a1 - a2)).cos())))
    }

    fn advance(&mut self, amount: f32) {
        self.a1a = self.dda1();
        self.a2a = self.dda2();

        self.a1v += self.a1a * amount;
        self.a2v += self.a2a * amount;

        self.a1 += self.a1v * amount;
        self.a2 += self.a2v * amount;
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

fn interp(x: f32, y: f32, part: f32) -> f32 {
    x + (y - x) * part
}

// fn wow(x: usize, y: usize) -> (u8, u8, u8) {
//     let mut double_pendulum = DoublePendulum::new();
//     double_pendulum.a1 = interp(-PI / 2.0, PI / 2.0, x as f32 / WIDTH as f32);
//     double_pendulum.a2 = interp(-PI / 2.0, PI / 2.0, y as f32 / HEIGHT as f32);

//     let (initial_p1, initial_p2) = double_pendulum.get_position();

//     for i in 0..255 {
//         double_pendulum.advance();

//         let (p1, p2) = double_pendulum.get_position();

//         if p1.similar_to(&initial_p1, 0.1) && p2.similar_to(&initial_p2, 0.1) && i > 5 {
//             let r = (i * 8) & 0xff;
//             let g = (i * 9) & 0xff;
//             let b = (0xff - (i * 4)) & 0xff;

//             return (r, g, b);
//         }
//     }

//     return (0, 0, 0);
// }

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // model.pendulum.advance(0.-01);
    model.pendulum.advance(0.05);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let (p1, p2) = model.pendulum.get_position();

    draw.line()
        .start(Vec2::ZERO)
        .end(Vec2::new(p1.x, p1.y))
        .stroke_weight(4.0)
        .color(WHITE);
    draw.line()
        .start(Vec2::new(p1.x, p1.y))
        .end(Vec2::new(p2.x, p2.y))
        .stroke_weight(4.0)
        .color(WHITE);

    draw.ellipse()
        .color(ORANGE)
        .x(p1.x)
        .y(p1.y)
        .radius(model.pendulum.m1 * 4.0);
    draw.ellipse()
        .color(ORANGE)
        .x(p2.x)
        .y(p2.y)
        .radius(model.pendulum.m2 * 4.0);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    pendulum: DoublePendulum,
}

fn model(_app: &App) -> Model {
    Model {
        pendulum: DoublePendulum::new(-PI / 2.0, -PI / 4.0),
    }
}
