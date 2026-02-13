use crate::graph::CrateGraph;
use petgraph::graph::NodeIndex;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn distance_sq(&self, other: Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_layout() {
        let mut graph = Graph::new();
        let n1 = graph.add_node("A".to_string());
        let n2 = graph.add_node("B".to_string());
        graph.add_edge(n1, n2, ());

        let layout = Layout::new(&graph);
        assert_eq!(layout.positions.len(), 2);

        let p1 = layout.positions[&n1];
        let p2 = layout.positions[&n2];

        assert!(p1.x >= 0.0 && p1.x <= 1.0);
        assert!(p1.y >= 0.0 && p1.y <= 1.0);
        assert!(p1.z >= 0.0 && p1.z <= 1.0);

        // Ensure they are not on top of each other
        assert!(p1.distance_sq(p2) > 0.0);
    }
}

pub struct Layout {
    pub positions: HashMap<NodeIndex, Vec3>,
}

impl Layout {
    pub fn new(graph: &CrateGraph) -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = HashMap::new();

        for node in graph.node_indices() {
            positions.insert(
                node,
                Vec3::new(
                    rng.gen_range(0.2..0.8),
                    rng.gen_range(0.2..0.8),
                    rng.gen_range(0.2..0.8),
                ),
            );
        }

        let mut layout = Layout { positions };
        layout.optimize(graph, 50);
        layout
    }

    pub fn optimize(&mut self, graph: &CrateGraph, iterations: usize) {
        let repulsion = 0.005;
        let spring = 0.05;
        let center_gravity = 0.01;

        for _ in 0..iterations {
            let mut forces: HashMap<NodeIndex, Vec3> = HashMap::new();

            // Repulsion
            let nodes: Vec<NodeIndex> = graph.node_indices().collect();
            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    let u = nodes[i];
                    let v = nodes[j];
                    let p_u = self.positions[&u];
                    let p_v = self.positions[&v];

                    let delta = Vec3::new(p_u.x - p_v.x, p_u.y - p_v.y, p_u.z - p_v.z);
                    let dist_sq = p_u.distance_sq(p_v);

                    if dist_sq > 0.0001 {
                        let force = repulsion / dist_sq;
                        let dir = delta.normalize();

                        let f_u = Vec3::new(dir.x * force, dir.y * force, dir.z * force);

                        let entry = forces.entry(u).or_insert(Vec3::zero());
                        entry.x += f_u.x;
                        entry.y += f_u.y;
                        entry.z += f_u.z;

                        let entry = forces.entry(v).or_insert(Vec3::zero());
                        entry.x -= f_u.x;
                        entry.y -= f_u.y;
                        entry.z -= f_u.z;
                    }
                }
            }

            // Springs (Edges)
            for edge in graph.edge_indices() {
                if let Some((u, v)) = graph.edge_endpoints(edge) {
                    let p_u = self.positions[&u];
                    let p_v = self.positions[&v];

                    let delta = Vec3::new(p_v.x - p_u.x, p_v.y - p_u.y, p_v.z - p_u.z);
                    let dist = (delta.x * delta.x + delta.y * delta.y + delta.z * delta.z).sqrt();

                    // Hooke's Law: F = k * x
                    let force = (dist - 0.2) * spring;
                    let dir = delta.normalize();

                    let f = Vec3::new(dir.x * force, dir.y * force, dir.z * force);

                    let entry = forces.entry(u).or_insert(Vec3::zero());
                    entry.x += f.x;
                    entry.y += f.y;
                    entry.z += f.z;

                    let entry = forces.entry(v).or_insert(Vec3::zero());
                    entry.x -= f.x;
                    entry.y -= f.y;
                    entry.z -= f.z;
                }
            }

            // Gravity to center
            for u in &nodes {
                let p = self.positions[u];
                let center = Vec3::new(0.5, 0.5, 0.5);
                let delta = Vec3::new(center.x - p.x, center.y - p.y, center.z - p.z);

                let entry = forces.entry(*u).or_insert(Vec3::zero());
                entry.x += delta.x * center_gravity;
                entry.y += delta.y * center_gravity;
                entry.z += delta.z * center_gravity;
            }

            // Apply forces
            for (node, force) in forces {
                if let Some(pos) = self.positions.get_mut(&node) {
                    pos.x += force.x.clamp(-0.1, 0.1);
                    pos.y += force.y.clamp(-0.1, 0.1);
                    pos.z += force.z.clamp(-0.1, 0.1);

                    // Clamp to box
                    pos.x = pos.x.clamp(0.05, 0.95);
                    pos.y = pos.y.clamp(0.05, 0.95);
                    pos.z = pos.z.clamp(0.05, 0.95);
                }
            }
        }
    }
}
