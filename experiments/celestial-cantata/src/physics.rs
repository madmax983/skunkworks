use macroquad::prelude::*;
use std::collections::VecDeque;

pub const G: f32 = 1.0;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub trail: VecDeque<Vec2>,
}

pub struct Universe {
    pub bodies: Vec<Body>,
}

impl Universe {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, dt: f32) {
        let n = self.bodies.len();
        let mut forces = vec![Vec2::ZERO; n];

        // 1. Calculate forces (O(N^2) but optimized to N(N-1)/2)
        for i in 0..n {
            for j in (i + 1)..n {
                let r_vec = self.bodies[j].pos - self.bodies[i].pos;
                let r_sq = r_vec.length_squared();
                if r_sq > 1e-4 {
                    let r_mag = r_sq.sqrt();
                    let f_mag = G * self.bodies[i].mass * self.bodies[j].mass / r_sq;
                    let f_vec = r_vec / r_mag * f_mag;
                    forces[i] += f_vec;
                    forces[j] -= f_vec;
                }
            }
        }

        // 2. Symplectic Euler Integration
        for i in 0..n {
            let acc = forces[i] / self.bodies[i].mass;
            self.bodies[i].vel += acc * dt;
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * dt;

            // Update trail
            if self.bodies[i].trail.len() >= 500 {
                self.bodies[i].trail.pop_front();
            }
            let pos = self.bodies[i].pos;
            self.bodies[i].trail.push_back(pos);
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
        // Sun at (0,0) with mass 1000
        universe.add_body(Body {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            mass: 1000.0,
            radius: 20.0,
            color: YELLOW,
            trail: VecDeque::new(),
        });

        // Earth at (100, 0) with circular velocity
        // v = sqrt(G * M / r) = sqrt(1.0 * 1000.0 / 100.0) = sqrt(10.0) approx 3.162
        let v_circ = (G * 1000.0 / 100.0).sqrt();
        universe.add_body(Body {
            pos: Vec2::new(100.0, 0.0),
            vel: Vec2::new(0.0, v_circ),
            mass: 1.0,
            radius: 5.0,
            color: BLUE,
            trail: VecDeque::new(),
        });

        let initial_energy = universe.total_energy();
        println!("Initial Energy: {}", initial_energy);

        // Simulate for a long time
        for _ in 0..1000 {
            universe.step(0.1);
        }

        let final_energy = universe.total_energy();
        println!("Final Energy: {}", final_energy);

        let diff = (final_energy - initial_energy).abs();

        // Assert that the Earth has moved
        let earth = &universe.bodies[1];
        assert!(
            earth.pos.x != 100.0 || earth.pos.y != 0.0,
            "Body did not move!"
        );

        // Then assert energy conservation
        assert!(diff < 1.0, "Energy drifted too much: {}", diff);
    }
}
