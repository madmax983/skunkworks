use crate::boid::{Boid, distance_squared, limit};
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

pub struct Critic {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
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
            position: (x, y),
            velocity: (angle.cos() * 1.5, angle.sin() * 1.5),
            acceleration: (0.0, 0.0),
            max_speed: 1.8, // Slightly faster than average boid (0.5-1.5)
            max_force: 0.15,
            color: Color::Red,
            symbol: '@',
            kill_radius: 3.0,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;

        self.velocity = limit(self.velocity, self.max_speed);

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        self.acceleration = (0.0, 0.0);

        // Wrap around
        if self.position.0 < 0.0 {
            self.position.0 += width;
        }
        if self.position.0 >= width {
            self.position.0 -= width;
        }
        if self.position.1 < 0.0 {
            self.position.1 += height;
        }
        if self.position.1 >= height {
            self.position.1 -= height;
        }
    }

    pub fn apply_force(&mut self, force: (f64, f64)) {
        self.acceleration.0 += force.0;
        self.acceleration.1 += force.1;
    }

    // Hunts the nearest boid
    pub fn hunt(&self, boids: &[Boid]) -> (f64, f64) {
        let mut closest_dist = f64::MAX;
        let mut target = None;

        for boid in boids {
            let d_sq = distance_squared(self.position, boid.position);
            if d_sq < closest_dist {
                closest_dist = d_sq;
                target = Some(boid.position);
            }
        }

        if let Some(target_pos) = target {
            self.seek(target_pos)
        } else {
            (0.0, 0.0)
        }
    }

    fn seek(&self, target: (f64, f64)) -> (f64, f64) {
        let desired = (target.0 - self.position.0, target.1 - self.position.1);
        // Normalize and scale to max_speed
        let len = (desired.0.powi(2) + desired.1.powi(2)).sqrt();
        if len == 0.0 {
            return (0.0, 0.0);
        }

        let desired = (
            (desired.0 / len) * self.max_speed,
            (desired.1 / len) * self.max_speed,
        );

        let steer = (desired.0 - self.velocity.0, desired.1 - self.velocity.1);
        limit(steer, self.max_force)
    }
}

// Helper for boids to flee from critic
pub fn flee(
    boid_pos: (f64, f64),
    boid_vel: (f64, f64),
    critic_pos: (f64, f64),
    max_speed: f64,
    max_force: f64,
) -> (f64, f64) {
    let d_sq = distance_squared(boid_pos, critic_pos);
    if d_sq > 2500.0 {
        // 50.0 radius
        return (0.0, 0.0);
    }

    let desired = (boid_pos.0 - critic_pos.0, boid_pos.1 - critic_pos.1);
    let len = (desired.0.powi(2) + desired.1.powi(2)).sqrt();
    if len == 0.0 {
        return (0.0, 0.0);
    }

    let desired = ((desired.0 / len) * max_speed, (desired.1 / len) * max_speed);

    let steer = (desired.0 - boid_vel.0, desired.1 - boid_vel.1);
    limit(steer, max_force)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hunt_seeks_nearest() {
        let mut critic = Critic::new(0.0, 0.0);
        critic.velocity = (0.0, 0.0);

        let mut boid = crate::boid::Boid::new(10.0, 0.0); // Target at (10, 0)
        boid.dna.max_speed = 1.0;

        let boids = vec![boid];
        let force = critic.hunt(&boids);

        // Should pull towards positive X
        assert!(force.0 > 0.0);
        assert!((force.1).abs() < 0.1);
    }

    #[test]
    fn test_flee_avoids_critic() {
        let boid_pos = (10.0, 0.0);
        let boid_vel = (0.0, 0.0);
        let critic_pos = (0.0, 0.0); // Critic is to the left

        let force = flee(boid_pos, boid_vel, critic_pos, 2.0, 0.1);

        // Should push away towards positive X
        assert!(force.0 > 0.0);
    }
}
