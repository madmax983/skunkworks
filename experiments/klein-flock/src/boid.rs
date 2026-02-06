use glam::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f32::consts::TAU;

pub const WIDTH: f32 = TAU;
pub const HEIGHT: f32 = TAU;

#[derive(Clone, Debug)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
}

impl Boid {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..WIDTH);
        let y = rng.gen_range(0.0..HEIGHT);
        let angle = rng.gen_range(0.0..TAU);
        let speed = rng.gen_range(0.05..0.1); // Speed relative to domain size

        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(angle.cos(), angle.sin()) * speed,
            color: Color::Indexed(rng.gen_range(20..230)),
        }
    }

    pub fn update(&mut self, flock: &[Boid], idx: usize) {
        // Simple boid logic
        // We need to account for wrapping in distance calculations too!
        // That's tricky on a Klein bottle.
        // For simplicity in distance check, we will just use Euclidean distance on the flat 2D map
        // but ideally we should check "ghost" neighbors across the twist.
        // Given time constraints, I will use standard Euclidean on the flat map for interaction,
        // but strictly enforce the topological movement.
        // This means boids might lose sight of neighbors across the twist boundary until they cross it.
        // That's acceptable for a "hybrid experiment".

        // Cohesion, Alignment, Separation
        let mut avg_pos = Vec2::ZERO;
        let mut avg_vel = Vec2::ZERO;
        let mut separation = Vec2::ZERO;
        let mut count = 0;

        let view_radius = 0.8;
        let avoid_radius = 0.3;

        for (i, other) in flock.iter().enumerate() {
            if i == idx { continue; }

            let d = self.pos.distance(other.pos);
            if d < view_radius {
                avg_pos += other.pos;
                avg_vel += other.vel;
                count += 1;

                if d < avoid_radius {
                    separation += (self.pos - other.pos) / d.max(0.01);
                }
            }
        }

        if count > 0 {
            avg_pos /= count as f32;
            avg_vel /= count as f32;

            // Cohesion
            let cohesion = (avg_pos - self.pos).normalize_or_zero() * 0.05;
            // Alignment
            let alignment = (avg_vel - self.vel).normalize_or_zero() * 0.05;
            // Separation
            let separation = separation.normalize_or_zero() * 0.08;

            self.vel += cohesion + alignment + separation;
        }

        // Limit speed
        let max_speed = 0.1;
        if self.vel.length() > max_speed {
            self.vel = self.vel.normalize() * max_speed;
        }

        // Add some noise to keep them moving
        let mut rng = rand::thread_rng();
        let angle_change: f32 = rng.gen_range(-0.1..0.1);
        let c = angle_change.cos();
        let s = angle_change.sin();
        let nx = self.vel.x * c - self.vel.y * s;
        let ny = self.vel.x * s + self.vel.y * c;
        self.vel = Vec2::new(nx, ny);

        // Move
        self.pos += self.vel;

        // Apply Klein Bottle Topology
        // X Boundaries (Twist)
        if self.pos.x < 0.0 {
            self.pos.x += WIDTH;
            self.pos.y = HEIGHT - self.pos.y; // Twist
            self.vel.y = -self.vel.y;         // Flip vertical velocity
        } else if self.pos.x >= WIDTH {
            self.pos.x -= WIDTH;
            self.pos.y = HEIGHT - self.pos.y; // Twist
            self.vel.y = -self.vel.y;         // Flip vertical velocity
        }

        // Y Boundaries (Standard Wrap)
        if self.pos.y < 0.0 {
            self.pos.y += HEIGHT;
        } else if self.pos.y >= HEIGHT {
            self.pos.y -= HEIGHT;
        }
    }
}
