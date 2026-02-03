use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    pub fn dot(&self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketKind {
    Http,    // Green, Safe
    Ssh,     // Blue, Safe
    Malware, // Red, Bad
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub kind: PacketKind,
    pub radius: f64,
    pub active: bool,
}

impl Particle {
    pub fn new(x: f64, y: f64, kind: PacketKind) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0.0, 0.0),
            kind,
            radius: 0.5,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f64, gravity: Vec2) {
        if !self.active {
            return;
        }
        self.vel = self.vel + gravity * dt;
        self.pos = self.pos + self.vel * dt;

        // Simple friction/air resistance
        self.vel = self.vel * 0.99;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PinKind {
    Bumper,   // Standard bounce
    Redirect, // Bounces with extra force or specific direction (maybe later)
    Blocker,  // Absorbs? Or just bounce.
}

#[derive(Clone, Debug)]
pub struct Pin {
    pub pos: Vec2,
    pub radius: f64,
    pub kind: PinKind,
}

impl Pin {
    pub fn new(x: f64, y: f64, kind: PinKind) -> Self {
        Self {
            pos: Vec2::new(x, y),
            radius: 1.0,
            kind,
        }
    }
}

/// Resolves collision between particle and pin.
/// Returns true if collision occurred.
pub fn resolve_collision(particle: &mut Particle, pin: &Pin) -> bool {
    let diff = particle.pos - pin.pos;
    let dist_sq = diff.length_squared();
    let min_dist = particle.radius + pin.radius;

    if dist_sq < min_dist * min_dist {
        // Collision!
        let dist = dist_sq.sqrt();
        let normal = if dist == 0.0 {
            Vec2::new(0.0, 1.0)
        } else {
            diff * (1.0 / dist)
        };

        // Push particle out of pin
        let overlap = min_dist - dist;
        particle.pos = particle.pos + normal * overlap;

        // Reflect velocity
        // v' = v - 2(v . n)n
        // Add some restitution (bounciness)
        let restitution = 0.7;
        let v_dot_n = particle.vel.dot(normal);

        // Only reflect if moving towards the pin
        if v_dot_n < 0.0 {
            let j = -(1.0 + restitution) * v_dot_n;
            particle.vel = particle.vel + normal * j;

            // Add some randomness to prevent infinite loops/stacking
            // particle.vel.x += (rand::random::<f64>() - 0.5) * 0.1;
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_ops() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v2 - v1, Vec2::new(2.0, 2.0));
        assert_eq!(v1 * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_collision_reflection() {
        let mut p = Particle::new(0.0, 5.0, PacketKind::Http);
        p.vel = Vec2::new(0.0, -10.0); // Moving down
        p.radius = 1.0;

        let mut pin = Pin::new(0.0, 0.0, PinKind::Bumper);
        pin.radius = 1.0; // Total min dist = 2.0

        // Move particle to collision point (e.g., y=1.9)
        p.pos = Vec2::new(0.0, 1.9);

        let hit = resolve_collision(&mut p, &pin);
        assert!(hit);

        // Should be pushed out
        assert!(p.pos.y >= 2.0);

        // Velocity should be reflected (positive y now)
        assert!(p.vel.y > 0.0);
    }
}
