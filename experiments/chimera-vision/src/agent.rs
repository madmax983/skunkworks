use chimera_lang::prelude::*;
use glam::Vec2;
use rand::Rng;

pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Vec2,
    pub vel: Vec2,
    pub age: usize,
    pub dna_color: (u8, u8, u8),
}

impl Agent {
    pub fn new(dna: Dna, pos: Vec2) -> Self {
        let mut vm = ChimeraVM::new(dna);
        // Give initial energy
        vm.energy = 100;

        let mut rng = rand::thread_rng();
        let color = (
            rng.gen_range(50..255),
            rng.gen_range(50..255),
            rng.gen_range(50..255),
        );

        Self {
            vm,
            pos,
            vel: Vec2::ZERO,
            age: 0,
            dna_color: color,
        }
    }

    pub fn update(&mut self, _dt: f32, chaos_level: f32, in_fovea: bool, grid_width: f32, grid_height: f32) {
        let mut rng = rand::thread_rng();

        // 1. Physics & Metabolism
        if in_fovea {
            // Safety: Gain Energy, Stable Movement
            self.vm.energy = (self.vm.energy + 1).min(1000);
            self.vel *= 0.90; // High damping
        } else {
            // Danger: Lose Energy, Chaotic Movement
            // Higher chaos = faster drain
            let drain = (1.0 + chaos_level * 0.1) as i64;
            self.vm.energy = self.vm.energy.saturating_sub(drain);

            // Random kicks based on chaos
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let kick_strength = chaos_level.clamp(0.1, 5.0) * 0.5;
            let kick = Vec2::new(angle.cos(), angle.sin()) * kick_strength;
            self.vel += kick;
        }

        // Apply Velocity
        self.pos += self.vel;

        // Bounce off walls
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.vel.x *= -1.0;
        } else if self.pos.x >= grid_width {
            self.pos.x = grid_width - 0.1;
            self.vel.x *= -1.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.vel.y *= -1.0;
        } else if self.pos.y >= grid_height {
            self.pos.y = grid_height - 0.1;
            self.vel.y *= -1.0;
        }

        // 2. Evolution (Mutation)
        if !in_fovea {
            // Mutation probability scales with chaos
            // Chaos level ~ 0.0 to 10.0+
            let mut_chance = (chaos_level * 0.05).clamp(0.0, 0.5);
            if rng.gen::<f32>() < mut_chance {
                // Mutate a random gene
                self.vm.mutate();
                // Shift color slightly to visualize drift
                self.dna_color.0 = self.dna_color.0.wrapping_add(rng.gen_range(0..10));
                self.dna_color.1 = self.dna_color.1.wrapping_add(rng.gen_range(0..10));
                self.dna_color.2 = self.dna_color.2.wrapping_add(rng.gen_range(0..10));
            }
        }

        // 3. Execution
        // Run a few steps of the VM
        for _ in 0..5 {
            if self.vm.energy > 0 {
                self.vm.step();
            } else {
                break;
            }
        }

        self.age += 1;
    }

    pub fn is_dead(&self) -> bool {
        self.vm.energy <= 0
    }
}
