use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::{ChimeraVM, Value},
};
use macroquad::prelude::*;
use macroquad::color::hsl_to_rgb;
use poincare_disk::{mobius_add, Point};

#[derive(Clone)]
pub struct Vehicle {
    pub pos: Point,
    pub heading: f64,
    pub vm: ChimeraVM,
    pub color: Color,
    pub age: f32,
}

impl Vehicle {
    pub fn new(pos: Point, seed: u64) -> Self {
        // Simple initial genome: Drive forward + Wiggle
        let genes = vec![
            // 1. Read Noise (Input from Grid[0][2])
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)], // X
            },
            Gene {
                op: OpCode::GRead,
                args: vec![],
            },
            // Scale Noise (Divide by 5)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Div,
                args: vec![],
            },
            // Output Turn to Grid[0][1]
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // X
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // 2. Push Constant (Thrust)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // Base speed
            },
            // Output Thrust to Grid[0][0]
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // X
            },
            Gene {
                op: OpCode::GWrite,
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
            pos,
            heading: (seed % 360) as f64 * std::f64::consts::PI / 180.0,
            vm,
            color: hsl_to_rgb((seed % 100) as f32 / 100.0, 0.8, 0.5),
            age: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.age += dt;

        // 1. Write Sensor Data to Grid
        // Grid[0][2] = Random Noise (-10 to 10)
        let noise = macroquad::rand::gen_range(-10, 11);
        self.vm.grid[0][2] = Value::Int(noise);

        // 2. Step VM
        for _ in 0..5 {
            if !self.vm.halted {
                self.vm.step();
            }
        }

        // Refuel
        if self.vm.energy < 10 {
            self.vm.energy = 100;
        }

        // 3. Read Outputs
        // Grid[0][0] = Thrust
        // Grid[0][1] = Turn

        let thrust = match self.vm.grid.get(0).and_then(|r| r.get(0)) {
            Some(Value::Int(v)) => (*v as f64).clamp(-10.0, 10.0),
            _ => 0.0,
        } * 0.1; // Scale down

        let turn = match self.vm.grid.get(0).and_then(|r| r.get(1)) {
            Some(Value::Int(v)) => (*v as f64).clamp(-10.0, 10.0),
            _ => 0.0,
        } * 0.5; // Scale turn

        // 4. Apply Physics
        self.heading += turn * dt as f64;

        let step_len = thrust * dt as f64;
        let step = Point::from_polar(step_len, self.heading);

        self.pos = mobius_add(self.pos, step);
    }
}
