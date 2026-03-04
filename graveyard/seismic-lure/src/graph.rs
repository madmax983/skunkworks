use crate::scanner::ScanResult;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub pos: Vec2,
    pub vel: Vec2,
    pub data: ScanResult,
    pub stress: f64,
}

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub damping: f64,
    pub repulsion: f64,
    pub center_attraction: f64,
    pub max_force: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            damping: 0.9,
            repulsion: 500.0,
            center_attraction: 0.01,
            max_force: 10.0,
        }
    }
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub width: f64,
    pub height: f64,
    pub config: SimulationConfig,
}

impl Graph {
    pub fn new(scan_results: Vec<ScanResult>) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes = Vec::new();

        for (i, res) in scan_results.into_iter().enumerate() {
            let stress = res.total_stress() as f64;
            // Initialize random position near center
            let pos = Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));

            nodes.push(Node {
                id: i,
                pos,
                vel: Vec2::new(0.0, 0.0),
                data: res,
                stress,
            });
        }

        Self {
            nodes,
            width: 200.0,
            height: 200.0,
            config: SimulationConfig::default(),
        }
    }

    fn apply_forces(&mut self) {
        // Clone positions for calculation to avoid borrow checker issues
        let positions: Vec<Vec2> = self.nodes.iter().map(|n| n.pos).collect();

        for (i, node) in self.nodes.iter_mut().enumerate() {
            let mut force = Vec2::new(0.0, 0.0);
            let p1 = positions[i];

            // Attract to center (0,0)
            force.x -= p1.x * self.config.center_attraction;
            force.y -= p1.y * self.config.center_attraction;

            // Repel from others
            for (j, p2) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq > 0.01 {
                    let f = self.config.repulsion / dist_sq.max(1.0);
                    let dist = dist_sq.sqrt();
                    force.x += (dx / dist) * f;
                    force.y += (dy / dist) * f;
                }
            }

            // Limit max force
            let f_len = (force.x.powi(2) + force.y.powi(2)).sqrt();
            if f_len > self.config.max_force {
                force.x = (force.x / f_len) * self.config.max_force;
                force.y = (force.y / f_len) * self.config.max_force;
            }

            node.vel.x = (node.vel.x + force.x) * self.config.damping;
            node.vel.y = (node.vel.y + force.y) * self.config.damping;
            node.pos.x += node.vel.x;
            node.pos.y += node.vel.y;
        }
    }

    pub fn step(&mut self) {
        self.apply_forces();
    }
}
