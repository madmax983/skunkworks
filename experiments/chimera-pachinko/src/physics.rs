use chimera_lang::prelude::*;
use locus::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketKind {
    Http,    // Green
    Ssh,     // Blue
    Malware, // Red
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub kind: PacketKind,
    pub radius: f64,
    pub active: bool,
    pub energy_value: i64,
}

impl Particle {
    pub fn new(x: f64, y: f64, kind: PacketKind) -> Self {
        let energy_value = match kind {
            PacketKind::Http => 10,
            PacketKind::Ssh => 25,
            PacketKind::Malware => -50, // Malware hurts!
        };
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0.0, 0.0),
            kind,
            radius: 0.5,
            active: true,
            energy_value,
        }
    }

    pub fn update(&mut self, dt: f64, gravity: Vec2) {
        if !self.active {
            return;
        }
        self.vel = self.vel + gravity * dt;
        self.pos = self.pos + self.vel * dt;
        self.vel = self.vel * 0.99; // Friction
    }
}

pub struct ChimeraPin {
    pub pos: Vec2,
    pub radius: f64,
    pub index: usize,
    pub vm: ChimeraVM,
    pub generation: u32,
}

impl ChimeraPin {
    pub fn new(x: f64, y: f64, index: usize, dna: Dna) -> Self {
        let mut vm = ChimeraVM::new(dna);
        // Give initial energy so they don't die instantly
        vm.energy = 50;
        Self {
            pos: Vec2::new(x, y),
            radius: 1.0,
            index,
            vm,
            generation: 0,
        }
    }
}

/// Resolves collision between particle and pin.
/// Returns Some(pin_index) if collision occurred.
pub fn resolve_collision(particle: &mut Particle, pin: &ChimeraPin) -> Option<usize> {
    let diff = particle.pos - pin.pos;
    let dist_sq = diff.magnitude_squared();
    let min_dist = particle.radius + pin.radius;

    if dist_sq < min_dist * min_dist {
        // Collision!
        let dist = dist_sq.sqrt();
        let normal = if dist == 0.0 {
            Vec2::new(0.0, 1.0)
        } else {
            diff * (1.0 / dist)
        };

        let overlap = min_dist - dist;
        particle.pos = particle.pos + normal * overlap;

        let restitution = 0.8;
        let v_dot_n = particle.vel.dot(normal);

        if v_dot_n < 0.0 {
            let j = -(1.0 + restitution) * v_dot_n;
            particle.vel = particle.vel + normal * j;
            return Some(pin.index);
        }
    }
    None
}
