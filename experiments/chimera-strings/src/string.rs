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

    // Genetics
    pub fitness: f32,
    pub is_mutating: bool,

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
            fitness: 0.0,
            is_mutating: false,
            magnetic_strength: 0.0,
        }
    }

    pub fn update_physics(&mut self, dt: f32, platter: &Platter, grid_scale: f32) {
        // Damped spring simulation
        // F = -kx - cv
        // PLUS magnetic force from Platter

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
        // Higher vibration = stronger magnetic pulse
        self.magnetic_strength = (self.vibration.abs() / 10.0).clamp(0.0, 1.0);
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }

    pub fn evolve(&mut self, base_freq: f32) {
        // If high fitness, don't mutate (Elitism)
        if self.fitness > 0.8 {
            self.is_mutating = false;
            return;
        }

        self.is_mutating = true;

        let mut rng = ::rand::thread_rng();
        let mutation_rate = (1.0 - self.fitness).max(0.1) * 0.1;

        // Mutate Frequency
        let f_change = rng.gen_range(-mutation_rate..mutation_rate) * base_freq;
        self.frequency = (self.frequency + f_change).clamp(base_freq * 0.5, base_freq * 2.0);

        // Mutate Tension
        let t_change = rng.gen_range(-mutation_rate..mutation_rate) * 100.0;
        self.tension = (self.tension + t_change).clamp(50.0, 600.0);

        // Mutate Decay
        let d_change = rng.gen_range(-mutation_rate..mutation_rate) * 0.01;
        self.decay = (self.decay + d_change).clamp(0.95, 0.999);
    }

    pub fn draw(&self) {
        let start = self.pos;
        let end = self.pos + vec2(0.0, self.length);

        // Draw segment based
        let segments = 20;
        let step = self.length / segments as f32;

        let mut prev = start;

        let thickness = 2.0;

        // Visualize vibration intensity/magnetic charge
        let charge = (self.vibration / 20.0).clamp(-1.0, 1.0);
        // Red = N, Blue = S
        let color = if charge > 0.0 {
            Color::new(1.0, 1.0 - charge, 1.0 - charge, 1.0)
        } else {
            Color::new(1.0 + charge, 1.0 + charge, 1.0, 1.0)
        };

        for i in 1..=segments {
            let y_offset = i as f32 * step;
            let ratio = y_offset / self.length;
            let shape = (std::f32::consts::PI * ratio).sin();
            let x = self.pos.x + self.vibration * shape;
            let y = self.pos.y + y_offset;
            let current = vec2(x, y);

            draw_line(prev.x, prev.y, current.x, current.y, thickness, color);
            prev = current;
        }

        // Draw tuning pegs (Agents)
        if self.is_mutating {
            draw_circle(start.x, start.y - 10.0, 4.0, RED);
        } else {
            draw_circle(start.x, start.y - 10.0, 4.0, GREEN);
        }

        // Draw frequency text
        draw_text(
            format!("{:.1}Hz", self.frequency).as_str(),
            start.x - 20.0,
            end.y + 20.0,
            16.0,
            LIGHTGRAY,
        );

        // Fitness
        draw_text(
            format!("Fit: {:.2}", self.fitness).as_str(),
            start.x - 20.0,
            end.y + 35.0,
            12.0,
            GRAY,
        );
    }
}
