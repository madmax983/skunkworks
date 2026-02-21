use crate::agent::HyperAgent;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use num_complex::Complex;
use poincare_disk::{hyperbolic_dist, Point};
use rand::Rng;
use ratatui::style::Color;

pub struct World {
    pub agents: Vec<HyperAgent>,
    pub food: Vec<Point>,
    pub score: u64,
}

impl World {
    pub fn new(num_agents: usize) -> Self {
        let mut agents = Vec::with_capacity(num_agents);
        let mut rng = rand::thread_rng();

        for _ in 0..num_agents {
            // Default DNA:
            // [ Push(Angle), Push(Speed), Jump(0) ]
            // Random movements for now
            let mut genes = Vec::new();

            // Gene 1: Push Random Angle (0-100)
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(rng.gen_range(0..100))],
            });
            // Gene 2: Push Speed (50)
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            });
            // Gene 3: GWrite Angle (0, 0)
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            });
            genes.push(Gene {
                op: OpCode::GWrite,
                args: vec![],
            });
             // Gene 4: GWrite Speed (0, 1)
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)], // Speed val
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            });
            genes.push(Gene {
                op: OpCode::GWrite,
                args: vec![],
            });

            // Loop
            genes.push(Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            });

            let dna = Dna {
                helix: Helix {
                    strands: vec![Strand { genes }],
                },
            };

            let r = rng.gen_range(0.0..0.5);
            let theta = rng.gen_range(0.0..std::f64::consts::TAU);
            let pos = Complex::from_polar(r, theta);

            let color = Color::Rgb(
                rng.gen_range(100..255),
                rng.gen_range(100..255),
                rng.gen_range(100..255),
            );

            agents.push(HyperAgent::new(dna, pos, color));
        }

        let mut food = Vec::new();
        for _ in 0..10 {
            let r = rng.gen_range(0.0..0.8);
            let theta = rng.gen_range(0.0..std::f64::consts::TAU);
            food.push(Complex::from_polar(r, theta));
        }

        Self {
            agents,
            food,
            score: 0,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Update Agents
        for agent in &mut self.agents {
            // Find nearest food
            let mut nearest_dist = f64::MAX;
            for f in &self.food {
                let d = hyperbolic_dist(agent.pos, *f);
                if d < nearest_dist {
                    nearest_dist = d;
                }
            }

            // Inject sensor data (Grid[0][2] is updated inside update())
            // Ideally we pass nearest_dist to agent.update(), but for now agent.update calculates distance to center.
            // Let's modify agent.update later to accept sensor inputs if needed.

            agent.update();
        }

        // Check Collisions
        for agent in &mut self.agents {
            if agent.vm.halted { continue; }

            let mut eaten_indices = Vec::new();
            for (i, f) in self.food.iter().enumerate() {
                if hyperbolic_dist(agent.pos, *f) < 0.1 {
                    eaten_indices.push(i);
                    agent.vm.energy += 50;
                    self.score += 1;
                }
            }

            // Remove eaten food and respawn
            // Iterate in reverse to safe remove
            for i in eaten_indices.iter().rev() {
                self.food.remove(*i);
                let r = rng.gen_range(0.0..0.9);
                let theta = rng.gen_range(0.0..std::f64::consts::TAU);
                self.food.push(Complex::from_polar(r, theta));
            }
        }

        // Remove dead agents
        self.agents.retain(|a| !a.vm.halted);

        // Respawn if extinct
        if self.agents.is_empty() {
             // Re-seed? For now just game over state or let loop handle it.
        }
    }
}
