use macroquad::prelude::*;
use std::collections::VecDeque;

pub const G: f32 = 5000.0; // Increased G for stronger gravity in bridge context
const TRAIL_LENGTH: usize = 200;

#[derive(Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
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
            acc: Vec2::ZERO,
            mass,
            radius,
            color,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }
}

/// Updates the physical state of the bodies in the system.
/// Uses an inline accumulator and an `acc` field on the `Body` struct to
/// avoid a per-frame `vec![Vec2::ZERO; n]` allocation in the hot path.
pub fn update(bodies: &mut [Body], dt: f32) {
    for i in 1..bodies.len() {
        let mut acc = Vec2::ZERO;
        let pos_i = bodies[i].pos;
        let radius_i = bodies[i].radius;

        for (j, body_j) in bodies.iter().enumerate() {
            if i == j {
                continue;
            }

            let r = body_j.pos - pos_i;
            let dist_sq = r.length_squared();

            // Softening
            if dist_sq < (radius_i + body_j.radius) * (radius_i + body_j.radius) {
                continue;
            }

            let dist = dist_sq.sqrt();
            let f = (G * body_j.mass) / (dist_sq * dist);
            acc += r * f;
        }
        bodies[i].acc = acc;
    }

    for body in bodies.iter_mut().skip(1) {
        body.vel += body.acc * dt;
        body.pos += body.vel * dt;

        if body.trail.len() >= TRAIL_LENGTH {
            body.trail.pop_front();
        }
        body.trail.push_back(body.pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_update() {
        let mut bodies = vec![
            Body::new(Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, WHITE),
            Body::new(Vec2::new(100.0, 0.0), Vec2::ZERO, 10.0, 5.0, WHITE),
        ];

        // Run update
        update(&mut bodies, 0.1);

        // Body 1 should have moved towards Body 0 (negative x)
        // a = G * M / r^2 = 5000 * 1000 / 100^2 = 5000000 / 10000 = 500.
        // v = a * dt = 500 * 0.1 = 50.
        // p = v * dt = 50 * 0.1 = 5.
        // New pos x should be 100 - 5 = 95.
        // Wait, update does:
        // body.vel += acc * dt;
        // body.pos += body.vel * dt;

        // Let's check if it moved left
        assert!(bodies[1].pos.x < 100.0);
    }
}
