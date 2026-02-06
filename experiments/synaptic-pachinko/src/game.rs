use crate::physics::{Particle, PacketKind, NeuronPin, resolve_collision};
use crate::audio::{NeuronHit, Snapshot};
use tui_shared::math::Vec2;
use rand::Rng;

pub struct GameState {
    pub width: f64,
    pub height: f64,
    pub particles: Vec<Particle>,
    pub pins: Vec<NeuronPin>,
    pub neuron_voltages: Vec<f32>,
    pub mean_field: f32,
    pub score: u64,
}

impl GameState {
    pub fn new(width: f64, height: f64) -> Self {
        // Generate Pins
        let mut pins = Vec::new();
        let rows = 12;
        let cols = 16;
        let spacing_x = width / (cols as f64 + 1.0);
        let spacing_y = (height * 0.6) / rows as f64;
        let start_y = height * 0.2;

        let mut index = 0;
        for r in 0..rows {
            let offset = if r % 2 == 0 { 0.0 } else { spacing_x * 0.5 };
            for c in 0..cols {
                let x = spacing_x + (c as f64 * spacing_x) + offset;
                let y = start_y + (r as f64 * spacing_y);
                pins.push(NeuronPin::new(x, y, index));
                index += 1;
            }
        }

        Self {
            width,
            height,
            particles: Vec::new(),
            pins,
            neuron_voltages: vec![-65.0; index],
            mean_field: -65.0,
            score: 0,
        }
    }

    pub fn spawn_packet(&mut self) {
        let mut rng = rand::thread_rng();
        let x = self.width * 0.5 + rng.gen_range(-5.0..5.0);
        let kind = match rng.gen_range(0..3) {
            0 => PacketKind::Http,
            1 => PacketKind::Ssh,
            _ => PacketKind::Malware,
        };
        self.particles.push(Particle::new(x, 0.0, kind));
    }

    pub fn tick(&mut self, dt: f64, hit_tx: &crossbeam_channel::Sender<NeuronHit>) {
        let gravity = Vec2::new(0.0, 40.0);

        for p in &mut self.particles {
            p.update(dt, gravity);

            // Wall collisions
            if p.pos.x < p.radius {
                p.pos.x = p.radius;
                p.vel.x *= -0.7;
            }
            if p.pos.x > self.width - p.radius {
                p.pos.x = self.width - p.radius;
                p.vel.x *= -0.7;
            }
        }

        // Pin collisions
        for p in &mut self.particles {
            if !p.active { continue; }
            for pin in &self.pins {
                if let Some(idx) = resolve_collision(p, pin) {
                    // Send Hit Event
                    let strength = match p.kind {
                        PacketKind::Http => 10.0,
                        PacketKind::Ssh => 15.0,
                        PacketKind::Malware => 25.0,
                    };
                    let _ = hit_tx.send(NeuronHit { index: idx, strength });
                    self.score += 1;
                }
            }
        }

        // Remove out of bounds
        self.particles.retain(|p| p.pos.y < self.height + 10.0);
    }

    pub fn update_voltages(&mut self, snap: Snapshot) {
        if snap.voltages.len() == self.pins.len() {
             self.neuron_voltages = snap.voltages;
        }
        self.mean_field = snap.mean_field;
    }
}
