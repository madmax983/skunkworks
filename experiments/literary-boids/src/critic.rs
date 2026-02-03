use crate::boid::{Boid, Vec2};
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

pub struct Critic {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_speed: f64,
    pub max_force: f64,
    pub color: Color,
    pub symbol: char,
    pub kill_radius: f64,
}

impl Critic {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * 1.5, angle.sin() * 1.5),
            acceleration: Vec2::zero(),
            max_speed: 1.8, // Slightly faster than average boid (0.5-1.5)
            max_force: 0.15,
            color: Color::Red,
            symbol: '@',
            kill_radius: 3.0,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.max_speed);

        self.position += self.velocity;
        self.acceleration = Vec2::zero();

        // Wrap around
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
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    // Hunts the nearest boid
    pub fn hunt(&self, boids: &[Boid]) -> Vec2 {
        let mut closest_dist = f64::MAX;
        let mut target = None;

        for boid in boids {
            let d_sq = self.position.distance_squared(boid.position);
            if d_sq < closest_dist {
                closest_dist = d_sq;
                target = Some(boid.position);
            }
        }

        if let Some(target_pos) = target {
            self.seek(target_pos)
        } else {
            Vec2::zero()
        }
    }

    fn seek(&self, target: Vec2) -> Vec2 {
        let mut desired = target - self.position;
        if desired.magnitude_squared() > 0.0 {
            desired = desired.normalize() * self.max_speed;
            desired -= self.velocity;
            desired = desired.limit(self.max_force);
            desired
        } else {
            Vec2::zero()
        }
    }
}

// Helper for boids to flee from critic
pub fn flee(
    boid_pos: Vec2,
    boid_vel: Vec2,
    critic_pos: Vec2,
    max_speed: f64,
    max_force: f64,
) -> Vec2 {
    let d_sq = boid_pos.distance_squared(critic_pos);
    if d_sq > 2500.0 {
        // 50.0 radius
        return Vec2::zero();
    }

    let mut desired = boid_pos - critic_pos;
    if desired.magnitude_squared() > 0.0 {
        desired = desired.normalize() * max_speed;
        desired -= boid_vel;
        desired = desired.limit(max_force);
        desired
    } else {
        Vec2::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hunt_seeks_nearest() {
        let mut critic = Critic::new(0.0, 0.0);
        critic.velocity = Vec2::zero();

        let mut boid = crate::boid::Boid::new(10.0, 0.0); // Target at (10, 0)
        boid.dna.max_speed = 1.0;

        let boids = vec![boid];
        let force = critic.hunt(&boids);

        // Should pull towards positive X
        assert!(force.x > 0.0);
        assert!((force.y).abs() < 0.1);
    }

    #[test]
    fn test_flee_avoids_critic() {
        let boid_pos = Vec2::new(10.0, 0.0);
        let boid_vel = Vec2::zero();
        let critic_pos = Vec2::zero(); // Critic is to the left

        let force = flee(boid_pos, boid_vel, critic_pos, 2.0, 0.1);

        // Should push away towards positive X
        assert!(force.x > 0.0);
    }
}
