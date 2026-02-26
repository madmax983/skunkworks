use chimera_lang::prelude::*;
use locus::Vec2;
use crate::laban::LabanEffort;
use rand::Rng;

pub struct Dancer {
    pub vm: ChimeraVM,
    pub pos: Vec2,
    pub vel: Vec2,
    pub effort: LabanEffort,
    pub id: usize,
    pub color_idx: u8,
}

impl Dancer {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        // Create random DNA
        let mut rng = rand::thread_rng();
        let mut genes = Vec::new();

        // Genes to manipulate grid (Laban parameters at 0,0 to 0,3)
        for _ in 0..30 {
             let r = rng.gen_range(0..10);
             match r {
                 0..=2 => {
                     // Push random number (0-100)
                     genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(0..100))] });
                 },
                 3 => {
                     // Write to random grid location (0-3 are Laban params)
                     genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(0..4))] }); // x
                     genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }); // y
                     genes.push(Gene { op: OpCode::GWrite, args: vec![] });
                 },
                 4 => {
                     // Math
                     genes.push(Gene { op: OpCode::Add, args: vec![] });
                 },
                 5 => {
                     // Sub
                     genes.push(Gene { op: OpCode::Sub, args: vec![] });
                 },
                 6 => {
                     // Photosynthesize (keep alive)
                     genes.push(Gene { op: OpCode::Photosynthesize, args: vec![] });
                 },
                 7 => {
                     // Duplicate
                     genes.push(Gene { op: OpCode::Dup, args: vec![] });
                 },
                 _ => {}
             }
        }
        // Jump back to 0
        genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] });

        let dna = Dna { evolution_config: None, helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);

        // Pre-seed grid with random Laban values
        for i in 0..4 {
            vm.grid[0][i] = Value::Int(rng.gen_range(0..100));
        }

        Self {
            vm,
            pos: Vec2::new(x as f64, y as f64),
            vel: Vec2::new(0.0, 0.0),
            effort: LabanEffort::new(),
            id,
            color_idx: rng.gen(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.vm.halted {
            self.vm.step();
            // Force energy refill to keep dancing forever
            if self.vm.energy < 10 {
                self.vm.energy = 50;
            }
        }

        // Read Laban Effort from Grid [0][0..3]
        // [0][0] = Weight (0=Strong, 1=Light)
        // [0][1] = Time   (0=Sudden, 1=Sustained)
        // [0][2] = Space  (0=Direct, 1=Indirect)
        // [0][3] = Flow   (0=Bound, 1=Free)

        let get_val = |vm: &ChimeraVM, idx: usize| -> f32 {
            match &vm.grid[0][idx] {
                Value::Int(n) => (*n as f32 / 100.0).clamp(0.0, 1.0),
                _ => 0.5,
            }
        };

        self.effort.weight = get_val(&self.vm, 0);
        self.effort.time = get_val(&self.vm, 1);
        self.effort.space = get_val(&self.vm, 2);
        self.effort.flow = get_val(&self.vm, 3);

        // Physics Logic
        let mut rng = rand::thread_rng();

        // Space: Direct vs Indirect
        // Indirect adds random noise to velocity direction
        let noise_strength = if self.effort.is_indirect() { 50.0 } else { 0.0 };
        let noise = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)) * noise_strength;

        // Use -pos to seek center (0,0)
        let center_dir = -self.pos.normalize(); // Locus normalize handles zero
        let center_strength = if self.effort.is_direct() { 20.0 } else { 2.0 };
        let center_force = center_dir * center_strength;

        // Weight: Acceleration Magnitude
        // Strong (low weight) = High Force
        // Light (high weight) = Low Force
        let force_mag = if self.effort.is_strong() { 40.0 } else { 10.0 };

        let mut force = (noise + center_force).normalize() * force_mag;

        // Time: Sudden vs Sustained
        if self.effort.is_sudden() {
             if rng.gen_bool(0.2) {
                 force *= 5.0; // JERK
             } else {
                 force *= 0.1; // Pause
             }
        }

        self.vel += force * (dt as f64);

        // Flow: Friction
        // Bound (low flow) = High Friction (Stops quickly)
        // Free (high flow) = Low Friction (Glides)
        let friction = if self.effort.is_bound() { 0.90 } else { 0.99 };
        self.vel *= friction;

        // Cap speed
        let max_speed = if self.effort.is_sudden() { 100.0 } else { 30.0 };
        if self.vel.magnitude() > max_speed {
             self.vel = self.vel.normalize() * max_speed;
        }

        self.pos += self.vel * (dt as f64);
    }
}
