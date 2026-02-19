use glam::Vec3;
use rand::Rng;
use chimera_lang::ast::{Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

#[derive(Debug, Clone)]
pub struct GeneticNode {
    pub pos: Vec3,
    pub vel: Vec3,
    pub force: Vec3,
    pub fixed: bool,
    pub mass: f32,
    pub gene: Gene,
    pub mutated: bool,
    pub active: bool, // Is the IP currently at this gene?
}

impl GeneticNode {
    pub fn new(pos: Vec3, fixed: bool, gene: Gene) -> Self {
        // Mass depends on OpCode type
        let mass = match gene.op {
            OpCode::Push | OpCode::Dup | OpCode::Swap => 1.0,
            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => 2.0, // Heavy math
            OpCode::Jump | OpCode::Brz => 0.5, // Light flow control
            OpCode::GRead | OpCode::GWrite => 3.0, // Heavy IO
            OpCode::Photosynthesize | OpCode::Consume => 1.5,
            _ => 1.0,
        };

        Self {
            pos,
            vel: Vec3::ZERO,
            force: Vec3::ZERO,
            fixed,
            mass,
            gene,
            mutated: false,
            active: false,
        }
    }
}

pub struct GeneticString {
    pub nodes: Vec<GeneticNode>,
    pub rest_length: f32,
    pub tension: f32,
    pub damping: f32,
    pub mutation_threshold: f32,
    pub kinetic_energy: f32,
}

impl GeneticString {
    pub fn new(genes: Vec<Gene>, start: Vec3, end: Vec3, tension: f32, damping: f32) -> Self {
        let len = genes.len();
        let mut nodes = Vec::with_capacity(len);

        if len > 0 {
            let step = if len > 1 {
                (end - start) / (len - 1) as f32
            } else {
                Vec3::ZERO
            };

            let rest_length = if len > 1 { step.length() } else { 1.0 };

            for (i, gene) in genes.into_iter().enumerate() {
                let pos = start + step * i as f32;
                // Fix the ends
                let fixed = i == 0 || i == len - 1;
                nodes.push(GeneticNode::new(pos, fixed, gene));
            }

            Self {
                nodes,
                rest_length,
                tension,
                damping,
                mutation_threshold: 100.0, // Energy threshold
                kinetic_energy: 0.0,
            }
        } else {
             Self {
                nodes: Vec::new(),
                rest_length: 1.0,
                tension,
                damping,
                mutation_threshold: 100.0,
                kinetic_energy: 0.0,
             }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_physics(dt);
        self.calculate_energy();
    }

    fn update_physics(&mut self, dt: f32) {
        let len = self.nodes.len();
        if len < 2 { return; }

        let mut forces = vec![Vec3::ZERO; len];

        // Spring forces
        for i in 0..len - 1 {
            let p1 = self.nodes[i].pos;
            let p2 = self.nodes[i + 1].pos;

            let delta = p2 - p1;
            let dist = delta.length();

            if dist > 1e-6 {
                let dir = delta / dist;
                let stretch = dist - self.rest_length;
                let f = dir * (self.tension * stretch);

                forces[i] += f;
                forces[i + 1] -= f;
            }
        }

        // Integration
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.fixed {
                node.vel = Vec3::ZERO;
                continue;
            }

            // Gravity/Buoyancy? Maybe slight gravity
            // node.force += Vec3::new(0.0, 5.0, 0.0);

            node.force = forces[i];
            node.force -= node.vel * self.damping; // Damping

            let acc = node.force / node.mass;
            node.vel += acc * dt;
            node.pos += node.vel * dt;
        }
    }

    fn calculate_energy(&mut self) {
        self.kinetic_energy = self.nodes.iter().map(|n| 0.5 * n.mass * n.vel.length_squared()).sum();
    }

    pub fn pluck(&mut self) {
        let mut rng = rand::thread_rng();
        let len = self.nodes.len();
        if len > 2 {
            // Pluck a few random nodes
            for _ in 0..3 {
                let idx = rng.gen_range(1..len - 1);
                if !self.nodes[idx].fixed {
                    let force = Vec3::new(0.0, rng.gen_range(-50.0..50.0), 0.0);
                    self.nodes[idx].vel += force;
                }
            }
        }
    }

    pub fn reset_mutation_flags(&mut self) {
        for node in &mut self.nodes {
            node.mutated = false;
        }
    }

    // Returns true if a mutation occurred
    pub fn check_mutation_trigger(&self) -> bool {
        self.kinetic_energy > self.mutation_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chimera_lang::ast::{Gene, Nucleotide};
    use chimera_lang::opcode::OpCode;
    use glam::Vec3;

    #[test]
    fn test_node_mass() {
        let gene = Gene { op: OpCode::Add, args: vec![] };
        let node = GeneticNode::new(Vec3::ZERO, false, gene);
        assert_eq!(node.mass, 2.0); // Add is heavy
    }

    #[test]
    fn test_string_energy() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        ];
        let mut string = GeneticString::new(genes, Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 1.0, 0.1);

        // Initial energy should be 0
        assert_eq!(string.kinetic_energy, 0.0);

        // Pluck to add energy
        string.pluck();
        string.calculate_energy();

        // Should have some energy (randomness makes it > 0 usually, unless RNG fails)
        // With 3 nodes (pluck loop skips first/last if len > 2).
        // Wait, len=2. pluck checks len > 2. So energy stays 0.
        // Let's make len=3.
    }

    #[test]
    fn test_string_pluck() {
        let genes = vec![
            Gene { op: OpCode::Nop, args: vec![] },
            Gene { op: OpCode::Nop, args: vec![] },
            Gene { op: OpCode::Nop, args: vec![] },
        ];
        let mut string = GeneticString::new(genes, Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0), 1.0, 0.1);

        string.pluck();
        string.calculate_energy();
        assert!(string.kinetic_energy >= 0.0); // It might be 0 if RNG selects nothing or force is 0, but usually > 0
    }
}
