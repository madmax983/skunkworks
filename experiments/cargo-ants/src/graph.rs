use anyhow::Result;
use cargo_metadata::{MetadataCommand, PackageId};
use petgraph::graph::{NodeIndex, UnGraph};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub version: String,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub is_root: bool,
}

#[derive(Debug, Clone)]
pub struct EdgeData {
    pub pheromone: f64,
}

pub struct DepGraph {
    pub graph: UnGraph<Node, EdgeData>,
    pub width: f64,
    pub height: f64,
}

impl DepGraph {
    pub fn new(width: f64, height: f64) -> Result<Self> {
        let metadata = MetadataCommand::new().exec()?;

        let mut graph = UnGraph::<Node, EdgeData>::default();
        let mut pkg_map: HashMap<PackageId, NodeIndex> = HashMap::new();
        let mut rng = rand::thread_rng();

        // Add nodes
        for package in &metadata.packages {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);

            let is_root = metadata.workspace_members.contains(&package.id);

            let node = Node {
                name: package.name.clone(),
                version: package.version.to_string(),
                x,
                y,
                vx: 0.0,
                vy: 0.0,
                is_root,
            };

            let idx = graph.add_node(node);
            pkg_map.insert(package.id.clone(), idx);
        }

        // Add edges
        for package in &metadata.packages {
            if let Some(source_idx) = pkg_map.get(&package.id) {
                for dep in &package.dependencies {
                    // Find the package that matches this dependency
                    // This is a bit heuristic since `dependencies` only has name/versionreq
                    // We need to look up in `resolve` ideally, but simple name matching works for basic viz
                    if let Some(target_pkg) = metadata.packages.iter().find(|p| p.name == dep.name) {
                        if let Some(target_idx) = pkg_map.get(&target_pkg.id) {
                            if !graph.contains_edge(*source_idx, *target_idx) {
                                graph.add_edge(*source_idx, *target_idx, EdgeData { pheromone: 0.0 });
                            }
                        }
                    }
                }
            }
        }

        // If we have a resolve graph, use it for better accuracy?
        // cargo_metadata provides `resolve` which has exact edges.
        // Let's try to use `resolve` if available, otherwise fallback.
        if let Some(resolve) = &metadata.resolve {
            graph.clear();
            pkg_map.clear();

            // Re-add nodes based on packages present in resolve
            for node in &resolve.nodes {
                 if let Some(package) = metadata.packages.iter().find(|p| p.id == node.id) {
                    let x = rng.gen_range(0.0..width);
                    let y = rng.gen_range(0.0..height);
                    let is_root = metadata.workspace_members.contains(&package.id);

                    let node_data = Node {
                        name: package.name.clone(),
                        version: package.version.to_string(),
                        x,
                        y,
                        vx: 0.0,
                        vy: 0.0,
                        is_root,
                    };
                    let idx = graph.add_node(node_data);
                    pkg_map.insert(package.id.clone(), idx);
                 }
            }

            for node in &resolve.nodes {
                if let Some(source_idx) = pkg_map.get(&node.id) {
                    for dep in &node.dependencies {
                        if let Some(target_idx) = pkg_map.get(dep) {
                             if !graph.contains_edge(*source_idx, *target_idx) {
                                graph.add_edge(*source_idx, *target_idx, EdgeData { pheromone: 0.0 });
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            graph,
            width,
            height,
        })
    }

    pub fn update_layout(&mut self) {
        let repulsion = 500.0;
        let attraction = 0.05;
        let damping = 0.9;
        let dt = 0.1;

        let node_indices: Vec<NodeIndex> = self.graph.node_indices().collect();
        let len = node_indices.len();

        // Repulsion
        for i in 0..len {
            for j in (i + 1)..len {
                let idx1 = node_indices[i];
                let idx2 = node_indices[j];

                let (p1, p2) = {
                    let n1 = &self.graph[idx1];
                    let n2 = &self.graph[idx2];
                    ((n1.x, n1.y), (n2.x, n2.y))
                };

                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq > 0.1 {
                    let dist = dist_sq.sqrt();
                    let f = repulsion / dist_sq;
                    let fx = (dx / dist) * f;
                    let fy = (dy / dist) * f;

                    self.graph[idx1].vx += fx * dt;
                    self.graph[idx1].vy += fy * dt;
                    self.graph[idx2].vx -= fx * dt;
                    self.graph[idx2].vy -= fy * dt;
                }
            }
        }

        // Attraction (Edges)
        // Clone edges to iterate to avoid borrow checker issues
        let edges: Vec<(NodeIndex, NodeIndex)> = self.graph.edge_indices()
            .map(|e| self.graph.edge_endpoints(e).unwrap())
            .collect();

        for (idx1, idx2) in edges {
            let (p1, p2) = {
                let n1 = &self.graph[idx1];
                let n2 = &self.graph[idx2];
                ((n1.x, n1.y), (n2.x, n2.y))
            };

            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;

            // Hooke's law: F = k * x
            let fx = dx * attraction;
            let fy = dy * attraction;

            self.graph[idx1].vx += fx * dt;
            self.graph[idx1].vy += fy * dt;
            self.graph[idx2].vx -= fx * dt;
            self.graph[idx2].vy -= fy * dt;
        }

        // Gravity towards center to keep things in view
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;

        for idx in &node_indices {
             let node = &mut self.graph[*idx];
             let dx = center_x - node.x;
             let dy = center_y - node.y;
             node.vx += dx * 0.01 * dt;
             node.vy += dy * 0.01 * dt;
        }

        // Update Position & Bounds Check
        for idx in node_indices {
            let node = &mut self.graph[idx];

            node.vx *= damping;
            node.vy *= damping;

            node.x += node.vx * dt;
            node.y += node.vy * dt;

            // Simple bounce off walls
            if node.x < 0.0 { node.x = 0.0; node.vx *= -1.0; }
            if node.x > self.width { node.x = self.width; node.vx *= -1.0; }
            if node.y < 0.0 { node.y = 0.0; node.vy *= -1.0; }
            if node.y > self.height { node.y = self.height; node.vy *= -1.0; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = DepGraph::new(100.0, 100.0);
        assert!(graph.is_ok(), "Failed to create graph: {:?}", graph.err());
        let graph = graph.unwrap();
        assert!(graph.graph.node_count() > 0);
        println!("Node count: {}", graph.graph.node_count());
    }
}
