use ratatui::style::Color;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalize(self) -> Self {
        let l = self.length();
        if l > 0.0 {
            self / l
        } else {
            Self::zero()
        }
    }
    pub fn rotate(self, angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

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
        }
    }

    pub fn update(&mut self, dt: f64) {
        // 1. Calculate Acceleration (Gravity) for Bodies
        // We use Symplectic Euler for N-Body:
        // v += a * dt
        // x += v * dt
        // But for better stability, maybe Verlet.
        // Let's stick to Semi-Implicit Euler (Symplectic Euler) as it's simple and energy conserving-ish.

        let mut forces = vec![Vec2::zero(); self.bodies.len()];

        for i in 0..self.bodies.len() {
            if self.bodies[i].is_fixed { continue; }
            for j in 0..self.bodies.len() {
                if i == j { continue; }
                let r = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = r.length_squared();
                if dist_sq < 0.1 { continue; } // Softening
                let dist = dist_sq.sqrt();
                let f = r / dist * (self.g * self.bodies[j].mass / dist_sq); // F/m = a. We just want 'a' from other body.
                // Wait, F = G m1 m2 / r^2. a1 = F / m1 = G m2 / r^2.
                forces[i] += f;
            }
        }

        // Update Bodies
        for (i, body) in self.bodies.iter_mut().enumerate() {
            if body.is_fixed { continue; }
            body.vel += forces[i] * dt;
            body.pos += body.vel * dt;
            body.acc = forces[i];
        }

        // 2. Ship Physics
        // Ship is affected by all bodies.
        let mut ship_acc = Vec2::zero();
        for body in &self.bodies {
            let r = body.pos - self.ship.pos;
            let dist_sq = r.length_squared();
            if dist_sq < 1.0 { continue; }
            let dist = dist_sq.sqrt();
            let a = r / dist * (self.g * body.mass / dist_sq);
            ship_acc += a;
        }

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
        let dist = earth.pos.length();

        // Should be close to 100.0
        // Symplectic Euler is stable but not exact.
        assert!((dist - 100.0).abs() < 5.0, "Orbit drifted too much: {}", dist);
    }
}
