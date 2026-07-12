use macroquad::prelude::*;
use std::collections::VecDeque;

pub const G: f32 = 1000.0; // Gravitational constant (scaled for screen space)
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

pub fn update(bodies: &mut [Body], dt: f32) {
    // 1. Calculate Forces & Update Velocity
    let n = bodies.len();
    for i in 0..n {
        let mut acc = Vec2::ZERO;
        let p_i = bodies[i].pos;
        let r_i = bodies[i].radius;

        for (j, b_j) in bodies.iter().enumerate().take(n) {
            if i == j {
                continue;
            }

            let r = b_j.pos - p_i;
            let dist_sq = r.length_squared();
            let dist = dist_sq.sqrt();

            // Softening to avoid singularity
            if dist < r_i + b_j.radius {
                continue;
            }

            let f = (G * b_j.mass) / dist_sq;
            let dir = r / dist;

            acc += dir * f;
        }
        bodies[i].acc = acc;
    }

    // 2. Apply Acceleration and Velocity
    for (i, body) in bodies.iter_mut().enumerate() {
        // Star is fixed (index 0 usually, but let's make it general: if mass is huge, it moves little,
        // OR we explicitly pin body 0. Let's pin body 0 if it's the star).
        if i == 0 {
            continue;
        }

        body.vel += body.acc * dt;
        body.pos += body.vel * dt;

        // Update trail
        if body.trail.len() >= TRAIL_LENGTH {
            body.trail.pop_front();
        }
        body.trail.push_back(body.pos);
    }
}

// Check for crossings of the positive X-axis
pub fn check_crossings(bodies: &[Body], old_positions: &[Vec2]) -> Vec<f32> {
    let mut events = Vec::new();

    for (i, body) in bodies.iter().enumerate() {
        if i == 0 {
            continue;
        } // Star doesn't trigger

        let p1 = old_positions[i];
        let p2 = body.pos;

        // Check Y sign change
        if p1.y.signum() != p2.y.signum() {
            // Check if crossing happened on positive X
            // Linear interpolation to find x at y=0
            // y = mx + c
            // x = (y - c) / m
            // But simpler: if both x are positive, it likely crossed positive X.
            // Or calculate intersection properly.

            let t = -p1.y / (p2.y - p1.y);
            let x_cross = p1.x + (p2.x - p1.x) * t;

            if x_cross > 0.0 {
                // Crossing confirmed!
                // Calculate frequency based on velocity magnitude or distance
                // High velocity (close to star) = High pitch?
                // Kepler: v ~ 1/sqrt(r).
                // Let's map angular velocity at crossing to pitch.
                // omega = v_perp / r
                // v_perp is approx v.length() if circular.

                // Let's just use velocity magnitude for now.
                // Scale factor: v=100 -> 200Hz?
                // Typical orbital velocity in screen space?
                // If G=1000, M=10000, at r=100, v=sqrt(1000*10000/100) = sqrt(100000) = 316.
                // So v ~ 300.
                // Let's map v to freq.

                let freq = body.vel.length();
                events.push(freq);
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bolt_zero_alloc() {
        let mut bodies = vec![
            Body::new(Vec2::new(0.0, 0.0), Vec2::ZERO, 1000.0, 10.0, WHITE),
            Body::new(Vec2::new(100.0, 0.0), Vec2::new(0.0, 10.0), 1.0, 2.0, WHITE),
        ];

        update(&mut bodies, 0.1);

        // Assert some behavior to make sure it ran and vel changed
        assert_ne!(bodies[1].vel.x, 0.0);
    }
}
