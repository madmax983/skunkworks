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
    pub mass: f64,
}

impl Node {
    pub fn new(pos: DVec2, name: String, kind: NodeKind, mass: f64) -> Self {
        Self {
            pos,
            vel: DVec2::ZERO,
            force: DVec2::ZERO,
            name,
            kind,
            mass: mass.max(1.0),
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
            repulsion_strength: 8000.0,
            spring_strength: 0.08,
            damping: 0.95,
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
            // Spring length proportional to sum of masses (radii) roughly
            let l = 40.0 + (self.nodes[source].mass + self.nodes[target].mass) * 2.0;
            self.edges.push(Edge {
                source,
                target,
                length: l,
            });
        }
    }

    fn compute_forces(&mut self) {
        // Reset forces
        for node in &mut self.nodes {
            node.force = DVec2::ZERO;
        }

        // 1. Repulsion (O(N^2))
        let n = self.nodes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = self.nodes[j].pos - self.nodes[i].pos;
                let dist_sq = delta.length_squared();

                if dist_sq < 0.1 {
                    continue;
                }

                let dist = dist_sq.sqrt();
                // F = k * (m1 * m2) / r^2 (Gravity-like repulsion?)
                // Actually usually just Coulomb for layout: k / r^2
                // Let's scale by mass to prevent heavy overlaps
                let force_mag = self.repulsion_strength * (self.nodes[i].mass.sqrt() * self.nodes[j].mass.sqrt()) / dist_sq;
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

            let displacement = dist - edge.length;
            let force_mag = self.spring_strength * displacement;
            let force = (delta / dist) * force_mag;

            self.nodes[edge.source].force += force;
            self.nodes[edge.target].force -= force;
        }

        // 3. Center Gravity
        for node in &mut self.nodes {
            let dist = node.pos.length();
            if dist > 1.0 {
                // Heavier nodes pulled to center more strongly? Or less?
                // Let's say all pulled equally to keep them on screen
                node.force -= node.pos.normalize() * (dist * 0.02 * node.mass.sqrt());
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.compute_forces();

        for node in &mut self.nodes {
            // F = ma -> a = F/m
            let accel = node.force / node.mass;
            node.vel += accel * dt;
            node.pos += node.vel * dt;
            node.vel *= self.damping;
        }
    }
}
