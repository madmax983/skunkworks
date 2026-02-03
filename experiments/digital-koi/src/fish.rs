use ratatui::style::Color;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
pub struct PondParams {
    pub width: f64,
    pub height: f64,
    pub turbulence: f64, // 0.0 to 1.0 (CPU Load)
    pub visual_range: f64,
    pub protected_range: f64,
    pub centering_factor: f64,
    pub matching_factor: f64,
    pub avoid_factor: f64,
    pub max_speed: f64,
    pub min_speed: f64,
}

impl Default for PondParams {
    fn default() -> Self {
        Self {
            width: 100.0,
            height: 100.0,
            turbulence: 0.0,
            visual_range: 20.0,
            protected_range: 4.0,
            centering_factor: 0.0005,
            matching_factor: 0.05,
            avoid_factor: 0.05,
            max_speed: 1.0,
            min_speed: 0.5,
        }
    }
}

#[derive(Clone)]
pub struct Fish {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub color: Color,
    pub history: VecDeque<(f64, f64)>,
}

impl Fish {
    pub fn new(x: f64, y: f64, color: Color) -> Self {
        let mut history = VecDeque::new();
        history.push_back((x, y));
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            color,
            history,
        }
    }

    pub fn update(&mut self, flock: &[Fish], params: &PondParams) {
        let mut close_dx = 0.0;
        let mut close_dy = 0.0;
        let mut x_avg = 0.0;
        let mut y_avg = 0.0;
        let mut vx_avg = 0.0;
        let mut vy_avg = 0.0;
        let mut neighbors = 0;

        for other in flock {
            // Simplified distance check (squared distance)
            // Handle wrapping distance logic here is complex, so we'll do simple Euclidean first
            // and maybe add wrapping distance later if it looks weird.

            // Actually, for a proper toroid, we need to check the shortest path.
            // But simple Euclidean is often "good enough" for visual toys.
            let dx = other.x - self.x;
            let dy = other.y - self.y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < params.visual_range * params.visual_range && dist_sq > 0.00001 {
                if dist_sq < params.protected_range * params.protected_range {
                    close_dx += self.x - other.x;
                    close_dy += self.y - other.y;
                } else {
                    x_avg += other.x;
                    y_avg += other.y;
                    vx_avg += other.vx;
                    vy_avg += other.vy;
                    neighbors += 1;
                }
            }
        }

        // Separation
        self.vx += close_dx * params.avoid_factor;
        self.vy += close_dy * params.avoid_factor;

        // Alignment & Cohesion
        if neighbors > 0 {
            x_avg /= neighbors as f64;
            y_avg /= neighbors as f64;
            vx_avg /= neighbors as f64;
            vy_avg /= neighbors as f64;

            self.vx += (x_avg - self.x) * params.centering_factor;
            self.vy += (y_avg - self.y) * params.centering_factor;

            self.vx += (vx_avg - self.vx) * params.matching_factor;
            self.vy += (vy_avg - self.vy) * params.matching_factor;
        }

        // Turbulence (System Entropy)
        if params.turbulence > 0.0 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let noise_scale = params.turbulence * 0.2; // Max noise impulse
            self.vx += rng.gen_range(-noise_scale..noise_scale);
            self.vy += rng.gen_range(-noise_scale..noise_scale);
        }

        // Speed Limits
        let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if speed > params.max_speed {
            self.vx = (self.vx / speed) * params.max_speed;
            self.vy = (self.vy / speed) * params.max_speed;
        } else if speed < params.min_speed && speed > 0.0 {
            self.vx = (self.vx / speed) * params.min_speed;
            self.vy = (self.vy / speed) * params.min_speed;
        }

        // Update Position
        self.x += self.vx;
        self.y += self.vy;

        // Wrap around
        if self.x < 0.0 {
            self.x += params.width;
        }
        if self.x >= params.width {
            self.x -= params.width;
        }
        if self.y < 0.0 {
            self.y += params.height;
        }
        if self.y >= params.height {
            self.y -= params.height;
        }

        // History Trail
        self.history.push_front((self.x, self.y));
        if self.history.len() > 10 {
            self.history.pop_back();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_wrapping() {
        let mut fish = Fish::new(99.0, 99.0, Color::Red);
        fish.vx = 2.0;
        fish.vy = 2.0;
        let mut params = PondParams::default();
        params.width = 100.0;
        params.height = 100.0;
        params.max_speed = 5.0; // Allow it to move fast enough to wrap

        let flock = vec![];
        fish.update(&flock, &params);

        // Should wrap to ~1.0
        assert!(fish.x < 5.0, "X should wrap around (got {})", fish.x);
        assert!(fish.y < 5.0, "Y should wrap around (got {})", fish.y);
    }

    #[test]
    fn test_speed_limit() {
        let mut fish = Fish::new(50.0, 50.0, Color::Red);
        fish.vx = 100.0; // Way too fast
        fish.vy = 0.0;
        let params = PondParams::default(); // max_speed defaults to 1.0

        let flock = vec![];
        fish.update(&flock, &params);

        let speed = (fish.vx.powi(2) + fish.vy.powi(2)).sqrt();
        assert!(
            (speed - params.max_speed).abs() < 0.001,
            "Speed should be clamped"
        );
    }
}
