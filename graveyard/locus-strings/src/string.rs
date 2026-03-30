use macroquad::prelude::*;

pub struct FerrousString {
    // Physics / Visuals
    pub pos: Vec2,
    pub length: f32,
    pub vibration: f32,
    pub velocity: f32,

    // Properties
    pub frequency: f32, // Hz
    pub tension: f32,   // Spring constant k
    pub decay: f32,     // Damping factor (0.0 - 1.0)

    // Magnetic output
    pub magnetic_strength: f32,
}

impl FerrousString {
    pub const fn new(pos: Vec2, length: f32, base_freq: f32) -> Self {
        // Simplified non-genetic version
        Self {
            pos,
            length,
            vibration: 0.0,
            velocity: 0.0,
            frequency: base_freq,
            tension: 300.0,
            decay: 0.99,
            magnetic_strength: 0.0,
        }
    }

    pub fn update_physics(&mut self, dt: f32) {
        let damping = 2.0;

        let acceleration = (-self.tension).mul_add(self.vibration, -(damping * self.velocity));
        self.velocity += acceleration * dt;
        self.vibration += self.velocity * dt;

        self.vibration = self.vibration.clamp(-40.0, 40.0);
        self.magnetic_strength = (self.vibration.abs() / 10.0).clamp(0.0, 1.0);
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }

    pub fn draw(&self) {
        let start = self.pos;
        let end = self.pos + vec2(0.0, self.length);

        let segments = 20;
        let step = self.length / segments as f32;

        let mut prev = start;

        let thickness = 2.0;
        let charge = (self.vibration / 20.0).clamp(-1.0, 1.0);
        let color = if charge > 0.0 {
            Color::new(1.0, 1.0 - charge, 1.0 - charge, 1.0)
        } else {
            Color::new(1.0 + charge, 1.0 + charge, 1.0, 1.0)
        };

        for i in 1..=segments {
            let y_offset = i as f32 * step;
            let ratio = y_offset / self.length;
            let shape = (std::f32::consts::PI * ratio).sin();
            let x = self.vibration.mul_add(shape, self.pos.x);
            let y = self.pos.y + y_offset;
            let current = vec2(x, y);

            draw_line(prev.x, prev.y, current.x, current.y, thickness, color);
            prev = current;
        }

        draw_text(
            &format!("{:.1}Hz", self.frequency),
            start.x - 20.0,
            end.y + 20.0,
            16.0,
            LIGHTGRAY,
        );
    }
}
