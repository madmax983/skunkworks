use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::zero()
        } else {
            *self / mag
        }
    }

    pub fn limit(&self, max: f64) -> Self {
        if self.magnitude_squared() > max * max {
            self.normalize() * max
        } else {
            *self
        }
    }

    pub fn distance_squared(&self, other: Vec2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, other: Vec2) -> f64 {
        self.distance_squared(other).sqrt()
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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
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

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
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

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

#[derive(Clone, Debug)]
pub struct DNA {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub color: Color,
    pub char_representation: char,
}

impl DNA {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.5..1.5),
            max_force: rng.gen_range(0.02..0.1),
            view_radius: rng.gen_range(5.0..15.0),
            separation_weight: rng.gen_range(1.0..2.0),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            color: Color::White,
            char_representation: '*',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: DNA,
    pub energy: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = DNA::random();

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            dna,
            energy: 100.0,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);

        self.position += self.velocity;

        // Reset acceleration
        self.acceleration = Vec2::zero();

        // Wrap around edges
        if self.position.x < 0.0 {
            self.position.x += width;
        }
        if self.position.x >= width {
            self.position.x -= width;
        }
        if self.position.y < 0.0 {
            self.position.y += height;
        }
        if self.position.y >= height {
            self.position.y -= height;
        }

        // Decay energy
        self.energy -= 0.05;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    // Returns the force vector to be applied
    pub fn calculate_flocking_force(&self, boids: &[Boid]) -> Vec2 {
        let separation = self.calculate_separation(boids);
        let alignment = self.calculate_alignment(boids);
        let cohesion = self.calculate_cohesion(boids);

        separation * self.dna.separation_weight
            + alignment * self.dna.alignment_weight
            + cohesion * self.dna.cohesion_weight
    }

    fn calculate_separation(&self, boids: &[Boid]) -> Vec2 {
        let mut steer = Vec2::zero();
        let mut count = 0;
        let separation_radius_sq = (self.dna.view_radius / 2.0).powi(2);

        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < separation_radius_sq {
                let diff = self.position - other.position;
                steer += diff / d_sq;
                count += 1;
            }
        }

        if count > 0 && steer.magnitude_squared() > 0.0 {
            steer = steer.normalize() * self.dna.max_speed;
            steer -= self.velocity;
            steer = steer.limit(self.dna.max_force);
            steer
        } else {
            Vec2::zero()
        }
    }

    fn calculate_alignment(&self, boids: &[Boid]) -> Vec2 {
        let mut sum = Vec2::zero();
        let mut count = 0;
        let view_radius_sq = self.dna.view_radius.powi(2);

        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < view_radius_sq {
                sum += other.velocity;
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f64;
            if sum.magnitude_squared() > 0.0 {
                sum = sum.normalize() * self.dna.max_speed;
                sum -= self.velocity;
                sum = sum.limit(self.dna.max_force);
                return sum;
            }
        }
        Vec2::zero()
    }

    fn calculate_cohesion(&self, boids: &[Boid]) -> Vec2 {
        let mut sum = Vec2::zero();
        let mut count = 0;
        let view_radius_sq = self.dna.view_radius.powi(2);

        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < view_radius_sq {
                sum += other.position;
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f64;
            return self.seek(sum);
        }
        Vec2::zero()
    }

    fn seek(&self, target: Vec2) -> Vec2 {
        let mut desired = target - self.position;
        if desired.magnitude_squared() > 0.0 {
            desired = desired.normalize() * self.dna.max_speed;
            desired -= self.velocity;
            desired = desired.limit(self.dna.max_force);
            desired
        } else {
            Vec2::zero()
        }
    }
}

// Deprecated or wrappers
pub fn distance(p1: Vec2, p2: Vec2) -> f64 {
    p1.distance(p2)
}

pub fn distance_squared(p1: Vec2, p2: Vec2) -> f64 {
    p1.distance_squared(p2)
}

// limit is no longer needed as standalone, but if we keep it for backward compat it needs Vec2
pub fn limit(vector: Vec2, max: f64) -> Vec2 {
    vector.limit(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boid_movement() {
        let mut boid = Boid::new(0.0, 0.0);
        boid.dna.max_speed = 2.0; // Ensure speed isn't capped
        boid.velocity = Vec2::new(1.0, 0.0);
        boid.acceleration = Vec2::zero();
        boid.update(100.0, 100.0);

        assert!((boid.position.x - 1.0).abs() < 1e-6);
        assert!((boid.position.y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_boundary_wrapping() {
        let mut boid = Boid::new(99.5, 50.0);
        boid.dna.max_speed = 2.0; // Ensure speed isn't capped
        boid.velocity = Vec2::new(1.0, 0.0);
        boid.update(100.0, 100.0);

        // Should wrap to 0.5
        assert!((boid.position.x - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_distance_squared() {
        let p1 = Vec2::new(0.0, 0.0);
        let p2 = Vec2::new(3.0, 4.0);
        assert!((distance_squared(p1, p2) - 25.0).abs() < 1e-6);
        assert!((distance(p1, p2) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_flocking_force_zero_alone() {
        let boid = Boid::new(50.0, 50.0);
        let flock = vec![];
        let force = boid.calculate_flocking_force(&flock);
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_flocking_force_separation() {
        // Create a boid at (50, 50)
        let mut boid1 = Boid::new(50.0, 50.0);
        boid1.velocity = Vec2::zero();
        boid1.dna.view_radius = 10.0;
        boid1.dna.max_speed = 2.0;
        boid1.dna.max_force = 0.1;
        boid1.dna.separation_weight = 1.0;
        boid1.dna.alignment_weight = 0.0; // Isolate separation
        boid1.dna.cohesion_weight = 0.0;

        // Create another boid very close (50.1, 50.0)
        let boid2 = Boid::new(50.1, 50.0);

        // This should trigger separation force pushing boid1 to the LEFT (negative X)
        // boid1 is at 50, boid2 is at 50.1. Diff is 50 - 50.1 = -0.1.
        let force = boid1.calculate_flocking_force(&[boid2]);

        assert!(
            force.x < 0.0,
            "Force X should be negative (separation), got {}",
            force.x
        );
        assert_eq!(force.y, 0.0, "Force Y should be zero");
    }
}
