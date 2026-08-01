pub use locus::Vec2;
use ratatui::style::Color;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
    pub radius: f64,
    pub name: String,
    pub color: Color,
    pub is_fixed: bool, // Sun doesn't move
}

pub struct Ship {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2, // Thrust acc
    pub angle: f64,
    pub fuel: f64,
    pub max_fuel: f64,
    pub cargo: Vec<String>,
    pub thrusting: bool,
}

pub struct System {
    pub bodies: Vec<Body>,
    pub ship: Ship,
    pub g: f64,
    /// Pre-allocated buffer for force calculations to avoid allocation in the loop.
    pub forces: Vec<Vec2>,
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            ship: Ship {
                pos: Vec2::new(100.0, 0.0), // Start in orbit
                vel: Vec2::new(0.0, 3.0),
                acc: Vec2::zero(),
                angle: 0.0,
                fuel: 1000.0,
                max_fuel: 1000.0,
                cargo: Vec::new(),
                thrusting: false,
            },
            g: 100.0, // High gravity for fun
            forces: Vec::new(),
        }
    }

    fn calculate_n_body_forces(&mut self) {
        if self.forces.len() != self.bodies.len() {
            self.forces.resize(self.bodies.len(), Vec2::zero());
        }

        // Reset forces
        for force in &mut self.forces {
            *force = Vec2::zero();
        }

        for i in 0..self.bodies.len() {
            if self.bodies[i].is_fixed {
                continue;
            }

            let mut total_force = Vec2::zero();

            for j in 0..self.bodies.len() {
                if i == j {
                    continue;
                }

                let r = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = r.magnitude_squared();

                if dist_sq < 0.1 {
                    continue;
                }

                let dist = dist_sq.sqrt();
                let f = r / dist * (self.g * self.bodies[j].mass / dist_sq);
                total_force += f;
            }

            self.forces[i] = total_force;
        }
    }

    fn update_bodies(&mut self, dt: f64) {
        for (i, body) in self.bodies.iter_mut().enumerate() {
            if body.is_fixed {
                continue;
            }
            body.vel += self.forces[i] * dt;
            body.pos += body.vel * dt;
            body.acc = self.forces[i];
        }
    }

    fn calculate_ship_forces(&self) -> Vec2 {
        let mut ship_acc = Vec2::zero();
        for body in &self.bodies {
            let r = body.pos - self.ship.pos;
            let dist_sq = r.magnitude_squared();
            if dist_sq < 1.0 {
                continue;
            }
            let dist = dist_sq.sqrt();
            let a = r / dist * (self.g * body.mass / dist_sq);
            ship_acc += a;
        }
        ship_acc
    }

    fn update_ship(&mut self, dt: f64) {
        let mut ship_acc = self.calculate_ship_forces();

        // Add Thrust
        if self.ship.thrusting && self.ship.fuel > 0.0 {
            let thrust_dir = Vec2::new(self.ship.angle.cos(), self.ship.angle.sin());
            let thrust_power = 50.0;
            ship_acc += thrust_dir * thrust_power;
            self.ship.fuel -= dt * 10.0; // Burn fuel
        }

        self.ship.vel += ship_acc * dt;
        self.ship.pos += self.ship.vel * dt;
        self.ship.acc = ship_acc;
    }

    pub fn update(&mut self, dt: f64) {
        // 1. Calculate Acceleration (Gravity) for Bodies
        // We use Symplectic Euler for N-Body:
        // v += a * dt
        // x += v * dt
        self.calculate_n_body_forces();
        self.update_bodies(dt);

        // 2. Ship Physics
        self.update_ship(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v1 * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_orbit_stability() {
        // Earth orbits Sun.
        // v = sqrt(G * M / r)
        let mut sys = System::new();
        sys.g = 1.0;
        sys.bodies.push(Body {
            pos: Vec2::zero(),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            mass: 1000.0,
            radius: 10.0,
            name: "Sun".to_string(),
            color: Color::Yellow,
            is_fixed: true,
        });

        let r = 100.0;
        let v = (sys.g * 1000.0 / r).sqrt(); // sqrt(1 * 1000 / 100) = sqrt(10) ~= 3.16

        sys.bodies.push(Body {
            pos: Vec2::new(r, 0.0),
            vel: Vec2::new(0.0, v),
            acc: Vec2::zero(),
            mass: 1.0,
            radius: 2.0,
            name: "Earth".to_string(),
            color: Color::Blue,
            is_fixed: false,
        });

        // Simulate
        for _ in 0..1000 {
            sys.update(0.1);
        }

        let earth = &sys.bodies[1];
        let dist = earth.pos.magnitude();

        // Should be close to 100.0
        // Symplectic Euler is stable but not exact.
        assert!(
            (dist - 100.0).abs() < 5.0,
            "Orbit drifted too much: {}",
            dist
        );
    }
}
