use macroquad::prelude::*;
use std::collections::VecDeque;

pub const G: f32 = 1000.0;
pub const TRAIL_LENGTH: usize = 200;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub trail: VecDeque<Vec2>,
}

impl Body {
    pub fn new(x: f32, y: f32, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            mass,
            radius,
            color,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }

    pub fn with_velocity(mut self, vx: f32, vy: f32) -> Self {
        self.vel = Vec2::new(vx, vy);
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

    pub fn step(&mut self, dt: f32) {
        // Velocity Verlet (Symplectic - Energy Conserving)
        let n = self.bodies.len();

        // 1. Calculate Forces (Initial)
        let mut forces = vec![Vec2::ZERO; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let r_vec = self.bodies[j].pos - self.bodies[i].pos;
                let r_sq = r_vec.length_squared();
                if r_sq > 1e-4 {
                    let r_mag = r_sq.sqrt();
                    let f_mag = G * self.bodies[i].mass * self.bodies[j].mass / r_sq;
                    forces[i] += r_vec / r_mag * f_mag;
                }
            }
        }

        // 2. First Half-Kick & Drift
        for i in 0..n {
            let acc = forces[i] / self.bodies[i].mass;
            let body = &mut self.bodies[i];

            body.vel += acc * 0.5 * dt;
            body.pos += body.vel * dt;

            // Update trail
            if body.trail.len() >= TRAIL_LENGTH {
                body.trail.pop_front();
            }
            body.trail.push_back(body.pos);
        }

        // 3. Calculate Forces (Final)
        let mut new_forces = vec![Vec2::ZERO; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let r_vec = self.bodies[j].pos - self.bodies[i].pos;
                let r_sq = r_vec.length_squared();
                if r_sq > 1e-4 {
                    let r_mag = r_sq.sqrt();
                    let f_mag = G * self.bodies[i].mass * self.bodies[j].mass / r_sq;
                    new_forces[i] += r_vec / r_mag * f_mag;
                }
            }
        }

        // 4. Second Half-Kick
        for i in 0..n {
            let acc = new_forces[i] / self.bodies[i].mass;
            self.bodies[i].vel += acc * 0.5 * dt;
        }
    }

    pub fn total_energy(&self) -> f32 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;

        for (i, body) in self.bodies.iter().enumerate() {
            kinetic += 0.5 * body.mass * body.vel.length_squared();

            for (j, other) in self.bodies.iter().enumerate() {
                if i >= j {
                    continue;
                }
                let r = body.pos.distance(other.pos);
                if r > 1e-6 {
                    potential -= G * body.mass * other.mass / r;
                }
            }
        }
        kinetic + potential
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_conservation() {
        let mut universe = Universe::new();
        // Sun
        universe.add_body(Body::new(0.0, 0.0, 1000.0, 10.0, YELLOW));
        // Earth
        let r = 100.0;
        let v = (G * 1000.0 / r).sqrt(); // ~100.0
        universe.add_body(Body::new(r, 0.0, 1.0, 5.0, BLUE).with_velocity(0.0, v));

        let initial_energy = universe.total_energy();

        // Simulate
        for _ in 0..10000 {
            universe.step(0.001);
        }

        let final_energy = universe.total_energy();
        let diff = (final_energy - initial_energy).abs();

        println!(
            "Initial E: {}, Final E: {}, Diff: {}",
            initial_energy, final_energy, diff
        );

        // Explicit Euler is horrible for orbits, energy should increase.
        // We assert strictly that energy is conserved to FAIL the test.
        assert!(diff < 1.0, "Energy drifted too much: {}", diff);
    }
}
