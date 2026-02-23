use chimera_lang::prelude::*;
use rand::Rng;

pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: (usize, usize),
    pub id: u64,
}

impl Agent {
    pub fn new(pos: (usize, usize), id: u64) -> Self {
        let mut rng = rand::thread_rng();

        // Randomly choose behavior
        let dna = if rng.gen_bool(0.5) {
             // "Builder": Adds sand (Radiate value 1)
             vec![
                 Gene { op: OpCode::Photosynthesize, args: vec![] },
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Val
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Radius
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // Y (Center)
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // X (Center)
                 Gene { op: OpCode::Radiate, args: vec![] },
                 Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
             ]
        } else {
             // "Eater": Removes sand (Siphon)
             vec![
                 Gene { op: OpCode::Photosynthesize, args: vec![] },
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Radius
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // Y
                 Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // X
                 Gene { op: OpCode::Siphon, args: vec![] },
                 Gene { op: OpCode::Drop, args: vec![] }, // Discard result
                 Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
             ]
        };

        let helix = Helix { strands: vec![Strand { genes: dna }] };
        let dna_struct = Dna { evolution_config: None, helix };

        Self {
            vm: ChimeraVM::new(dna_struct),
            pos,
            id,
        }
    }

    /// Syncs the local sand grid into the agent's memory.
    pub fn sync_input(&mut self, grid_view: &[[u32; 16]; 16]) {
        for y in 0..16 {
            for x in 0..16 {
                self.vm.grid[y][x] = Value::Int(grid_view[y][x] as i64);
            }
        }
    }

    /// Runs one step of the VM.
    pub fn step(&mut self) {
        self.vm.step();
    }

    /// Reads the agent's memory to see what changes it wants to make to the sand grid.
    /// Returns a list of (local_x, local_y, delta_sand).
    pub fn sync_output(&self, original_view: &[[u32; 16]; 16]) -> Vec<(usize, usize, i32)> {
        let mut changes = Vec::new();
        for y in 0..16 {
            for x in 0..16 {
                if let Value::Int(val) = self.vm.grid[y][x] {
                    let old_val = original_view[y][x] as i64;
                    if val != old_val {
                        let delta = val - old_val;
                        changes.push((x, y, delta as i32));
                    }
                }
            }
        }
        changes
    }
}
