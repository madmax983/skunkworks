use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub mass: f32,
    pub fixed: bool,
}

#[derive(Clone, Debug)]
pub struct Link {
    pub a: usize,
    pub b: usize,
    pub length: f32,
}

#[derive(Clone)]
pub struct PendulumSystem {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub gravity: Vec2,
    pub friction: f32,
}

impl PendulumSystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            gravity: Vec2::new(0.0, 9.8),
            friction: 0.999, // Air resistance
        }
    }

    pub fn step(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }

        // Apply forces (Gravity + Verlet Integration)
        for node in &mut self.nodes {
            if !node.fixed {
                let vel = (node.pos - node.prev_pos) * self.friction;
                node.prev_pos = node.pos;
                // a = F/m. F_g = m * g. a = g.
                let acc = self.gravity;
                node.pos += vel + acc * dt * dt;
            }
        }

        // Solve distance constraints (Position Based Dynamics)
        // Multiple iterations for stability
        let iterations = 20;
        for _ in 0..iterations {
            for link in &self.links {
                let n_a = &self.nodes[link.a];
                let n_b = &self.nodes[link.b];

                let delta = n_b.pos - n_a.pos;
                let current_dist = delta.length();
                if current_dist == 0.0 {
                    continue;
                }

                let diff = (current_dist - link.length) / current_dist;
                let inv_mass_a = if n_a.fixed { 0.0 } else { 1.0 / n_a.mass };
                let inv_mass_b = if n_b.fixed { 0.0 } else { 1.0 / n_b.mass };
                let total_inv_mass = inv_mass_a + inv_mass_b;

                if total_inv_mass == 0.0 {
                    continue;
                }

                let correction = delta * diff / total_inv_mass;

                if !self.nodes[link.a].fixed {
                    self.nodes[link.a].pos += correction * inv_mass_a;
                }
                if !self.nodes[link.b].fixed {
                    self.nodes[link.b].pos -= correction * inv_mass_b;
                }
            }
        }
    }
}

impl PendulumSystem {
    pub fn add_node(&mut self, pos: Vec2, mass: f32, fixed: bool, name: String) -> usize {
        let node = Node {
            name,
            pos,
            prev_pos: pos,
            mass,
            fixed,
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn add_link(&mut self, a: usize, b: usize, length: f32) {
        self.links.push(Link { a, b, length });
    }
}
