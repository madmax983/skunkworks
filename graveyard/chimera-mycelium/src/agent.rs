use ::rand::prelude::*;
use ::rand::seq::SliceRandom;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

static AGENT_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: IVec2,
    pub energy: f32,
    pub age: u32,
    pub id: u64,
}

#[derive(Debug)]
pub enum AgentAction {
    Move(IVec2),
    Branch(IVec2),
    Die,
    None,
}

impl Agent {
    pub fn new(dna: Dna, pos: IVec2) -> Self {
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100; // Boost initial energy
        let id = AGENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            vm,
            pos,
            energy: 100.0,
            age: 0,
            id,
        }
    }

    pub fn random_dna() -> Dna {
        let mut rng = thread_rng();
        // Create a strand that loops and mutates its direction
        // Simple heuristic:
        // 1. Read Input (Chaos) at 0,0
        // 2. Manipulate it
        // 3. Write Output (Dir) at 0,1
        // 4. Write Output (Branch) at 0,2
        // 5. Jump to start

        // We will generate random genes for step 2.
        let mut genes = vec![
            // Push coordinates for GRead (0,0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // X
            Gene {
                op: OpCode::GRead,
                args: vec![],
            }, // Stack: [ChaosVal]
        ];

        // Random processing
        for _ in 0..5 {
            let ops = [OpCode::Add, OpCode::Sub, OpCode::Mul, OpCode::BitXor];
            let op = ops.choose(&mut rng).unwrap().clone();
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(rng.gen_range(1..10))],
            });
            genes.push(Gene { op, args: vec![] });
        }

        // Output Direction
        genes.push(Gene {
            op: OpCode::Dup,
            args: vec![],
        }); // Keep value for Branch logic
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }); // X
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        }); // Write Dir

        // Output Branch Probability
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }); // X
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        }); // Write Branch

        // Jump Loop
        genes.push(Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        });

        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    pub fn step(&mut self, chaos_cost: f32) -> AgentAction {
        // Input: Write chaos cost to grid[0][0]
        let cost_int = (chaos_cost * 10.0) as i64;
        self.vm.grid[0][0] = Value::Int(cost_int);

        // Run VM
        // Run multiple steps per tick to allow calculation
        for _ in 0..20 {
            if self.vm.energy <= 0 {
                return AgentAction::Die;
            }
            self.vm.step();
            if self.vm.halted {
                return AgentAction::Die;
            }
        }

        // Output: Read grid[0][1] (Dir), grid[0][2] (Branch)
        let dir_val = &self.vm.grid[0][1];
        let branch_val = &self.vm.grid[0][2];

        // Interpret Direction
        let dir_idx = match dir_val {
            Value::Int(i) => (i.abs() % 8) as usize,
            _ => 0,
        };

        let dirs = [
            IVec2::new(0, -1),  // N
            IVec2::new(1, -1),  // NE
            IVec2::new(1, 0),   // E
            IVec2::new(1, 1),   // SE
            IVec2::new(0, 1),   // S
            IVec2::new(-1, 1),  // SW
            IVec2::new(-1, 0),  // W
            IVec2::new(-1, -1), // NW
        ];

        let move_dir = dirs[dir_idx];

        // Interpret Branch
        let do_branch = match branch_val {
            Value::Int(i) => i.abs() % 100 < 5, // 5% chance
            _ => false,
        };

        // Energy Management
        // Moving into chaos costs energy.
        // If chaos is low (stable), cost is low.
        // chaos_cost is [1.0, 100.0].

        let move_cost = chaos_cost * 0.5;
        self.energy -= move_cost;
        self.age += 1;

        if self.energy <= 0.0 {
            return AgentAction::Die;
        }

        if do_branch && self.energy > 50.0 {
            self.energy -= 25.0; // Branch cost
            return AgentAction::Branch(self.pos + move_dir);
        }

        // Always try to move if alive
        return AgentAction::Move(self.pos + move_dir);
    }

    pub fn mutate(&mut self) {
        // Use Chimera's built-in mutation
        self.vm.mutate();
    }
}
