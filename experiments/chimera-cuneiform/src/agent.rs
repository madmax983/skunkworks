use crate::tablet::Tablet;
use chimera_lang::prelude::*;
use rand::Rng;

pub struct Agent {
    pub id: u64,
    pub pos: (usize, usize),
    pub energy: f32,
    pub vm: ChimeraVM,
    pub dna: Dna,
}

impl Agent {
    pub fn new(id: u64, width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let dna = generate_random_dna(&mut rng, 32);
        let vm = ChimeraVM::new(dna.clone());

        Self {
            id,
            pos: (rng.gen_range(0..width), rng.gen_range(0..height)),
            energy: 100.0,
            vm,
            dna,
        }
    }

    pub fn step(&mut self, tablet: &mut Tablet) {
        if self.energy <= 0.0 {
            return;
        }

        // Execute VM for a few ticks
        for _ in 0..10 {
            self.vm.step();
        }

        // Check output
        if let Some(val) = self.vm.stack.pop() {
            match val {
                Value::Int(n) => {
                    let cmd = (n.abs() as usize) % 64;
                    match cmd {
                        0 => self.pos.1 = self.pos.1.saturating_sub(1), // N
                        1 => self.pos.1 = (self.pos.1 + 1).min(tablet.height - 1), // S
                        2 => self.pos.0 = (self.pos.0 + 1).min(tablet.width - 1), // E
                        3 => self.pos.0 = self.pos.0.saturating_sub(1), // W
                        4..=63 => {
                            let write_val = (cmd - 4) as u8;
                            if tablet.write(self.pos.0, self.pos.1, write_val, self.id) {
                                self.energy -= 2.0; // Writing costs energy
                            }
                        }
                        _ => {}
                    }
                }
                _ => {} // Ignore non-numbers
            }
        }

        self.energy -= 0.1; // Metabolic cost
    }
}

pub fn generate_random_dna(rng: &mut impl Rng, len: usize) -> Dna {
    let mut genes = Vec::new();
    for _ in 0..len {
        let op_type = rng.gen_range(0..5);
        let (op, args) = match op_type {
            0 => (
                OpCode::Push,
                vec![Nucleotide::Number(rng.gen_range(0..100))],
            ),
            1 => (OpCode::Add, vec![]),
            2 => (OpCode::Sub, vec![]),
            3 => (
                OpCode::Jump,
                vec![Nucleotide::Number(rng.gen_range(0..len as i64))],
            ),
            4 => (OpCode::Dup, vec![]),
            _ => (OpCode::Nop, vec![]),
        };
        genes.push(Gene { op, args });
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

pub fn mutate_dna(dna: &Dna, rng: &mut impl Rng) -> Dna {
    let mut new_dna = dna.clone();
    if let Some(strand) = new_dna.helix.strands.first_mut() {
        if !strand.genes.is_empty() {
            let idx = rng.gen_range(0..strand.genes.len());
            // 50% chance to change Op, 50% chance to change Arg
            if rng.gen_bool(0.5) {
                let op_type = rng.gen_range(0..5);
                let op = match op_type {
                    0 => OpCode::Push,
                    1 => OpCode::Add,
                    2 => OpCode::Sub,
                    3 => OpCode::Jump,
                    4 => OpCode::Dup,
                    _ => OpCode::Nop,
                };
                strand.genes[idx].op = op;
            } else if !strand.genes[idx].args.is_empty() {
                match &mut strand.genes[idx].args[0] {
                    Nucleotide::Number(n) => *n = rng.gen_range(0..100),
                    _ => {}
                }
            }
        }
    }
    new_dna
}
