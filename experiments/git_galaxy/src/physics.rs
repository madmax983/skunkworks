use crate::harvester::CommitData;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
    pub color: (u8, u8, u8), // RGB
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: usize, // Index in nodes vector
    pub target: usize,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    // Map hash to index
    pub node_indices: HashMap<String, usize>,
}

impl Graph {
    pub fn new(commits: Vec<CommitData>) -> Self {
        let mut nodes = Vec::new();
        let mut node_indices = HashMap::new();
        let mut edges = Vec::new();
        let mut rng = rand::thread_rng();

        // Create nodes
        for (i, commit) in commits.iter().enumerate() {
            // Random initial position to avoid stacking
            let x = rng.gen_range(-50.0..50.0);
            let y = rng.gen_range(-50.0..50.0);

            // Mass based on churn (clamped)
            let mass = (commit.churn as f64).clamp(1.0, 100.0);

            // Color based on author (hashing name)
            let color = string_to_rgb(&commit.author);

            nodes.push(Node {
                id: commit.hash.clone(),
                x,
                y,
                vx: 0.0,
                vy: 0.0,
                mass,
                color,
            });
            node_indices.insert(commit.hash.clone(), i);
        }

        // Create edges
        for (i, commit) in commits.iter().enumerate() {
            for parent_hash in &commit.parents {
                if let Some(&parent_idx) = node_indices.get(parent_hash) {
                    edges.push(Edge {
                        source: parent_idx,
                        target: i,
                    });
                }
            }
        }

        Graph {
            nodes,
            edges,
            node_indices,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let repulsion_constant = 1000.0;
        let spring_constant = 0.05;
        let rest_length = 10.0;
        let damping = 0.90;
        let center_attraction = 0.01;

        let node_count = self.nodes.len();
        let mut forces = vec![(0.0, 0.0); node_count];

        // 1. Repulsion (All vs All) - optimized slightly
        // Naive O(N^2) - okay for < 500 nodes
        for i in 0..node_count {
            for j in (i + 1)..node_count {
                let dx = self.nodes[i].x - self.nodes[j].x;
                let dy = self.nodes[i].y - self.nodes[j].y;
                let dist_sq = dx * dx + dy * dy;

                let dist = dist_sq.sqrt().max(0.1);

                // F = k / d^2
                let force = repulsion_constant / dist_sq;
                let fx = (dx / dist) * force;
                let fy = (dy / dist) * force;

                forces[i].0 += fx;
                forces[i].1 += fy;
                forces[j].0 -= fx;
                forces[j].1 -= fy;
            }
        }

        // 2. Spring Attraction (Edges)
        for edge in &self.edges {
            let u = edge.source;
            let v = edge.target;

            let dx = self.nodes[v].x - self.nodes[u].x;
            let dy = self.nodes[v].y - self.nodes[u].y;
            let dist = (dx * dx + dy * dy).sqrt().max(0.1);

            // F = k * (d - rest)
            let force = spring_constant * (dist - rest_length);
            let fx = (dx / dist) * force;
            let fy = (dy / dist) * force;

            // Pull u towards v
            forces[u].0 += fx;
            forces[u].1 += fy;
            // Pull v towards u
            forces[v].0 -= fx;
            forces[v].1 -= fy;
        }

        // 3. Center Attraction (Gravity towards 0,0)
        for (i, force) in forces.iter_mut().enumerate() {
            let dx = 0.0 - self.nodes[i].x;
            let dy = 0.0 - self.nodes[i].y;
            force.0 += dx * center_attraction;
            force.1 += dy * center_attraction;
        }

        // 4. Update Position
        for (i, node) in self.nodes.iter_mut().enumerate() {
            // F = ma -> a = F/m
            // Simplified: ignore mass for acceleration or use it?
            // Heavier nodes move slower?
            let ax = forces[i].0 / node.mass.sqrt(); // Square root damping for mass
            let ay = forces[i].1 / node.mass.sqrt();

            node.vx = (node.vx + ax * dt) * damping;
            node.vy = (node.vy + ay * dt) * damping;

            node.x += node.vx * dt;
            node.y += node.vy * dt;
        }
    }
}

// Simple hash to RGB
fn string_to_rgb(s: &str) -> (u8, u8, u8) {
    let mut hash: u32 = 0;
    for b in s.bytes() {
        hash = hash.wrapping_add(b as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);

    (
        (hash & 0xFF) as u8,
        ((hash >> 8) & 0xFF) as u8,
        ((hash >> 16) & 0xFF) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_physics() {
        let c1 = CommitData {
            hash: "a".to_string(),
            parents: vec![],
            author: "A".to_string(),
            time: 0,
            churn: 10,
        };
        let c2 = CommitData {
            hash: "b".to_string(),
            parents: vec!["a".to_string()],
            author: "B".to_string(),
            time: 1,
            churn: 10,
        };

        let commits = vec![c1, c2];
        let mut graph = Graph::new(commits);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);

        let initial_x = graph.nodes[0].x;

        // Run physics
        graph.update(0.1);

        // Nodes should have moved
        assert!(graph.nodes[0].x != initial_x || graph.nodes[0].vx != 0.0);
    }
}
