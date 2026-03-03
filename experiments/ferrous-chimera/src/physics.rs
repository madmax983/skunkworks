pub use locus::Vec2;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
    pub radius: f64,
    pub color: Color,
    pub trail: Vec<Vec2>,
}

impl Body {
    pub fn new(x: f64, y: f64, mass: f64, radius: f64, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            mass,
            radius,
            color,
            trail: Vec::with_capacity(50),
        }
    }

    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Self {
        self.vel = Vec2::new(vx, vy);
        self
    }
}
