use locus::Vec2;
use rand::Rng;

pub struct Fissure {
    pub points: Vec<Vec2>,
    pub intensity: f64,
    pub active: bool,
    pub direction: f64, // -1.0 or 1.0 (growth direction relative to strata normal)
}

impl Fissure {
    pub fn new(start: Vec2, intensity: f64, direction: f64) -> Self {
        Self {
            points: vec![start],
            intensity,
            active: true,
            direction,
        }
    }

    pub fn grow(&mut self) {
        if !self.active {
            return;
        }

        // Limit growth based on intensity
        if self.points.len() > (self.intensity * 5.0).max(5.0) as usize {
            self.active = false;
            return;
        }

        let mut rng = rand::thread_rng();
        let last = *self.points.last().unwrap();

        // Jagged growth perpendicular to strata (assuming strata is horizontal)
        // Direction 1.0 = Down, -1.0 = Up
        let angle_base = if self.direction > 0.0 {
            std::f64::consts::PI / 2.0
        } else {
            -std::f64::consts::PI / 2.0
        };

        let angle_var = rng.gen_range(-0.5..0.5);
        let len = rng.gen_range(1.0..3.0);

        let next_x = last.x + (angle_base + angle_var).cos() * len;
        let next_y = last.y + (angle_base + angle_var).sin() * len;

        self.points.push(Vec2::new(next_x, next_y));
    }
}
