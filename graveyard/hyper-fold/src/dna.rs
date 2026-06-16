use chimera_lang::ast::{Dna, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};
use hyper_system::math::Vec4;
use hyper_system::physics::Particle4D;
use rand::Rng;

pub struct ChimeraAgent {
    pub vm: ChimeraVM,
    pub index: usize,
}

impl ChimeraAgent {
    pub fn new(index: usize, _seed: u64) -> Self {
        // Simple DNA:
        // 1. Read Strain from Grid(0, 2)
        // 2. Multiply by 2
        // 3. Leave on stack as output

        // Genes:
        // push(2) // X=2 (Strain)
        // push(0) // Y=0
        // g_read() // -> [Strain]
        // push(2)
        // mul()    // -> [Strain * 2]

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::GRead,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Mul,
                args: vec![],
            },
        ];

        let dna = Dna::from_genes(genes);
        let vm = ChimeraVM::new(dna);

        Self { vm, index }
    }

    pub fn update(&mut self, particles: &[Particle4D], grid_size: (usize, usize)) -> Vec4 {
        // Read sensors
        let p = &particles[self.index];
        let w = p.pos.w;

        // Neighbor average W
        let (cols, rows) = grid_size;
        let x = self.index % (cols + 1);
        let y = self.index / (cols + 1);

        let mut neighbor_w_sum = 0.0;
        let mut count = 0;

        let idx = self.index as i32;
        let stride = (cols + 1) as i32;

        let neighbors = [
            if x > 0 { Some(idx - 1) } else { None },
            if x < cols { Some(idx + 1) } else { None },
            if y > 0 { Some(idx - stride) } else { None },
            if y < rows { Some(idx + stride) } else { None },
        ];

        for n_idx in neighbors.iter().flatten() {
            if let Some(np) = particles.get(*n_idx as usize) {
                neighbor_w_sum += np.pos.w;
                count += 1;
            }
        }

        let avg_w = if count > 0 {
            neighbor_w_sum / count as f32
        } else {
            0.0
        };
        let strain = (w - avg_w).abs();

        // Write inputs to Grid
        // (0,0) = W
        // (0,1) = AvgW
        // (0,2) = Strain
        self.vm.grid[0][0] = Value::Int((w * 100.0) as i64);
        self.vm.grid[0][1] = Value::Int((avg_w * 100.0) as i64);
        self.vm.grid[0][2] = Value::Int((strain * 100.0) as i64);

        // Reset VM state for new tick (or keep running? Usually reset for function-like behavior)
        self.vm.ip = (0, 0);
        self.vm.stack.clear();
        self.vm.halted = false;
        self.vm.energy = 50; // Recharge

        // Run a few steps
        for _ in 0..10 {
            if self.vm.halted {
                break;
            }
            self.vm.step();
        }

        // Output: Top of stack
        let output = if let Some(val) = self.vm.stack.last() {
            match val {
                Value::Int(i) => (*i as f32) / 100.0,
                _ => 0.0,
            }
        } else {
            0.0
        };

        // Add some random noise to start folding if flat
        let mut rng = rand::thread_rng();
        let noise = rng.gen_range(-0.1..0.1);

        let polarity_w = (output + noise).clamp(-2.0, 2.0);

        Vec4::new(0.0, 0.0, 0.0, polarity_w)
    }
}
