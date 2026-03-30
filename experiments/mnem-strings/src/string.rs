use ::rand::Rng;
use ferrous_core::Platter;
use macroquad::prelude::*;

pub struct FerrousString {
    // Physics / Visuals
    pub pos: Vec2,
    pub length: f32,
    pub vibration: f32,
    pub velocity: f32,

    // Properties (Phenotype)
    pub frequency: f32, // Hz
    pub tension: f32,   // Spring constant k
    pub decay: f32,     // Damping factor (0.0 - 1.0)

    // Ferrous Interaction
    pub magnetic_strength: f32,
}

impl FerrousString {
    pub fn new(pos: Vec2, length: f32, base_freq: f32) -> Self {
        let mut rng = ::rand::thread_rng();

        // Initial random phenotype
        let frequency = base_freq * rng.gen_range(0.8..1.2);
        let tension = rng.gen_range(200.0..400.0);
        let decay = rng.gen_range(0.98..0.995);

        Self {
            pos,
            length,
            vibration: 0.0,
            velocity: 0.0,
            frequency,
            tension,
            decay,
            magnetic_strength: 0.0,
        }
    }

    pub fn update_physics(&mut self, dt: f32, platter: &Platter, grid_scale: f32) {
        // Sample magnetic field at center of string
        let mid_x = self.pos.x;
        let mid_y = self.pos.y + self.length * 0.5;
        let gx = (mid_x / grid_scale) as usize;
        let gy = (mid_y / grid_scale) as usize;

        let field_strength = if gx < platter.width() && gy < platter.height() {
            platter.get_magnetism(gx, gy) as f32
        } else {
            0.0
        };

        // Magnetic field increases tension (stiffens the string)
        let effective_tension = self.tension + (field_strength * 200.0);
        let damping = 2.0;

        let acceleration = -effective_tension * self.vibration - damping * self.velocity;
        self.velocity += acceleration * dt;
        self.vibration += self.velocity * dt;

        // Clamp to avoid explosion
        self.vibration = self.vibration.clamp(-40.0, 40.0);

        // Update our magnetic output based on vibration
        self.magnetic_strength = (self.vibration.abs() / 10.0).clamp(0.0, 1.0);
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }
}