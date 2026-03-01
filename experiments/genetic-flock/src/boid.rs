use locus::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;
use chimera_lang::prelude::*;

#[derive(Clone, Debug)]
pub struct BoidDna {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub color: Color,
}

impl BoidDna {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.8..1.5),
            max_force: rng.gen_range(0.05..0.15),
            view_radius: rng.gen_range(10.0..25.0),
            separation_weight: rng.gen_range(1.0..2.5),
            alignment_weight: rng.gen_range(0.5..1.5),
            cohesion_weight: rng.gen_range(0.5..1.5),
            color: Color::Indexed(rng.gen_range(20..230)),
        }
    }
}

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: BoidDna,
    pub chimera_dna: Dna,
    pub vm: ChimeraVM,
    pub energy: f64,
    pub generation: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let boid_dna = BoidDna::random();

        // Initial random Chimera DNA for the crossover mechanics
        let mut genes = Vec::new();
        for _ in 0..10 {
            let op = match rng.gen_range(0..5) {
                0 => OpCode::Add,
                1 => OpCode::Sub,
                2 => OpCode::Mul,
                3 => OpCode::Div,
                _ => OpCode::Nop,
            };
            genes.push(Gene { op, args: vec![] });
        }

        let chimera_dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
            evolution_config: None,
        };

        let vm = ChimeraVM::new(chimera_dna.clone());

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * boid_dna.max_speed, angle.sin() * boid_dna.max_speed),
            acceleration: Vec2::zero(),
            dna: boid_dna,
            chimera_dna,
            vm,
            energy: 100.0,
            generation: 1,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::zero();

        // Consume energy slowly
        self.energy -= 0.05;

        // Wrap around edges
        if self.position.x < 0.0 {
            self.position.x += width;
        }
        if self.position.x >= width {
            self.position.x -= width;
        }
        if self.position.y < 0.0 {
            self.position.y += height;
        }
        if self.position.y >= height {
            self.position.y -= height;
        }

        // Step the VM occasionally to represent active biology
        if rand::thread_rng().gen_bool(0.1) {
            self.vm.step();
        }
    }

    pub fn crossover_and_mutate(&mut self, other_dna: &BoidDna, other_chimera: &Dna) {
        let mut rng = rand::thread_rng();

        // Crossover traits
        if rng.gen_bool(0.5) {
            self.dna.separation_weight = other_dna.separation_weight;
        }
        if rng.gen_bool(0.5) {
            self.dna.alignment_weight = other_dna.alignment_weight;
        }
        if rng.gen_bool(0.5) {
            self.dna.cohesion_weight = other_dna.cohesion_weight;
        }

        // Mutate
        let mutation_rate = 0.1;
        self.dna.separation_weight += rng.gen_range(-mutation_rate..mutation_rate);
        self.dna.alignment_weight += rng.gen_range(-mutation_rate..mutation_rate);
        self.dna.cohesion_weight += rng.gen_range(-mutation_rate..mutation_rate);

        // Keep within bounds
        self.dna.separation_weight = self.dna.separation_weight.clamp(0.1, 3.0);
        self.dna.alignment_weight = self.dna.alignment_weight.clamp(0.1, 3.0);
        self.dna.cohesion_weight = self.dna.cohesion_weight.clamp(0.1, 3.0);

        // Adopt color from partner with slight chance of mutation
        if rng.gen_bool(0.5) {
            self.dna.color = other_dna.color;
        } else if rng.gen_bool(0.1) {
            self.dna.color = Color::Indexed(rng.gen_range(20..230));
        }

        self.generation += 1;
        self.energy = 100.0; // Refill energy on crossover

        // Simple Chimera DNA crossover (just take theirs for simplicity sometimes)
        if rng.gen_bool(0.5) {
            self.chimera_dna = other_chimera.clone();
            self.vm = ChimeraVM::new(self.chimera_dna.clone());
        }
    }
}
