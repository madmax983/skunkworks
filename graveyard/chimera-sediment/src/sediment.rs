use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct SedimentParticle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    pub mass: f32,
}

impl SedimentParticle {
    pub fn new(pos: Vec2, color: Color, size: f32) -> Self {
        Self {
            pos,
            vel: vec2(0.0, 0.0),
            color,
            size,
            mass: size, // Simpler
        }
    }

    pub fn update(&mut self, dt: f32, ground_level: f32) {
        // Gravity
        self.vel.y += 200.0 * dt;
        self.pos += self.vel * dt;

        // Ground Collision
        if self.pos.y > ground_level {
            self.pos.y = ground_level;
            self.vel.y = 0.0;
            self.vel.x = 0.0;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x - self.size / 2.0,
            self.pos.y - self.size / 2.0,
            self.size,
            self.size,
            self.color,
        );
    }
}
