use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;
use locus::Vec2;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub coupling_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub fissure_attraction_weight: f64, // New trait: Attraction to fissures
    pub natural_freq: f64,
    pub coupling_strength: f64,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.8..1.5),
            max_force: rng.gen_range(0.05..0.15),
            view_radius: rng.gen_range(10.0..20.0),
            coupling_radius: rng.gen_range(15.0..30.0),
            separation_weight: rng.gen_range(1.2..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            fissure_attraction_weight: rng.gen_range(0.5..2.0), // Some boids love disaster
            natural_freq: 0.005 + rng.r#gen::<f64>() * 0.02,
            coupling_strength: 0.005,
            color: Color::Indexed(rng.gen_range(20..230)),
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: Dna,
    pub phase: f64,
    pub flash_timer: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::random();

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            dna,
            phase: rng.r#gen::<f64>(),
            flash_timer: 0,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;

        // Limit speed
        self.velocity = self.velocity.limit(self.dna.max_speed);

        self.position += self.velocity;

        // Reset acceleration
        self.acceleration = Vec2::zero();

        // Wrap around edges
        if self.position.x < 0.0 {
            self.position.x += width;
        }
        if self.position.x >= width {
            self.position.x -= width;
        }
        if self.position.y < 0.0 {
            self.position.y += height;
        }
        if self.position.y >= height {
            self.position.y -= height;
        }
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
