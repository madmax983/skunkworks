use crate::harvester::CommitData;
use rand::Rng;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn mag_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.mag();
        if m > 0.0 {
            *self / m
        } else {
            *self
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub author: String,
    pub pos: Vec2,
    pub vel: Vec2,
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
                author: commit.author.clone(),
                pos: Vec2::new(x, y),
                vel: Vec2::zero(),
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
        let node_count = self.nodes.len();
        let mut forces = vec![Vec2::zero(); node_count];

        self.apply_repulsion(&mut forces);
        self.apply_springs(&mut forces);
        self.apply_center_gravity(&mut forces);
        self.integrate(dt, &forces);
    }

    fn apply_repulsion(&self, forces: &mut [Vec2]) {
        const REPULSION_CONSTANT: f64 = 1000.0;
        let node_count = self.nodes.len();

        for i in 0..node_count {
            for j in (i + 1)..node_count {
                let delta = self.nodes[i].pos - self.nodes[j].pos;
                let dist_sq = delta.mag_sq();
                // Avoid division by zero and extreme forces
                let dist = dist_sq.sqrt().max(0.1);

                // F = k / d^2
                let force_mag = REPULSION_CONSTANT / (dist * dist); // dist_sq matches original logic? Original was dist_sq
                let force = delta / dist * force_mag;

                forces[i] = forces[i] + force;
                forces[j] = forces[j] - force;
            }
        }
    }

    fn apply_springs(&self, forces: &mut [Vec2]) {
        const SPRING_CONSTANT: f64 = 0.05;
        const REST_LENGTH: f64 = 10.0;

        for edge in &self.edges {
            let u = edge.source;
            let v = edge.target;

            let delta = self.nodes[v].pos - self.nodes[u].pos;
            let dist = delta.mag().max(0.1);

            // F = k * (d - rest)
            let force_mag = SPRING_CONSTANT * (dist - REST_LENGTH);
            let force = delta / dist * force_mag;

            // Pull u towards v
            forces[u] = forces[u] + force;
            // Pull v towards u
            forces[v] = forces[v] - force;
        }
    }

    fn apply_center_gravity(&self, forces: &mut [Vec2]) {
        const CENTER_ATTRACTION: f64 = 0.01;

        for (i, force) in forces.iter_mut().enumerate() {
            let delta = Vec2::zero() - self.nodes[i].pos;
            *force = *force + delta * CENTER_ATTRACTION;
        }
    }

    fn integrate(&mut self, dt: f64, forces: &[Vec2]) {
        const DAMPING: f64 = 0.90;

        for (i, node) in self.nodes.iter_mut().enumerate() {
            // F = ma -> a = F/m
            let accel = forces[i] / node.mass.sqrt();

            node.vel = (node.vel + accel * dt) * DAMPING;
            node.pos = node.pos + node.vel * dt;
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

    // Use hash to generate Hue (0-360)
    let hue = (hash % 360) as f64;
    // Saturation and Value high for distinct colors
    hsv_to_rgb(hue, 0.8, 0.95)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
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

        let initial_x = graph.nodes[0].pos.x;

        // Run physics
        graph.update(0.1);

        // Nodes should have moved
        assert!(graph.nodes[0].pos.x != initial_x || graph.nodes[0].vel.x != 0.0);
    }
}
