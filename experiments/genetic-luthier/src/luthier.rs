use chimera_lang::prelude::*;
use macroquad::prelude::*;
use ::rand::Rng;

pub struct LuthierString {
    // Physics / Visuals
    pub pos: Vec2,
    pub length: f32,
    pub vibration: f32,
    pub velocity: f32,

    // Properties (Phenotype)
    pub frequency: f32, // Hz
    pub tension: f32,   // Spring constant k
    pub decay: f32,     // Damping factor (0.0 - 1.0)
    pub color: Color,

    // Genetics
    pub vm: ChimeraVM,
    pub fitness: f32,
    pub is_mutating: bool,
    pub generation: usize,
}

impl LuthierString {
    pub fn new(pos: Vec2, length: f32, base_freq: f32) -> Self {
        let mut rng = ::rand::thread_rng();

        // Initial random phenotype
        let frequency = base_freq * rng.gen_range(0.5..2.0);
        let tension = rng.gen_range(100.0..500.0);
        let decay = rng.gen_range(0.98..0.999);

        // Generate random DNA
        // A simple genome that mutates the values in the Petri Dish (Grid)
        let mut genes = Vec::new();
        for _ in 0..10 {
            // Random opcodes to create chaos/mutation
            let op = match rng.gen_range(0..5) {
                0 => OpCode::Add,
                1 => OpCode::Sub,
                2 => OpCode::Mul,
                3 => OpCode::Div,
                _ => OpCode::Nop,
            };
            genes.push(Gene { op, args: vec![] });
        }

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }]
            }
        };

        let vm = ChimeraVM::new(dna);

        // Seed the VM memory with initial values
        // We use the Grid (Petri Dish) as the interface
        // (0, 0) -> Frequency Multiplier (mapped 0-255 -> 0.5-2.0)
        // (0, 1) -> Tension (mapped 0-255 -> 100-500)
        // (0, 2) -> Decay (mapped 0-255 -> 0.90-0.999)

        // We can't easily write to private grid, but we can rely on the VM initializing/running.

        Self {
            pos,
            length,
            vibration: 0.0,
            velocity: 0.0,
            frequency,
            tension,
            decay,
            color: WHITE,
            vm,
            fitness: 0.0,
            is_mutating: false,
            generation: 0,
        }
    }

    pub fn update_physics(&mut self, dt: f32) {
        // Damped spring simulation
        // F = -kx - cv
        let damping = 2.0; // Visual damping, distinct from audio decay

        let acceleration = -self.tension * self.vibration - damping * self.velocity;
        self.velocity += acceleration * dt;
        self.vibration += self.velocity * dt;

        // Clamp to avoid explosion
        self.vibration = self.vibration.clamp(-50.0, 50.0);
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }

    pub fn evolve(&mut self, base_freq: f32) {
        // If high fitness, don't mutate (Elitism)
        if self.fitness > 0.8 {
            self.is_mutating = false;
            self.color = GREEN;
            return;
        }

        self.is_mutating = true;
        self.generation += 1;
        self.color = RED;

        // Run the VM for a few ticks
        for _ in 0..10 {
            self.vm.step();
        }

        // Simulate "Genetic Drift" by mutating the phenotype directly

        let mut rng = ::rand::thread_rng();

        // Mutation magnitude inversely proportional to fitness
        let mutation_rate = (1.0 - self.fitness).max(0.1) * 0.1;

        // Mutate Frequency
        let f_change = rng.gen_range(-mutation_rate..mutation_rate) * base_freq;
        self.frequency = (self.frequency + f_change).clamp(base_freq * 0.5, base_freq * 2.0);

        // Mutate Tension
        let t_change = rng.gen_range(-mutation_rate..mutation_rate) * 100.0;
        self.tension = (self.tension + t_change).clamp(50.0, 600.0);

        // Mutate Decay
        let d_change = rng.gen_range(-mutation_rate..mutation_rate) * 0.01;
        self.decay = (self.decay + d_change).clamp(0.95, 0.999);
    }

    pub fn draw(&self) {
        let start = self.pos;
        let end = self.pos + vec2(0.0, self.length);

        let mid = (start + end) * 0.5;
        let offset = vec2(self.vibration, 0.0);
        let control = mid + offset;

        let thickness = 2.0;
        let mut color = self.color;

        // Visualize vibration intensity
        let intensity = (self.vibration.abs() / 10.0).clamp(0.0, 1.0);
        color.a = 0.5 + 0.5 * intensity;

        draw_line(start.x, start.y, control.x, control.y, thickness, color);
        draw_line(control.x, control.y, end.x, end.y, thickness, color);

        // Draw tuning pegs (Agents)
        if self.is_mutating {
            draw_circle(start.x, start.y - 10.0, 4.0, RED);
        } else {
            draw_circle(start.x, start.y - 10.0, 4.0, GREEN);
        }

        // Draw frequency text
        draw_text(&format!("{:.1}Hz", self.frequency), start.x - 20.0, end.y + 20.0, 16.0, LIGHTGRAY);
    }
}
