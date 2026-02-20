use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub u: f32,
    pub v: f32,
    pub facing: Vec2,
}

impl Point {
    pub fn new(u: f32, v: f32, facing: Vec2) -> Self {
        Self { u, v, facing }
    }
}

pub fn step(mut p: Point, delta: Vec2) -> Point {
    p.u += delta.x;
    p.v += delta.y;

    // Wrap logic
    let mut wrapped = true;
    let mut iterations = 0;
    while wrapped && iterations < 10 {
        wrapped = false;
        iterations += 1;

        if p.u > 1.0 {
            p.u -= 1.0;
            p.v = 1.0 - p.v;
            p.facing.y = -p.facing.y;
            wrapped = true;
        } else if p.u < 0.0 {
            p.u += 1.0;
            p.v = 1.0 - p.v;
            p.facing.y = -p.facing.y;
            wrapped = true;
        }

        if p.v > 1.0 {
            p.v -= 1.0;
            p.u = 1.0 - p.u;
            p.facing.x = -p.facing.x;
            wrapped = true;
        } else if p.v < 0.0 {
            p.v += 1.0;
            p.u = 1.0 - p.u;
            p.facing.x = -p.facing.x;
            wrapped = true;
        }
    }

    p
}
