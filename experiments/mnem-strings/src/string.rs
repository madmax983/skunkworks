use macroquad::prelude::*;

pub struct FerrousString {
    pub pos: Vec2,
    pub length: f32,
    pub base_freq: f32,
    pub vibration: f32,
    pub velocity: f32,
    pub tension: f32,
    pub decay_rate: f32,
    pub color: Color,
    pub segments: Vec<Vec2>,
}

impl FerrousString {
    pub fn new(pos: Vec2, length: usize, base_freq: f32) -> Self {
        let mut segments = Vec::new();
        let step = length as f32 / 20.0;
        for i in 0..=20 {
            segments.push(vec2(pos.x, pos.y + i as f32 * step));
        }

        Self {
            pos,
            length: length as f32,
            base_freq,
            vibration: 0.0,
            velocity: 0.0,
            tension: 200.0,
            decay_rate: 0.98,
            color: WHITE,
            segments,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let damping = 2.0;
        let acceleration = -self.tension * self.vibration - damping * self.velocity;
        self.velocity += acceleration * dt;
        self.vibration += self.velocity * dt;
        self.vibration *= self.decay_rate;
        self.vibration = self.vibration.clamp(-40.0, 40.0);

        let step = self.length / 20.0;
        for i in 0..=20 {
            let ratio = i as f32 / 20.0;
            let shape = (std::f32::consts::PI * ratio).sin();
            let x = self.pos.x + self.vibration * shape;
            let y = self.pos.y + i as f32 * step;
            self.segments[i] = vec2(x, y);
        }
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }

    pub fn draw(&self) {
        let mut prev = self.segments[0];
        let color = if self.vibration.abs() > 5.0 { RED } else { self.color };

        for i in 1..self.segments.len() {
            let current = self.segments[i];
            draw_line(prev.x, prev.y, current.x, current.y, 2.0, color);
            prev = current;
        }
    }
}
