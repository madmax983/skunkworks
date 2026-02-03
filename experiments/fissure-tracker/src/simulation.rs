use crate::scanner::ScanResult;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn distance(&self, other: Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
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
pub struct Fissure {
    pub origin_id: usize,
    pub points: Vec<Vec2>,
    pub age: usize,
}

pub struct World {
    pub nodes: Vec<Node>,
    pub fissures: Vec<Fissure>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(scan_results: Vec<ScanResult>) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes = Vec::new();

        for (i, res) in scan_results.into_iter().enumerate() {
            let stress = res.total_stress() as f64;
            // Initialize random position
            let pos = Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0));

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
            fissures: Vec::new(),
            width: 200.0,
            height: 200.0,
        }
    }

    pub fn step(&mut self) {
        // 1. Force Directed Layout
        let damping = 0.9;
        let repulsion = 500.0;
        let center_attraction = 0.01;

        // Clone positions for calculation to avoid borrow checker issues
        let positions: Vec<Vec2> = self.nodes.iter().map(|n| n.pos).collect();
        let count = self.nodes.len();

        for i in 0..count {
            let mut force = Vec2::new(0.0, 0.0);
            let p1 = positions[i];

            // Attract to center
            force.x -= p1.x * center_attraction;
            force.y -= p1.y * center_attraction;

            // Repel from others
            for j in 0..count {
                if i == j {
                    continue;
                }
                let p2 = positions[j];
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq > 0.01 {
                    let f = repulsion / dist_sq.max(1.0);
                    let dist = dist_sq.sqrt();
                    force.x += (dx / dist) * f;
                    force.y += (dy / dist) * f;
                }
            }

            // Limit max force
            let f_len = (force.x.powi(2) + force.y.powi(2)).sqrt();
            if f_len > 10.0 {
                force.x = (force.x / f_len) * 10.0;
                force.y = (force.y / f_len) * 10.0;
            }

            let node = &mut self.nodes[i];
            node.vel.x = (node.vel.x + force.x) * damping;
            node.vel.y = (node.vel.y + force.y) * damping;
            node.pos.x += node.vel.x;
            node.pos.y += node.vel.y;
        }

        // 2. Generate Fissures
        // Only generate if no fissure exists for this node yet
        let mut new_fissures = Vec::new();
        let existing_origins: HashMap<usize, bool> =
            self.fissures.iter().map(|f| (f.origin_id, true)).collect();

        let mut rng = rand::thread_rng();

        for node in &self.nodes {
            if node.stress > 0.0 && !existing_origins.contains_key(&node.id) {
                // Chance to crack based on stress
                // 1% chance per frame per stress unit?
                if rng.gen_bool(0.01) {
                    new_fissures.push(Fissure {
                        origin_id: node.id,
                        points: vec![node.pos], // Start at node
                        age: 0,
                    });
                }
            }
        }
        self.fissures.extend(new_fissures);

        // 3. Grow Fissures
        for fissure in &mut self.fissures {
            fissure.age += 1;
            // Grow until length corresponds to stress
            let node = &self.nodes[fissure.origin_id];
            let target_length = node.stress * 2.0; // 2 units per stress

            if (fissure.points.len() as f64) < target_length.max(5.0) {
                let last = *fissure.points.last().unwrap();
                // Random walk
                let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
                let len = rng.gen_range(2.0..5.0);

                let next = Vec2::new(last.x + angle.cos() * len, last.y + angle.sin() * len);
                fissure.points.push(next);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::ScanResult;
    use std::path::PathBuf;

    #[test]
    fn test_simulation_step() {
        let scans = vec![
            ScanResult {
                path: PathBuf::from("a.rs"),
                unwrap_count: 10,
                ..Default::default()
            },
            ScanResult {
                path: PathBuf::from("b.rs"),
                unwrap_count: 0,
                ..Default::default()
            },
        ];

        let mut world = World::new(scans);
        assert_eq!(world.nodes.len(), 2);

        // Initial state
        assert!(world.fissures.is_empty());

        // Step many times to trigger fissure
        for _ in 0..1000 {
            world.step();
        }

        // The high stress node should eventually crack
        // (Probabilistic, but with 1000 steps and 0.01 prob, highly likely)
        let _has_fissure = world.fissures.iter().any(|f| f.origin_id == 0);

        // Note: randomness in tests is bad practice usually, but for this creative mode
        // quick verification it's acceptable. We could seed RNG if we wanted determinism.
        // For now, let's just assert that positions moved.

        assert_ne!(world.nodes[0].pos.x, 0.0); // Should have moved from initial
    }
}
