use glam::DVec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeKind {
    Struct,
    Enum,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub pos: DVec2,
    pub vel: DVec2,
    pub force: DVec2,
    pub name: String,
    pub kind: NodeKind,
}

impl Node {
    pub fn new(pos: DVec2, name: String, kind: NodeKind) -> Self {
        Self {
            pos,
            vel: DVec2::ZERO,
            force: DVec2::ZERO,
            name,
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
    pub length: f64,
}

pub struct System {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    // Physics Parameters
    pub repulsion_strength: f64,
    pub spring_strength: f64,
    pub damping: f64,
}

impl Default for System {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            repulsion_strength: 5000.0,
            spring_strength: 0.05,
            damping: 0.9,
        }
    }
}

impl System {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        if source < self.nodes.len() && target < self.nodes.len() {
            self.edges.push(Edge {
                source,
                target,
                length: 50.0, // Default spring length
            });
        }
    }

    fn compute_forces(&mut self) {
        // Reset forces
        for node in &mut self.nodes {
            node.force = DVec2::ZERO;
        }

        // 1. Repulsion (O(N^2))
        // Optimized: only compute for i < j
        let n = self.nodes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = self.nodes[j].pos - self.nodes[i].pos;
                let dist_sq = delta.length_squared();

                if dist_sq < 0.1 {
                    continue;
                } // Avoid singularity

                let dist = dist_sq.sqrt();
                // F = k / r^2
                let force_mag = self.repulsion_strength / dist_sq;
                let force = (delta / dist) * force_mag;

                self.nodes[i].force -= force;
                self.nodes[j].force += force;
            }
        }

        // 2. Springs (Hooke's Law)
        for edge in &self.edges {
            let p1 = self.nodes[edge.source].pos;
            let p2 = self.nodes[edge.target].pos;

            let delta = p2 - p1;
            let dist = delta.length();
            if dist < 0.1 {
                continue;
            }

            // F = k * (x - L)
            let displacement = dist - edge.length;
            let force_mag = self.spring_strength * displacement;
            let force = (delta / dist) * force_mag;

            self.nodes[edge.source].force += force;
            self.nodes[edge.target].force -= force;
        }

        // 3. Center Gravity (keep everything somewhat centered)
        // Weak force pulling towards (0,0) to prevent drifting to infinity
        for node in &mut self.nodes {
            let dist = node.pos.length();
            if dist > 1.0 {
                node.force -= node.pos.normalize() * (dist * 0.01);
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        // Velocity Verlet

        // First half-kick
        for node in &mut self.nodes {
            let accel = node.force;
            node.vel += accel * dt * 0.5;
        }

        // Full drift
        for node in &mut self.nodes {
            node.pos += node.vel * dt;
        }

        // Compute new forces
        self.compute_forces();

        // Second half-kick
        for node in &mut self.nodes {
            let accel = node.force;
            node.vel += accel * dt * 0.5;

            // Damping (Drag) applied to velocity directly
            node.vel *= self.damping;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_stability() {
        let mut sys = System::new();
        let n1 = sys.add_node(Node::new(
            DVec2::new(-10.0, 0.0),
            "A".into(),
            NodeKind::Struct,
        ));
        let n2 = sys.add_node(Node::new(
            DVec2::new(10.0, 0.0),
            "B".into(),
            NodeKind::Struct,
        ));
        sys.add_edge(n1, n2);

        // Run for a bit
        for _ in 0..100 {
            sys.step(0.1);
        }

        let p1 = sys.nodes[n1].pos;
        let p2 = sys.nodes[n2].pos;

        // They should attract (spring) but also repel.
        // Spring target length is 50.0.
        // Repulsion pushes them apart.
        // Equilibrium should be roughly where Repulsion = Spring force.
        // k_r / r^2 = k_s * (r - L)

        let dist = p1.distance(p2);
        assert!(dist > 1.0, "Nodes collapsed!");
        assert!(dist < 200.0, "Nodes flew apart! {}", dist);
    }
}
