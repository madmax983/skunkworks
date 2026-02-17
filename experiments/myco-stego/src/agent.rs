use glam::Vec2;
use rand::Rng;
use std::f32::consts::PI;

use crate::grid::{Grid, HiddenLayer};

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
    pub sensor_angle: f32,
    pub sensor_dist: f32,
    pub turn_angle: f32,
    pub move_speed: f32,
}

impl Agent {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        Self {
            pos,
            angle,
            sensor_angle: PI / 4.0,
            sensor_dist: 2.0, // Reduced distance for TUI grid resolution
            turn_angle: PI / 4.0,
            move_speed: 1.0,
        }
    }

    pub fn sense(&self, grid: &Grid, hidden: &HiddenLayer, angle_offset: f32) -> f32 {
        let angle = self.angle + angle_offset;
        let sensor_dir = Vec2::new(angle.cos(), angle.sin());
        let sensor_pos = self.pos + sensor_dir * self.sensor_dist;

        let x = (sensor_pos.x.rem_euclid(grid.width as f32)) as usize;
        let y = (sensor_pos.y.rem_euclid(grid.height as f32)) as usize;

        let pheromone = grid.get(x, y);
        let hidden_signal = if hidden.is_active(x, y) { 2.0 } else { 0.0 }; // Strong attraction to hidden data

        pheromone + hidden_signal
    }

    pub fn update(&mut self, grid: &mut Grid, hidden: &HiddenLayer) {
        let mut rng = rand::thread_rng();

        let left = self.sense(grid, hidden, -self.sensor_angle);
        let center = self.sense(grid, hidden, 0.0);
        let right = self.sense(grid, hidden, self.sensor_angle);

        if center > left && center > right {
            // Stay course
        } else if center < left && center < right {
            // Turn randomly
            if rng.gen_bool(0.5) {
                self.angle += self.turn_angle;
            } else {
                self.angle -= self.turn_angle;
            }
        } else if left > right {
            self.angle -= self.turn_angle;
        } else if right > left {
            self.angle += self.turn_angle;
        }

        // Move
        let dir = Vec2::new(self.angle.cos(), self.angle.sin());
        self.pos += dir * self.move_speed;

        // Wrap
        self.pos.x = self.pos.x.rem_euclid(grid.width as f32);
        self.pos.y = self.pos.y.rem_euclid(grid.height as f32);

        // Deposit Pheromone
        // Deposit MORE if on hidden data (reinforcement)
        let x = self.pos.x as usize;
        let y = self.pos.y as usize;
        let deposit_amount = if hidden.is_active(x, y) { 0.5 } else { 0.1 };

        // Write to grid (NOTE: This is serial update, avoiding race conditions since single threaded app loop)
        grid.deposit(self.pos, deposit_amount);
    }
}
