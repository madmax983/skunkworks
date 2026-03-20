use ferrous_core::Platter;
use macroquad::prelude::*;

use crate::git::Commit;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub commit: Commit,
    pub active: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32, commit: Commit) -> Self {
        let mut hasher = DefaultHasher::new();
        commit.author.hash(&mut hasher);
        let h = hasher.finish();

        let color = match h % 6 {
            0 => Color::new(1.0, 0.2, 0.2, 0.8),
            1 => Color::new(0.2, 1.0, 0.2, 0.8),
            2 => Color::new(1.0, 1.0, 0.2, 0.8),
            3 => Color::new(0.2, 0.2, 1.0, 0.8),
            4 => Color::new(1.0, 0.2, 1.0, 0.8),
            5 => Color::new(0.2, 1.0, 1.0, 0.8),
            _ => Color::new(1.0, 1.0, 1.0, 0.8),
        };

        Self {
            pos: vec2(x, y),
            vel: vec2(0.0, 0.0),
            color,
            commit,
            active: true,
        }
    }

    pub fn update(&mut self, platter: &Platter, grid_scale: f32, dt: f32) {
        // Sample magnetic field gradient
        // Grid coordinates
        let gx = (self.pos.x / grid_scale) as i32;
        let gy = (self.pos.y / grid_scale) as i32;

        let mut force = vec2(0.0, 0.0);

        // Simple gradient: Look at neighbors
        // Center value
        let _c = get_mag(platter, gx, gy);
        let r = get_mag(platter, gx + 1, gy);
        let l = get_mag(platter, gx - 1, gy);
        let d = get_mag(platter, gx, gy + 1);
        let u = get_mag(platter, gx, gy - 1);

        // Gradient points towards HIGHER magnetism
        force.x = (r - l) * 100.0;
        force.y = (d - u) * 100.0;

        // Also add some random jitter (Brownian motion)
        force.x += rand::gen_range(-5.0, 5.0);
        force.y += rand::gen_range(-5.0, 5.0);

        // Apply force
        self.vel += force * dt;

        // Damping
        self.vel *= 0.95;

        // Move
        self.pos += self.vel * dt;

        // Destroy if offscreen horizontally
        let w = screen_width();
        if self.pos.x > w + 50.0 || self.pos.x < -50.0 {
            self.active = false;
        }
    }

    pub fn draw(&self) {
        if !self.active { return; }
        draw_circle(self.pos.x, self.pos.y, 4.0, self.color);
        // Draw commit message nearby occasionally or maybe just author
        draw_text(&self.commit.author, self.pos.x + 6.0, self.pos.y + 3.0, 16.0, Color::new(1.0, 1.0, 1.0, 0.4));
    }
}

fn get_mag(platter: &Platter, x: i32, y: i32) -> f32 {
    if x < 0 || y < 0 || x >= platter.width() as i32 || y >= platter.height() as i32 {
        return 0.0;
    }
    platter.get_magnetism(x as usize, y as usize) as f32
}
