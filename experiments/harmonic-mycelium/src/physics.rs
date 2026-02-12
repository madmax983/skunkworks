use macroquad::prelude::*;
use std::collections::VecDeque;

pub const G: f32 = 1000.0; // Gravitational constant (scaled for screen space)
const TRAIL_LENGTH: usize = 200;

#[derive(Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub trail: VecDeque<Vec2>,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            mass,
            radius,
            color,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }
}

pub fn update(bodies: &mut [Body], dt: f32) {
    let mut accelerations = vec![Vec2::ZERO; bodies.len()];

    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i == j { continue; }

            let r = bodies[j].pos - bodies[i].pos;
            let dist_sq = r.length_squared();
            let dist = dist_sq.sqrt();

            // Softening to avoid singularity
            if dist < bodies[i].radius + bodies[j].radius { continue; }

            let f = (G * bodies[j].mass) / dist_sq;
            let dir = r / dist;

            accelerations[i] += dir * f;
        }
    }

    for (i, body) in bodies.iter_mut().enumerate() {
        if i == 0 { continue; } // Star is fixed

        body.vel += accelerations[i] * dt;
        body.pos += body.vel * dt;

        if body.trail.len() >= TRAIL_LENGTH {
            body.trail.pop_front();
        }
        body.trail.push_back(body.pos);
    }
}
