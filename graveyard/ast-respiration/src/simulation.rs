use glam::DVec2;

#[derive(Clone)]
pub struct Node {
    pub pos: DVec2,
    pub vel: DVec2,
    pub force: DVec2,
    pub mass: f64,
    pub fixed: bool,
    pub radius: f64,
}

impl Node {
    pub fn new(pos: DVec2, mass: f64) -> Self {
        Self {
            pos,
            vel: DVec2::ZERO,
            force: DVec2::ZERO,
            mass,
            fixed: false,
            radius: 5.0, // Default visual radius
        }
    }
}

pub struct Edge {
    pub source: usize,
    pub target: usize,
    pub base_len: f64,
}

pub struct Simulation {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub breath_phase: f64,
    pub breath_speed: f64,
    pub breath_amp: f64,

    // Physics parameters
    pub stiffness: f64,
    pub repulsion: f64,
    pub damping: f64,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            breath_phase: 0.0,
            breath_speed: 2.0, // rad/s
            breath_amp: 0.3,   // 30% expansion
            stiffness: 50.0,
            repulsion: 1000.0,
            damping: 0.9,
        }
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn add_edge(&mut self, source: usize, target: usize, len: f64) {
        self.edges.push(Edge {
            source,
            target,
            base_len: len,
        });
    }

    pub fn tick(&mut self, dt: f64) {
        // Update breathing
        self.breath_phase += self.breath_speed * dt;
        let breath_factor = 1.0 + self.breath_amp * self.breath_phase.sin();

        // Reset forces
        for node in &mut self.nodes {
            node.force = DVec2::ZERO;
        }

        // 1. Repulsion (All pairs)
        // O(N^2) - acceptable for ASTs < 500 nodes.
        let count = self.nodes.len();
        for i in 0..count {
            for j in (i + 1)..count {
                let diff = self.nodes[i].pos - self.nodes[j].pos;
                let dist_sq = diff.length_squared();

                if dist_sq > 0.01 {
                    let dist = dist_sq.sqrt();
                    // Coulomb-like repulsion
                    let force_mag = self.repulsion / dist_sq;
                    let force = diff / dist * force_mag;

                    if !self.nodes[i].fixed {
                        self.nodes[i].force += force;
                    }
                    if !self.nodes[j].fixed {
                        self.nodes[j].force -= force;
                    }
                }
            }
        }

        // 2. Springs (Edges)
        for edge in &self.edges {
            let idx1 = edge.source;
            let idx2 = edge.target;

            let p1 = self.nodes[idx1].pos;
            let p2 = self.nodes[idx2].pos;

            let diff = p1 - p2;
            let dist = diff.length();

            if dist > 0.001 {
                let target_len = edge.base_len * breath_factor;
                let displacement = dist - target_len;
                let force_mag = self.stiffness * displacement;
                let force = diff / dist * (-force_mag);

                if !self.nodes[idx1].fixed {
                    self.nodes[idx1].force += force;
                }
                if !self.nodes[idx2].fixed {
                    self.nodes[idx2].force -= force;
                }
            }
        }

        // 3. Integration (Symplectic Euler)
        for node in &mut self.nodes {
            if !node.fixed {
                let acc = node.force / node.mass;
                node.vel += acc * dt;
                node.vel *= self.damping; // Damping
                node.pos += node.vel * dt;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_moves_nodes() {
        let mut sim = Simulation::new();
        let n1 = sim.add_node(Node::new(DVec2::new(0.0, 0.0), 1.0));
        let n2 = sim.add_node(Node::new(DVec2::new(10.0, 0.0), 1.0)); // Dist 10

        sim.add_edge(n1, n2, 5.0); // Target 5. Should pull together.

        sim.tick(0.1);

        let p1 = sim.nodes[n1].pos;
        let p2 = sim.nodes[n2].pos;

        // They should have moved closer
        let new_dist = (p1 - p2).length();
        assert!(new_dist < 10.0);
        assert!(new_dist > 0.0);
    }

    #[test]
    fn test_breathing() {
        let mut sim = Simulation::new();
        sim.breath_amp = 0.5;
        sim.breath_speed = 10.0;

        let initial_factor = 1.0 + sim.breath_amp * sim.breath_phase.sin(); // 1.0
        assert_eq!(initial_factor, 1.0);

        sim.tick(0.1);

        let new_factor = 1.0 + sim.breath_amp * sim.breath_phase.sin();
        assert_ne!(new_factor, 1.0);
    }
}
