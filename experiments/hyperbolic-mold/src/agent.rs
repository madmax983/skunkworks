use macroquad::prelude::*;
use ::rand::Rng;

const SENSOR_ANGLE: f32 = std::f32::consts::PI / 4.0; // 45 degrees
// Wait, if grid is [-1, 1], then screen pixels don't apply directly unless mapped.
// I should decide on coordinate system.
// Option A: World coordinates are [-1.0, 1.0]. Grid is WxH.
// Option B: World coordinates are [0, Width].
// Using [-1, 1] is more natural for Poincaré math.
// But SENSOR_DIST needs to be small (e.g. 0.01).
// If width is 800, 9 pixels is roughly 9/400 = 0.02.
// Let's use coordinate system [-1.0, 1.0].

// Revised constants for [-1.0, 1.0] space
const SENSOR_DIST_WORLD: f32 = 0.02;

const ROTATION_ANGLE: f32 = std::f32::consts::PI / 8.0; // 22.5 degrees

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub position: Vec2,
    pub angle: f32,
    pub base_speed: f32,
}

impl Agent {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self {
            position,
            angle,
            base_speed: 0.005, // Small step size in normalized coords
        }
    }

    pub fn sense(
        &self,
        sensor_angle_offset: f32,
        width: usize,
        height: usize,
        grid: &[f32],
    ) -> f32 {
        let sensor_angle = self.angle + sensor_angle_offset;
        let sensor_dir = vec2(sensor_angle.cos(), sensor_angle.sin());
        let sensor_pos = self.position + sensor_dir * SENSOR_DIST_WORLD;

        // Map [-1, 1] to [0, width]
        let x_norm = (sensor_pos.x + 1.0) / 2.0;
        let y_norm = (sensor_pos.y + 1.0) / 2.0;

        if x_norm < 0.0 || x_norm >= 1.0 || y_norm < 0.0 || y_norm >= 1.0 {
            return 0.0;
        }

        let x = (x_norm * (width as f32)) as usize;
        let y = (y_norm * (height as f32)) as usize;

        if x >= width || y >= height {
            return 0.0;
        }

        grid[y * width + x]
    }

    pub fn update(&mut self, width: usize, height: usize, grid: &[f32]) {
        let sensor_left = self.sense(-SENSOR_ANGLE, width, height, grid);
        let sensor_center = self.sense(0.0, width, height, grid);
        let sensor_right = self.sense(SENSOR_ANGLE, width, height, grid);

        let mut rng = ::rand::thread_rng();
        let random_steer: f32 = rng.gen_range(0.0..1.0);

        if sensor_center > sensor_left && sensor_center > sensor_right {
            // Keep direction
        } else if sensor_center < sensor_left && sensor_center < sensor_right {
             if random_steer < 0.5 {
                self.angle -= ROTATION_ANGLE;
            } else {
                self.angle += ROTATION_ANGLE;
            }
        } else if sensor_left < sensor_right {
            self.angle += ROTATION_ANGLE;
        } else if sensor_right < sensor_left {
            self.angle -= ROTATION_ANGLE;
        }

        let direction = vec2(self.angle.cos(), self.angle.sin());

        let r_sq = self.position.length_squared();
        // Scale speed by (1 - r^2) / 2 (conformal factor)
        // Actually, just (1 - r^2) gives a good visual effect.
        let scale_factor = (1.0 - r_sq).max(0.0);

        let current_speed = self.base_speed * scale_factor;

        self.position += direction * current_speed;

        // Boundary Check
        let r = self.position.length();
        if r >= 0.99 {
             self.position = self.position.normalize() * 0.99;
             self.angle += std::f32::consts::PI; // Turn back
        }
    }
}
