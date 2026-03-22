use crate::grid::{AcousticGrid4D, Point4D, GRID_SIZE};
use crate::monitor::SystemMonitor;
use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use std::collections::VecDeque;

pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Point4D,
    pub bio_energy: f32,
    pub color: Color,
    pub age: u32,
    pub id: u64,
}

impl Agent {
    pub fn new_random(id: u64) -> Self {
        let mut rng = ::rand::thread_rng();
        // Generate random simple program
        let mut genes = Vec::new();
        for _ in 0..24 {
            let op = match rng.gen_range(0..10) {
                0 => OpCode::Push,
                1 => OpCode::Drop,
                2 => OpCode::Add,
                3 => OpCode::Sub,
                4 => OpCode::Migrate,
                5 => OpCode::Signal,  // Output (Pluck/Dampen)
                6 => OpCode::Receive, // Input (Listen)
                7 => OpCode::Jump,
                8 => OpCode::Brz,
                _ => OpCode::Nop,
            };

            let args = if op == OpCode::Push || op == OpCode::Jump || op == OpCode::Brz {
                vec![Nucleotide::Number(rng.gen_range(0..10))]
            } else {
                vec![]
            };

            genes.push(Gene { op, args });
        }

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 500;

        Self {
            vm,
            pos: Point4D::new(
                rng.gen_range(1..GRID_SIZE - 1),
                rng.gen_range(1..GRID_SIZE - 1),
                rng.gen_range(1..GRID_SIZE - 1),
                rng.gen_range(1..GRID_SIZE - 1),
            ),
            bio_energy: 100.0,
            color: GREEN,
            age: 0,
            id,
        }
    }

    pub fn clone_mutate(&self, new_id: u64) -> Self {
        let mut vm = ChimeraVM::new(self.vm.dna.clone());
        vm.energy = 500;

        let mut rng = ::rand::thread_rng();
        if rng.gen_bool(0.1) {
            if let Some(strand) = vm.dna.helix.strands.get_mut(0) {
                if !strand.genes.is_empty() {
                    let idx = rng.gen_range(0..strand.genes.len());
                    // Mutate an op to Signal or Receive to encourage interaction
                    if rng.gen_bool(0.5) {
                        strand.genes[idx].op = OpCode::Signal;
                    } else {
                        strand.genes[idx].op = OpCode::Receive;
                    }
                }
            }
        }

        Self {
            vm,
            pos: self.pos,
            bio_energy: 50.0,
            color: self.color,
            age: 0,
            id: new_id,
        }
    }

    pub fn update(&mut self, grid: &mut AcousticGrid4D, _monitor: &SystemMonitor) {
        if self.bio_energy <= 0.0 {
            self.color = GRAY;
            return;
        }
        self.age += 1;
        self.vm.energy = 500; // Replenish Brain Energy

        // 1. Sense: Read Pressure
        let idx = grid.idx(self.pos);
        let pressure = grid.u[idx];

        // Push to Ether Channel 0 (Input)
        let sensor_val = (pressure * 100.0) as i64;
        self.vm
            .ether
            .entry(0)
            .or_insert(VecDeque::new())
            .push_back(Value::Int(sensor_val));

        // Also limit the queue size to prevent memory leak if agent ignores it
        if let Some(queue) = self.vm.ether.get_mut(&0) {
            while queue.len() > 10 {
                queue.pop_front();
            }
        }

        // 2. Run VM
        let old_loc = self.vm.context_loc;
        self.vm.step();
        let new_loc = self.vm.context_loc;

        // 3. Act
        let dy = (new_loc.0 as isize - old_loc.0 as isize);
        let dx = (new_loc.1 as isize - old_loc.1 as isize);

        let mut dz: isize = 0;
        let mut dw: isize = 0;
        let mut pluck_intent = 0.0;
        let mut dampen_intent = 0.0;

        // Channel 1: Move Z/W
        if let Some(queue) = self.vm.ether.get_mut(&1) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    match v.abs() % 4 {
                        0 => dz -= 1,
                        1 => dz += 1,
                        2 => dw -= 1,
                        3 => dw += 1,
                        _ => {}
                    }
                }
            }
        }

        // Channel 2: Pluck (Sing)
        if let Some(queue) = self.vm.ether.get_mut(&2) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    pluck_intent += (v as f32) / 10.0;
                }
            }
        }

        // Channel 3: Eat (Dampen)
        if let Some(queue) = self.vm.ether.get_mut(&3) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    dampen_intent += (v.abs() as f32) / 100.0;
                }
            }
        }

        // Apply Move
        let mut new_x = self.pos.x as isize + dx;
        let mut new_y = self.pos.y as isize + dy;
        let mut new_z = self.pos.z as isize + dz;
        let mut new_w = self.pos.w as isize + dw;

        // Constrain
        new_x = new_x.clamp(1, (GRID_SIZE - 2) as isize);
        new_y = new_y.clamp(1, (GRID_SIZE - 2) as isize);
        new_z = new_z.clamp(1, (GRID_SIZE - 2) as isize);
        new_w = new_w.clamp(1, (GRID_SIZE - 2) as isize);

        let new_pos = Point4D::new(
            new_x as usize,
            new_y as usize,
            new_z as usize,
            new_w as usize,
        );

        if new_pos != self.pos {
            self.bio_energy -= 1.0; // Move cost
            self.pos = new_pos;
        }

        // Action: Pluck (Sing)
        if pluck_intent.abs() > 0.1 {
            grid.pluck(self.pos, pluck_intent);
            self.bio_energy -= pluck_intent.abs() * 2.0;
            self.color = GOLD;
        }

        // Action: Eat (Dampen)
        if dampen_intent > 0.0 {
            let factor = (1.0 - dampen_intent).max(0.0);
            grid.set_agent_damping(self.pos, factor);

            let energy_gain = pressure.abs() * dampen_intent * 5.0;
            self.bio_energy += energy_gain;

            self.color = BLUE;
        } else {
            grid.set_agent_damping(self.pos, 1.0);

            let e_norm = (self.bio_energy / 200.0).clamp(0.0, 1.0);
            self.color = Color::new(e_norm, 0.0, e_norm, 1.0);
        }

        self.bio_energy -= 0.1;
    }
}
