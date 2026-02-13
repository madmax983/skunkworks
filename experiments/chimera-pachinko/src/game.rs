use crate::physics::{ChimeraPin, PacketKind, Particle, resolve_collision};
use chimera_lang::prelude::*;
use locus::Vec2;
use rand::Rng;

pub struct GameState {
    pub width: f64,
    pub height: f64,
    pub particles: Vec<Particle>,
    pub pins: Vec<ChimeraPin>,
    pub score: u64,
    pub generation: u32,
}

impl GameState {
    pub fn new(width: f64, height: f64) -> Self {
        let mut pins = Vec::new();
        let rows = 8;
        let cols = 10;
        let spacing_x = width / (cols as f64 + 1.0);
        let spacing_y = (height * 0.7) / rows as f64;
        let start_y = height * 0.2;

        let mut index = 0;
        for r in 0..rows {
            let offset = if r % 2 == 0 { 0.0 } else { spacing_x * 0.5 };
            for c in 0..cols {
                let x = spacing_x + (c as f64 * spacing_x) + offset;
                let y = start_y + (r as f64 * spacing_y);

                let dna = random_dna();
                let pin = ChimeraPin::new(x, y, index, dna);
                pins.push(pin);
                index += 1;
            }
        }

        Self {
            width,
            height,
            particles: Vec::new(),
            pins,
            score: 0,
            generation: 0,
        }
    }

    pub fn spawn_packet(&mut self) {
        let mut rng = rand::thread_rng();
        let x = self.width * 0.5 + rng.gen_range(-10.0..10.0);
        let kind = match rng.gen_range(0..5) {
            0..=2 => PacketKind::Http, // 60%
            3 => PacketKind::Ssh,      // 20%
            _ => PacketKind::Malware,  // 20%
        };
        // Malware is smaller and faster
        let mut p = Particle::new(x, 0.0, kind);
        if let PacketKind::Malware = kind {
             p.vel.y = 10.0;
        }
        self.particles.push(p);
    }

    pub fn tick(&mut self, dt: f64) {
        let gravity = Vec2::new(0.0, 30.0);

        // Update particles
        for p in &mut self.particles {
            p.update(dt, gravity);

            // Walls
             if p.pos.x < 1.0 { p.pos.x = 1.0; p.vel.x *= -0.8; }
             if p.pos.x > self.width - 1.0 { p.pos.x = self.width - 1.0; p.vel.x *= -0.8; }
        }

        // Collisions
        let mut hits = Vec::new();

        for p in &mut self.particles {
             if !p.active { continue; }
             for pin in &mut self.pins {
                 if let Some(_) = resolve_collision(p, pin) {
                     // Hit!
                     hits.push((pin.index, p.energy_value));
                     self.score += 1;
                 }
             }
        }

        // Process Hits (VM Execution)
        for (idx, energy) in hits {
            if let Some(pin) = self.pins.get_mut(idx) {
                // Transfer energy
                pin.vm.energy = pin.vm.energy.saturating_add(energy);
                if pin.vm.energy < 0 { pin.vm.energy = 0; }

                if energy > 0 {
                    // Execute VM step if hit by good packet
                    // Run a few steps
                    for _ in 0..5 {
                        let _ = pin.vm.step();
                    }

                    if pin.vm.energy > 100 {
                        pin.vm.energy -= 20; // Cost of reproduction/improvement
                        // Mutate pos slightly to catch more
                        let nudge_x = rand::thread_rng().gen_range(-0.5..0.5);
                         pin.pos.x += nudge_x;
                    }
                } else {
                     // Malware hit!
                     // pin.vm.energy already reduced
                }
            }
        }

        // Remove dead pins & Re-populate
        let mut new_dnas = Vec::new();
        let pin_count = self.pins.len();

        for i in 0..pin_count {
            if self.pins[i].vm.energy == 0 {
                // Die. Find a rich neighbor.
                let mut best_dna = None;
                let mut max_energy = 0;

                // Look at neighbors (simple scan)
                for j in 0..pin_count {
                     if i == j { continue; }
                     let dist = (self.pins[i].pos - self.pins[j].pos).magnitude();
                     if dist < 10.0 && self.pins[j].vm.energy > 50 {
                         if self.pins[j].vm.energy > max_energy {
                             max_energy = self.pins[j].vm.energy;
                             // Clone their DNA
                             best_dna = Some(self.pins[j].vm.dna.clone());
                         }
                     }
                }

                if let Some(dna) = best_dna {
                    new_dnas.push((i, dna));
                } else {
                    // Random respawn
                    new_dnas.push((i, random_dna()));
                }
            } else {
                // Metabolic cost
                // pin.vm.energy = pin.vm.energy.saturating_sub(1);
            }
        }

        for (idx, dna) in new_dnas {
            self.pins[idx].vm = ChimeraVM::new(dna);
            self.pins[idx].vm.energy = 50;
            self.pins[idx].generation += 1;
            // Reset color/stats?
        }

        // Cleanup particles
        self.particles.retain(|p| p.pos.y < self.height + 10.0);
    }
}

fn random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let ops = vec![
        OpCode::Push, OpCode::Add, OpCode::Sub, OpCode::Photosynthesize,
        OpCode::GRead, OpCode::GWrite, OpCode::Nop, OpCode::Dup
    ];

    let mut genes = Vec::new();
    for _ in 0..5 {
        let op_idx = rng.gen_range(0..ops.len());
        let op = ops[op_idx].clone();
        let args = if matches!(op, OpCode::Push) {
            vec![Nucleotide::Number(rng.gen_range(0..10))]
        } else {
            vec![]
        };
        genes.push(Gene { op, args });
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }]
        }
    }
}
