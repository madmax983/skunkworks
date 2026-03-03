use crate::git::Commit;
use locus::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub coupling_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub natural_freq: f64,
    pub coupling_strength: f64,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn from_commit(commit: &Commit) -> Self {
        // Derive properties from the commit hash
        // The hash is a 40-character hex string. Let's use different parts of it.
        let bytes = hex::decode(&commit.hash).unwrap_or_else(|_| vec![0; 20]);
        let mut b = bytes.iter().copied().cycle();

        let b1 = b.next().unwrap_or(0);
        let b2 = b.next().unwrap_or(0);
        let b3 = b.next().unwrap_or(0);
        let b4 = b.next().unwrap_or(0);
        let b5 = b.next().unwrap_or(0);
        let b6 = b.next().unwrap_or(0);

        // Max speed: 0.5 to 2.0 based on b1
        let max_speed = 0.5 + (b1 as f64 / 255.0) * 1.5;

        // Separation weight: 1.0 to 3.0 based on b2
        let separation_weight = 1.0 + (b2 as f64 / 255.0) * 2.0;

        // Alignment weight: 0.5 to 1.5 based on b3
        let alignment_weight = 0.5 + (b3 as f64 / 255.0) * 1.0;

        // Cohesion weight: 0.5 to 1.5 based on b4
        let cohesion_weight = 0.5 + (b4 as f64 / 255.0) * 1.0;

        // Color based on author name's length or hash
        let author_len = commit.author.len() as u8;
        let r = b5.wrapping_add(author_len);
        let g = b6;
        let b_val = b1.wrapping_add(100);

        Self {
            max_speed,
            max_force: 0.1 + (b2 as f64 / 255.0) * 0.1, // 0.1 to 0.2
            view_radius: 10.0 + (b3 as f64 / 255.0) * 15.0, // 10 to 25
            coupling_radius: 15.0 + (b4 as f64 / 255.0) * 20.0, // 15 to 35
            separation_weight,
            alignment_weight,
            cohesion_weight,
            natural_freq: 0.005 + (b5 as f64 / 255.0) * 0.02,
            coupling_strength: 0.005,
            color: Color::Rgb(r, g, b_val),
            char_representation: if b6 % 2 == 0 { '✦' } else { '•' },
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
    pub commit: Commit,
}

impl Boid {
    pub fn new(x: f64, y: f64, commit: Commit) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::from_commit(&commit);

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            dna,
            phase: rng.r#gen::<f64>(),
            flash_timer: 0,
            commit,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
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
