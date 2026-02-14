use chimera_lang::prelude::*;
use crate::chem_sim::ChemicalState;
use rand::prelude::*;

pub struct Organism {
    pub vm: ChimeraVM,
    pub x: usize,
    pub y: usize,
    pub energy: f32,
    pub id: u64,
}

impl Organism {
    pub fn new(x: usize, y: usize, id: u64) -> Self {
        // Create simple DNA: Wander and Eat
        // Loop:
        //   Push(Random(-1..1)) -> DX
        //   Push(0) -> Y
        //   Push(0) -> X
        //   GWrite
        //   Push(Random(-1..1)) -> DY
        //   Push(0) -> Y
        //   Push(1) -> X
        //   GWrite
        //   Push(1) -> Action (Eat)
        //   Push(1) -> Y
        //   Push(0) -> X
        //   GWrite
        //   Jump(0)

        let mut genes = Vec::new();

        // 0: Read Sensor (Grid[8][8])
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }); // Y
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }); // X
        genes.push(Gene { op: OpCode::GRead, args: vec![] }); // Stack: [SensorValue]

        // 1: Check if > 20 (Food present)
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] });
        genes.push(Gene { op: OpCode::Sub, args: vec![] }); // Stack: [Sensor - 20]

        // Clear stack
        genes.push(Gene { op: OpCode::Drop, args: vec![] });

        // Let's just create a random gene sequence for movement for EACH organism.
        let mut rng = rand::thread_rng();

        let dx: i64 = rng.gen_range(-1..=1);
        let dy: i64 = rng.gen_range(-1..=1);

        // Write DX to [0,0]
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(dx)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] });
        genes.push(Gene { op: OpCode::GWrite, args: vec![] });

        // Write DY to [0,1]
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(dy)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] });
        genes.push(Gene { op: OpCode::GWrite, args: vec![] });

        // Write Action (Eat = 1) to [1,0]
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] });
        genes.push(Gene { op: OpCode::GWrite, args: vec![] });

        // Jump back to start
        genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] });

        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);

        // Enable Chaos for evolution
        vm.chaos_mode = true;

        Self {
            vm,
            x,
            y,
            energy: 100.0,
            id,
        }
    }

    pub fn tick(&mut self, world: &mut ChemicalState) {
        // 1. Sense: Read V from world (Initial Position)
        let initial_v_level = world.get_v(self.x, self.y);
        let sensor_val = (initial_v_level * 100.0) as i64;

        // Inject into VM Grid [8][8]
        self.vm.grid[8][8] = Value::Int(sensor_val);

        // 2. Think: Step VM
        for _ in 0..10 {
            self.vm.step();
            if self.vm.halted { break; }
        }

        if self.vm.halted {
            self.energy = 0.0; // Dead
            return;
        }

        // 3. Act: Read Outputs
        // DX: [0][0]
        let dx = match self.vm.grid[0][0] {
            Value::Int(n) => n.clamp(-1, 1),
            _ => 0,
        };

        // DY: [0][1]
        let dy = match self.vm.grid[0][1] {
            Value::Int(n) => n.clamp(-1, 1),
            _ => 0,
        };

        // Action: [1][0]
        let action = match self.vm.grid[1][0] {
            Value::Int(n) => n,
            _ => 0,
        };

        // Apply Movement
        if dx != 0 || dy != 0 {
            let new_x = (self.x as isize + dx as isize).rem_euclid(world.width as isize) as usize;
            let new_y = (self.y as isize + dy as isize).rem_euclid(world.height as isize) as usize;

            self.x = new_x;
            self.y = new_y;
            self.energy -= 0.1; // Movement cost
        }

        // Re-sample chemical state at NEW position for interaction
        let current_v_level = world.get_v(self.x, self.y);

        // Apply Action
        match action {
            1 => { // Eat (Remove V)
                if current_v_level > 0.0 {
                    let amount = 0.1; // Eat amount
                    world.remove_chemical(self.x, self.y, amount);
                    self.energy += amount * 50.0; // Gain energy
                }
            },
            2 => { // Secrete (Add V)
                let amount = 0.1;
                if self.energy > 10.0 {
                    world.add_chemical(self.x, self.y, amount);
                    self.energy -= 5.0; // Cost to secrete
                }
            },
            _ => {}
        }

        // Metabolism
        self.energy -= 0.05;

        // Sync VM Energy to Float Energy (rough sync)
        self.vm.energy = self.energy as i64;
    }
}
