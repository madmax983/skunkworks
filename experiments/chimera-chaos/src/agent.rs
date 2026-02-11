use chimera_lang::prelude::*;
use crate::lattice::Lattice;
use ::rand::Rng;

pub struct Agent {
    pub id: usize,
    pub vm: ChimeraVM,
    pub x: usize,
    pub y: usize,
    pub energy: f32,
    pub age: usize,
    pub color: (f32, f32, f32), // RGB
}

impl Agent {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        let mut rng = ::rand::thread_rng();

        // DNA Strategy: Random Walk
        // 1. Get Random Number (grid[0][3]) -> Rand (0-1000)
        // 2. DX = Rand - 500
        // 3. Write DX to grid[1][0]
        // 4. Get Random Number again
        // 5. DY = Rand - 500
        // 6. Write DY to grid[1][1]

        let genes = vec![
            // --- DX ---
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] }, // x=3
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // y=0
            Gene { op: OpCode::GRead, args: vec![] }, // [Rand]

            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(500)] },
            Gene { op: OpCode::Sub, args: vec![] }, // [Rand - 500]

            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // x=0
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // y=1
            Gene { op: OpCode::GWrite, args: vec![] }, // Write DX

            // --- DY ---
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] }, // x=3
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // y=0
            Gene { op: OpCode::GRead, args: vec![] }, // [Rand]

            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(500)] },
            Gene { op: OpCode::Sub, args: vec![] }, // [Rand - 500]

            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // x=1
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // y=1
            Gene { op: OpCode::GWrite, args: vec![] }, // Write DY

            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Loop
        ];

        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        Self {
            id,
            vm,
            x,
            y,
            energy: 100.0,
            age: 0,
            color: (rng.gen(), rng.gen(), rng.gen()),
        }
    }

    pub fn update(&mut self, lattice: &mut Lattice) {
        self.age += 1;
        self.energy -= 0.1; // Metabolic cost

        // 1. Update Sensors (Input to VM)
        let val = lattice.get_val(self.x, self.y);
        let r_val = lattice.get_r(self.x, self.y);

        let mut rng = ::rand::thread_rng();

        // Map to integer scale for VM (0-100)
        self.vm.grid[0][0] = Value::Int((val * 100.0) as i64);
        self.vm.grid[0][1] = Value::Int((r_val * 25.0) as i64); // r is 0-4 -> 0-100
        self.vm.grid[0][2] = Value::Int(self.energy as i64);
        self.vm.grid[0][3] = Value::Int(rng.gen_range(0..1000)); // Random input

        // 2. Step VM
        // Execute a few instructions
        for _ in 0..20 {
            self.vm.step();
        }

        // 3. Read Actuators (Output from VM)
        // Move X: grid[1][0]
        let dx = match self.vm.grid[1][0] {
            Value::Int(n) => n,
            _ => 0,
        };
        // Move Y: grid[1][1]
        let dy = match self.vm.grid[1][1] {
            Value::Int(n) => n,
            _ => 0,
        };
        // Modify R: grid[1][2]
        let mod_r = match self.vm.grid[1][2] {
            Value::Int(n) => n,
            _ => 0,
        };

        // 4. Act
        // Movement
        // Correct casting: dx is i64. signum() returns i64 (-1, 0, 1).
        // self.x is usize.
        let move_x = dx.signum();
        let move_y = dy.signum();

        let width = lattice.width as isize;
        let height = lattice.height as isize;

        let mut next_x = (self.x as isize + move_x as isize);
        let mut next_y = (self.y as isize + move_y as isize);

        // Wrap
        next_x = next_x.rem_euclid(width);
        next_y = next_y.rem_euclid(height);

        self.x = next_x as usize;
        self.y = next_y as usize;

        // Modify Environment
        if mod_r != 0 {
            let current_r = lattice.get_r(self.x, self.y);
            let change = (mod_r as f32) * 0.001;
            lattice.set_r(self.x, self.y, current_r + change);
            self.energy -= 0.5; // Cost of modification
        }

        // 5. Feed
        let harvest = lattice.get_val(self.x, self.y) * 0.5;
        self.energy += harvest;

        // Capping energy
        if self.energy > 200.0 {
            self.energy = 200.0;
        }

        // Update Color based on state
        self.color.0 = (self.energy / 200.0).clamp(0.0, 1.0);
        self.color.1 = (lattice.get_r(self.x, self.y) / 4.0).clamp(0.0, 1.0);
    }
}
