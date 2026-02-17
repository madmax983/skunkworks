use macroquad::prelude::*;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use crate::graph::DependencyGraph;

pub struct Layout {
    pub positions: HashMap<NodeIndex, Vec3>,
    pub velocity: HashMap<NodeIndex, Vec3>,
}

impl Layout {
    pub fn new(graph: &DependencyGraph) -> Self {
        let mut positions = HashMap::new();
        let mut velocity = HashMap::new();

        for node in graph.graph.node_indices() {
             positions.insert(node, vec3(
                 rand::gen_range(0.0, 100.0),
                 rand::gen_range(0.0, 100.0),
                 rand::gen_range(0.0, 100.0)
             ));
             velocity.insert(node, Vec3::ZERO);
        }
        Self { positions, velocity }
    }

    pub fn update(&mut self, graph: &DependencyGraph) {
        let repulsion_force = 1000.0;
        let attraction_force = 0.05;
        let damping = 0.85;
        let time_step = 0.1;

        // Apply Repulsion (Naive O(N^2) - optimize if slow)
        // With 1500 nodes, N^2 is 2.25M.
        // We can optimize by only checking random subset of neighbors if too slow.
        // Let's try full loop first.

        let nodes: Vec<NodeIndex> = self.positions.keys().cloned().collect();
        let count = nodes.len();

        // Accumulate forces
        let mut forces: HashMap<NodeIndex, Vec3> = HashMap::new();

        // Parallelizing this is hard without rayon.
        // Let's use a spatial grid or just simple random sampling for repulsion?
        // Let's do random sampling for repulsion: each node repels 50 random other nodes.

        for &u in &nodes {
            let mut f = Vec3::ZERO;
            let pos_u = self.positions[&u];

            // Random sampling for repulsion
            for _ in 0..20 {
                let v = nodes[rand::gen_range(0, count)];
                if u == v { continue; }
                let pos_v = self.positions[&v];
                let delta = pos_u - pos_v;
                let dist_sq = delta.length_squared();
                if dist_sq > 0.1 && dist_sq < 1000.0 {
                    f += delta.normalize() * (repulsion_force / dist_sq);
                }
            }
            forces.insert(u, f);
        }

        // Apply Attraction
        for edge in graph.graph.edge_indices() {
            if let Some((u, v)) = graph.graph.edge_endpoints(edge) {
                if let (Some(pos_u), Some(pos_v)) = (self.positions.get(&u), self.positions.get(&v)) {
                    let delta = *pos_v - *pos_u;
                    let dist = delta.length();
                    let f = delta.normalize() * (dist * attraction_force);

                    if let Some(fu) = forces.get_mut(&u) { *fu += f; }
                    if let Some(fv) = forces.get_mut(&v) { *fv -= f; }
                }
            }
        }

        // Update Positions
        for &u in &nodes {
            if let Some(f) = forces.get(&u) {
                let vel = self.velocity.get_mut(&u).unwrap();
                *vel += *f * time_step;
                *vel *= damping;

                let pos = self.positions.get_mut(&u).unwrap();
                *pos += *vel * time_step;

                // Centering force
                let center_dist = *pos - vec3(50.0, 50.0, 50.0);
                *pos -= center_dist * 0.01;
            }
        }
    }
}
