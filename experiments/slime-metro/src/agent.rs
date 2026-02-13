use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct SimParams {
    pub width: usize,
    pub height: usize,
    pub sensor_angle: f32,
    pub sensor_dist: f32,
    pub turn_speed: f32,
    pub move_speed: f32,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
    pub cargo: f32, // 0.0 to 1.0
}

impl Agent {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        Self {
            pos,
            angle,
            cargo: 0.0,
        }
    }

    /// Senses the trail map at (x, y)
    /// Returns 0.0 if out of bounds.
    #[inline(always)]
    fn sense(
        &self,
        map: &[f32],
        width: usize,
        height: usize,
        angle_offset: f32,
        sensor_dist: f32,
    ) -> f32 {
        let angle = self.angle + angle_offset;
        let sens_x = self.pos.x + angle.cos() * sensor_dist;
        let sens_y = self.pos.y + angle.sin() * sensor_dist;

        let ix = sens_x as isize;
        let iy = sens_y as isize;

        if ix >= 0 && ix < width as isize && iy >= 0 && iy < height as isize {
            map[iy as usize * width + ix as usize]
        } else {
            0.0
        }
    }

    pub fn update(&mut self, trail_map: &[f32], params: SimParams) {
        let width = params.width;
        let height = params.height;

        // Sense
        let forward = self.sense(trail_map, width, height, 0.0, params.sensor_dist);
        let left = self.sense(trail_map, width, height, -params.sensor_angle, params.sensor_dist);
        let right = self.sense(trail_map, width, height, params.sensor_angle, params.sensor_dist);

        // Turn
        let random_steer = (fastrand::f32() - 0.5) * 0.1; // Small jitter

        if forward > left && forward > right {
            // Keep going, maybe slight jitter
            self.angle += random_steer;
        } else if forward < left && forward < right {
            // Random turn
            self.angle += (fastrand::f32() - 0.5) * 2.0 * params.turn_speed;
        } else if left > right {
            self.angle -= params.turn_speed + random_steer;
        } else if right > left {
            self.angle += params.turn_speed + random_steer;
        } else {
             self.angle += random_steer;
        }

        // Move
        let dx = self.angle.cos() * params.move_speed;
        let dy = self.angle.sin() * params.move_speed;

        self.pos.x += dx;
        self.pos.y += dy;

        // Bounce off walls
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.angle = PI - self.angle;
        } else if self.pos.x >= width as f32 {
            self.pos.x = width as f32 - 1.0;
            self.angle = PI - self.angle;
        }

        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.angle = -self.angle;
        } else if self.pos.y >= height as f32 {
            self.pos.y = height as f32 - 1.0;
            self.angle = -self.angle;
        }
    }
}
