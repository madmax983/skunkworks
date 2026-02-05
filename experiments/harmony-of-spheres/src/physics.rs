use macroquad::prelude::*;
use std::collections::VecDeque;

pub const G: f64 = 1000.0; // Scaled for visual effect
pub const TRAIL_LENGTH: usize = 200;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: DVec2,
    pub vel: DVec2,
    pub mass: f64,
    pub radius: f32,
    pub color: Color,
    pub trail: VecDeque<Vec2>,
}

impl Body {
    pub fn new(x: f64, y: f64, mass: f64, radius: f32, color: Color) -> Self {
        Self {
            pos: DVec2::new(x, y),
            vel: DVec2::ZERO,
            mass,
            radius,
            color,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }

    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Self {
        self.vel = DVec2::new(vx, vy);
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, dt: f64) {
        let n = self.bodies.len();
        let mut accelerations = vec![DVec2::ZERO; n];

        // Compute forces
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let diff = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = diff.length_squared();

                // Softening to avoid singularity
                let softening = 1.0;
                let dist_cubed = (dist_sq + softening).powf(1.5);

                let f = G * self.bodies[j].mass / dist_cubed;

                accelerations[i] += diff * f;
            }
        }

        // Symplectic Euler
        // v(t+1) = v(t) + a(x(t)) * dt
        // x(t+1) = x(t) + v(t+1) * dt
        for i in 0..n {
            self.bodies[i].vel += accelerations[i] * dt;
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * dt;

            // Update trail
            if self.bodies[i].trail.len() >= TRAIL_LENGTH {
                self.bodies[i].trail.pop_front();
            }
            // Convert to f32 for rendering
            let pos = self.bodies[i].pos;
            self.bodies[i]
                .trail
                .push_back(Vec2::new(pos.x as f32, pos.y as f32));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_stability() {
        // Earth-Sun analog
        let mut universe = Universe::new();

        // Sun
        universe.add_body(Body::new(0.0, 0.0, 1000.0, 10.0, YELLOW));

        // Earth at r=100
        // v_circ = sqrt(G * M / r) = sqrt(1000 * 1000 / 100) = sqrt(10000) = 100
        let r = 100.0;
        let v = (G * 1000.0 / r).sqrt();

        universe.add_body(Body::new(r, 0.0, 1.0, 2.0, BLUE).with_velocity(0.0, v));

        let dt = 0.01;
        // Simulate for a significant time
        for _ in 0..10000 {
            universe.step(dt);
        }

        let earth = &universe.bodies[1];
        let final_r = earth.pos.length();

        // Check if radius stayed close to 100.0
        // Symplectic Euler preserves energy on average but radius oscillates.
        // It shouldn't drift away indefinitely.
        let drift = (final_r - r).abs();
        println!("Final radius: {}, drift: {}", final_r, drift);

        assert!(drift < 5.0, "Orbit drifted too much: {}", drift);
    }
}
