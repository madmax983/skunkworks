use crate::grid::SpinGrid4D;
use crate::math::Vec4D;
use crate::monitor::SystemMonitor;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use ::rand::Rng; // Import Rng trait
use std::collections::VecDeque;

pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Vec4D,
    pub bio_energy: f32,
    pub color: Color,
    pub age: u32,
}

impl Agent {
    pub fn new_random() -> Self {
        // Generate random DNA
        let mut genes = Vec::new();
        let mut rng = ::rand::thread_rng();

        // Random simple program
        for _ in 0..20 {
            let op = match rng.gen_range(0..10) {
                0 => OpCode::Push,
                1 => OpCode::Drop,
                2 => OpCode::Add,
                3 => OpCode::Sub,
                4 => OpCode::Migrate, // Move X/Y
                5 => OpCode::Signal,  // Move Z/W or Flip
                6 => OpCode::Listen,
                7 => OpCode::Jump,
                8 => OpCode::Brz,
                _ => OpCode::Nop, // No-op
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
        vm.energy = 1000;

        Self {
            vm,
            pos: Vec4D::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            bio_energy: 100.0,
            color: GREEN,
            age: 0,
        }
    }

    pub fn clone_mutate(&self) -> Self {
        // Clone with mutation
        let dna = self.vm.dna.clone();
        // Mutate DNA logic (simplified: perfect clone for now)

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 500;

        Self {
            vm,
            pos: self.pos, // Spawn at parent
            bio_energy: 50.0, // Start with some energy
            color: self.color,
            age: 0,
        }
    }

    pub fn update(&mut self, grid: &mut SpinGrid4D, monitor: &SystemMonitor) {
        if self.bio_energy <= 0.0 {
            self.color = GRAY;
            return;
        }
        self.age += 1;

        // 1. Sense Environment
        // Map 4D pos to Grid Index
        let x_idx = (self.pos.x.rem_euclid(1.0) * grid.width as f32) as usize;
        let y_idx = (self.pos.y.rem_euclid(1.0) * grid.height as f32) as usize;
        let z_idx = (self.pos.z.rem_euclid(1.0) * grid.depth as f32) as usize;
        let w_idx = (self.pos.w.rem_euclid(1.0) * grid.hypersize as f32) as usize;
        let idx = grid.idx(x_idx, y_idx, z_idx, w_idx);

        // Input: Local Spin
        let theta = grid.get_spin(idx);
        let spin_val = ((theta / (2.0 * std::f32::consts::PI)) * 255.0) as i64;

        // Push to Ether Channel 0 (Simulated Input)
        self.vm.ether.entry(0).or_insert(VecDeque::new()).push_back(Value::Int(spin_val));

        // 2. Run VM
        let old_loc = self.vm.context_loc;
        self.vm.step();
        let new_loc = self.vm.context_loc;

        // 3. Act
        // Movement (X, Y) from Migrate
        let dy = (new_loc.0 as isize - old_loc.0 as isize) as f32;
        let dx = (new_loc.1 as isize - old_loc.1 as isize) as f32;

        // Movement (Z, W) and Flip from Signal
        let mut dz: f32 = 0.0;
        let mut dw: f32 = 0.0;
        let mut flip_intent = 0.0;

        // Read signals
        // Channel 1: Z/W Move
        if let Some(queue) = self.vm.ether.get_mut(&1) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    match v % 4 {
                        0 => dz -= 1.0,
                        1 => dz += 1.0,
                        2 => dw -= 1.0,
                        3 => dw += 1.0,
                        _ => {}
                    }
                }
            }
        }

        // Channel 2: Flip Spin
        if let Some(queue) = self.vm.ether.get_mut(&2) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    // Map int to angle delta
                    flip_intent += v as f32 / 10.0;
                }
            }
        }

        // Apply Movement with Relativistic Cost
        // Distortion based on System Metrics
        let sx = 1.0 + monitor.cpu_usage * 2.0;
        let sy = 1.0 + monitor.mem_usage * 2.0;
        let sz = 1.0 + monitor.swap_usage * 2.0;
        let sw = 1.0 + monitor.load_avg;

        let move_cost = (dx.abs() * sx + dy.abs() * sy + dz.abs() * sz + dw.abs() * sw) * 0.1;
        self.bio_energy -= move_cost;

        let speed = 0.01; // Movement speed
        self.pos.x += dx * speed;
        self.pos.y += dy * speed;
        self.pos.z += dz * speed;
        self.pos.w += dw * speed;

        // Wrap coords (0.0 - 1.0)
        self.pos.x = self.pos.x.rem_euclid(1.0);
        self.pos.y = self.pos.y.rem_euclid(1.0);
        self.pos.z = self.pos.z.rem_euclid(1.0);
        self.pos.w = self.pos.w.rem_euclid(1.0);

        // Apply Spin Flip
        if flip_intent.abs() > 0.0 {
            let new_theta = theta + flip_intent;
            // Calculate Energy Delta
            // Field strength from RAM
            let field = monitor.mem_usage * 2.0;
            let d_e = grid.calculate_energy_delta(idx, new_theta, field);

            // Apply flip
            grid.set_spin(idx, new_theta);

            // Thermodynamics of Life
            // If d_e < 0 (Energy released), Agent gains it.
            // If d_e > 0 (Energy absorbed), Agent pays it.
            // Scale factor?
            self.bio_energy -= d_e * 5.0;

            // Visualization feedback
            if d_e < 0.0 {
                // Happy color
                self.color = GOLD;
            } else {
                // Sad color
                self.color = RED;
            }
        } else {
             // Resting color based on energy
             let e_norm = (self.bio_energy / 200.0).clamp(0.0, 1.0);
             self.color = Color::new(0.0, e_norm, e_norm, 0.8);
        }

        // Base Metabolism
        self.bio_energy -= 0.1;
    }
}
