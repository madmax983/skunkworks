use chimera_lang::vm::{ChimeraVM, Chirality};
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use glam::Vec2;
use rand::Rng;

pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Vec2,
    pub vel: Vec2,
    pub chirality: Chirality,
    pub color: (u8, u8, u8),
}

impl Agent {
    pub fn new(pos: Vec2) -> Self {
        let mut rng = rand::thread_rng();

        // Genome:
        // 1. Push(dy)
        // 2. Push(dx)
        // 3. Jump(0) (Loop)
        // Values mutated by chaos
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(-10..=10))] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(-10..=10))] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);
        vm.chaos_mode = true; // Enable mutation

        // Give infinite energy for movement simulation
        vm.energy = 1000;

        Self {
            vm,
            pos,
            vel: Vec2::ZERO,
            chirality: Chirality::Left,
            color: (rng.gen(), rng.gen(), rng.gen()),
        }
    }

    pub fn update(&mut self) -> Vec2 {
        // Sync Chirality to VM
        self.vm.chirality = self.chirality;

        // Run VM
        self.vm.step();

        // Refill energy
        if self.vm.energy < 10 {
            self.vm.energy = 100;
        }

        // Check Stack for Motor Output
        // If stack has >= 2 items, we peek/pop
        let mut force = Vec2::ZERO;

        if self.vm.stack.len() >= 2 {
             // We pop to consume the "impulse"
             if let Some(chimera_lang::vm::Value::Int(dy)) = self.vm.stack.pop() {
                 if let Some(chimera_lang::vm::Value::Int(dx)) = self.vm.stack.pop() {
                     // Scale down
                     force = Vec2::new(dx as f32 * 0.001, dy as f32 * 0.001);
                 }
             }
        }

        // Apply limit
        force.clamp_length_max(0.05)
    }

    pub fn flip_chirality(&mut self) {
        self.chirality = match self.chirality {
            Chirality::Left => Chirality::Right,
            Chirality::Right => Chirality::Left,
        };
        // Color shift to indicate flip
        self.color = (255 - self.color.0, 255 - self.color.1, 255 - self.color.2);
    }
}
