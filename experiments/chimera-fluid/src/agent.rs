use crate::math::Vec4;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use std::collections::VecDeque;
use ::rand::Rng;

#[derive(Clone)]
pub struct Agent {
    pub vm: ChimeraVM,
    pub pos: Vec4,
    pub vel: Vec4,
    pub acc: Vec4,
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
        for _ in 0..32 {
            let op = match rng.gen_range(0..12) {
                0 => OpCode::Push,
                1 => OpCode::Drop,
                2 => OpCode::Add,
                3 => OpCode::Sub,
                4 => OpCode::Migrate,
                5 => OpCode::Signal,  // Output (Swim Force)
                6 => OpCode::Receive, // Input (Sense Gradient)
                7 => OpCode::Jump,
                8 => OpCode::Brz,
                9 => OpCode::Dup,
                10 => OpCode::Swap,
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
        vm.energy = 1000;

        Self {
            vm,
            pos: Vec4::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ),
            vel: Vec4::zero(),
            acc: Vec4::zero(),
            bio_energy: 100.0,
            color: GREEN,
            age: 0,
            id,
        }
    }

    pub fn update(&mut self, density: f32, gradient: Vec4, dt: f32) {
        if self.bio_energy <= 0.0 {
            self.color = GRAY;
            return;
        }
        self.age += 1;
        self.vm.energy = 1000; // Replenish Brain Energy

        // 1. Sense: Density and Gradient
        // Channel 0: Density
        // Channel 1-4: Gradient X, Y, Z, W

        let sens_density = (density * 100.0) as i64;
        self.vm.ether.entry(0).or_insert(VecDeque::new()).push_back(Value::Int(sens_density));

        let sens_dx = (gradient.x * 100.0) as i64;
        self.vm.ether.entry(1).or_insert(VecDeque::new()).push_back(Value::Int(sens_dx));

        let sens_dy = (gradient.y * 100.0) as i64;
        self.vm.ether.entry(2).or_insert(VecDeque::new()).push_back(Value::Int(sens_dy));

        let sens_dz = (gradient.z * 100.0) as i64;
        self.vm.ether.entry(3).or_insert(VecDeque::new()).push_back(Value::Int(sens_dz));

        let sens_dw = (gradient.w * 100.0) as i64;
        self.vm.ether.entry(4).or_insert(VecDeque::new()).push_back(Value::Int(sens_dw));

        // Limit queues
        for i in 0..5 {
            if let Some(queue) = self.vm.ether.get_mut(&i) {
                while queue.len() > 5 {
                    queue.pop_front();
                }
            }
        }

        // 2. Run VM
        // Run for a few steps
        for _ in 0..10 {
            if self.vm.energy > 0 {
                self.vm.step();
            } else {
                break;
            }
        }

        // 3. Act: Apply Force (Swim)
        // Channel 5: Force X
        // Channel 6: Force Y
        // Channel 7: Force Z
        // Channel 8: Force W
        // Channel 9: Change Color (Mood)

        let mut force = Vec4::zero();

        if let Some(queue) = self.vm.ether.get_mut(&5) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val { force.x += (v as f32) * 0.1; }
            }
        }
        if let Some(queue) = self.vm.ether.get_mut(&6) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val { force.y += (v as f32) * 0.1; }
            }
        }
        if let Some(queue) = self.vm.ether.get_mut(&7) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val { force.z += (v as f32) * 0.1; }
            }
        }
        if let Some(queue) = self.vm.ether.get_mut(&8) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val { force.w += (v as f32) * 0.1; }
            }
        }

        // Metabolic Cost of Movement
        let effort = force.x.abs() + force.y.abs() + force.z.abs() + force.w.abs();
        self.bio_energy -= effort * 0.01;

        // Apply Force
        self.acc = self.acc.add(force);

        // Update Color based on Channel 9
        if let Some(queue) = self.vm.ether.get_mut(&9) {
            if let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    // Hue shift?
                    let r = ((v % 255).abs() as f32) / 255.0;
                    self.color.r = (self.color.r + r) / 2.0;
                }
            }
        }

        // Decay bio_energy slowly
        self.bio_energy -= dt * 0.5;

        // Clamp Energy
        if self.bio_energy < 0.0 {
            self.bio_energy = 0.0;
        }
    }
}
