use crate::math::Vec4D;
use crate::monitor::SystemMonitor;
use chimera_lang::prelude::*;
use macroquad::prelude::*;

pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Vec4D,
    pub vel: Vec4D,
    pub color: Color,
    pub bio_energy: f32,
}

impl Agent {
    pub fn new() -> Self {
        // DNA: Simple 4D Explorer
        // [ Push(1) Push(0) Migrate ] (East) -> X
        // [ Push(1) Push(0) Signal ] (Z)
        // [ Push(1) Push(1) Signal ] (W)
        // [ Jump(0) ]
        let genes = vec![
            // Move East (dy=0, dx=1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
            // Signal Z+ (Channel 0, Value 1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Signal,
                args: vec![],
            },
            // Signal W+ (Channel 1, Value 1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Signal,
                args: vec![],
            },
            // Loop
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        Self {
            vm,
            pos: Vec4D::zero(),
            vel: Vec4D::zero(),
            color: GREEN,
            bio_energy: 1000.0,
        }
    }

    pub fn update(&mut self, monitor: &SystemMonitor) {
        if self.bio_energy <= 0.0 {
            self.color = GRAY;
            return;
        }

        let old_loc = self.vm.context_loc;
        self.vm.step();
        let new_loc = self.vm.context_loc;

        // 1. Grid Movement (X, Y) from Migrate
        let dy = (new_loc.0 as isize - old_loc.0 as isize) as f32;
        let dx = (new_loc.1 as isize - old_loc.1 as isize) as f32;

        // 2. Ether Signals (Z, W) from Signal
        let mut dz = 0.0;
        let mut dw = 0.0;

        if let Some(queue) = self.vm.ether.get_mut(&0) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    dz += v as f32 * 0.1;
                }
            }
        }
        if let Some(queue) = self.vm.ether.get_mut(&1) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    dw += v as f32 * 0.1;
                }
            }
        }

        // 3. Relativistic Cost (Metabolism scaled by Distortion)
        let sx = 1.0 + monitor.cpu_usage * 5.0; // High CPU stretches X
        let sy = 1.0 + monitor.mem_usage * 5.0; // High Mem stretches Y
        let sz = 1.0 + monitor.swap_usage * 5.0; // High Swap stretches Z
        let sw = 1.0 + monitor.load_avg; // Load oscillates W

        let cost = (dx.abs() * sx) + (dy.abs() * sy) + (dz.abs() * sz) + (dw.abs() * sw);
        self.bio_energy -= cost * 0.5; // Base metabolic rate
        self.vm.energy = self.bio_energy as i64;

        // 4. Physics
        self.vel.x += dx * 0.02;
        self.vel.y += dy * 0.02;
        self.vel.z += dz * 0.02;
        self.vel.w += dw * 0.02;

        // Add some brownian motion
        let r = 0.005;
        self.vel.x += macroquad::rand::gen_range(-r, r);
        self.vel.y += macroquad::rand::gen_range(-r, r);
        self.vel.z += macroquad::rand::gen_range(-r, r);
        self.vel.w += macroquad::rand::gen_range(-r, r);

        self.vel = self.vel.scale(0.95); // Friction
        self.pos = self.pos.add(self.vel);

        // Bounds (Hyper-Torus)
        let limit = 4.0;
        if self.pos.x > limit {
            self.pos.x -= 2.0 * limit;
        }
        if self.pos.x < -limit {
            self.pos.x += 2.0 * limit;
        }
        if self.pos.y > limit {
            self.pos.y -= 2.0 * limit;
        }
        if self.pos.y < -limit {
            self.pos.y += 2.0 * limit;
        }
        if self.pos.z > limit {
            self.pos.z -= 2.0 * limit;
        }
        if self.pos.z < -limit {
            self.pos.z += 2.0 * limit;
        }
        if self.pos.w > limit {
            self.pos.w -= 2.0 * limit;
        }
        if self.pos.w < -limit {
            self.pos.w += 2.0 * limit;
        }

        // Color
        let energy_norm = (self.bio_energy / 1000.0).clamp(0.0, 1.0);
        self.color = Color::new(energy_norm, 0.5, 1.0 - energy_norm, 1.0);
    }
}
