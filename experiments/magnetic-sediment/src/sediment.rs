use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct SedimentParticle {
    pub pos: Vec2,
    pub mass: f32,
    pub magnetic_charge: f32,
    pub color: Color,
}

impl SedimentParticle {
    pub fn new(pos: Vec2, mass: f32) -> Self {
        Self {
            pos,
            mass,
            magnetic_charge: mass * 0.1, // Charge proportional to mass
            color: Color::new(0.4, 0.4, 0.4, 0.8), // Gray
        }
    }
}
