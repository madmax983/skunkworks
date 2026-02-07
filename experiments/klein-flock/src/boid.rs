use glam::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f32::consts::TAU;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f32,
    pub max_force: f32,
    pub view_radius: f32,
    pub coupling_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub natural_freq: f32,
    pub coupling_strength: f32,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.05..0.15), // Adjusted for UV space (0..2PI)
            max_force: rng.gen_range(0.005..0.02),
            view_radius: rng.gen_range(0.5..1.5), // In 3D distance units
            coupling_radius: rng.gen_range(1.0..2.0),
            separation_weight: rng.gen_range(1.2..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            natural_freq: 0.005 + rng.gen::<f32>() * 0.02,
            coupling_strength: 0.005,
            color: Color::Indexed(rng.gen_range(20..230)),
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec2, // u, v in [0, 2PI)
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: Dna,
    pub phase: f32,
    pub flash_timer: usize,
}

impl Boid {
    pub fn new(u: f32, v: f32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::random();

        Self {
            position: Vec2::new(u, v),
            velocity: Vec2::new(angle.cos(), angle.sin()) * dna.max_speed,
            acceleration: Vec2::ZERO,
            dna,
            phase: rng.gen::<f32>(),
            flash_timer: 0,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.dna.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;
    }

    pub fn update_flash(&mut self) {
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.flash_timer = 5;
        }
    }
}
