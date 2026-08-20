pub use locus::Vec2;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub color: Color,
    pub trail: Vec<Vec2>,
}

impl Body {
    pub fn new(x: f64, y: f64, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            color,
            trail: Vec::with_capacity(50),
        }
    }

    pub fn apply_wrap_around(&mut self, min: f64, max: f64) {
        if self.pos.x < min {
            self.pos.x = max;
        } else if self.pos.x > max {
            self.pos.x = min;
        }
        if self.pos.y < min {
            self.pos.y = max;
        } else if self.pos.y > max {
            self.pos.y = min;
        }
    }
}
