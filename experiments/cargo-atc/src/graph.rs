use anyhow::{Context, Result};
use cargo_metadata::{MetadataCommand, PackageId};
use petgraph::graph::NodeIndex;
use petgraph::Directed;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Node {
    #[allow(dead_code)]
    pub id: PackageId,
    pub name: String,
    pub version: String,
    // Physics / Rendering
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    // Game Logic
    #[allow(dead_code)]
    pub is_workspace_member: bool,
    pub build_cost: u64, // Ticks to build
}

#[derive(Debug, Clone)]
pub struct EdgeData {
    #[allow(dead_code)]
    pub relation: String,
}

pub struct DepGraph {
    pub graph: petgraph::Graph<Node, EdgeData, Directed>,
    pub width: f64,
    pub height: f64,
}

impl DepGraph {
    pub fn new(width: f64, height: f64) -> Result<Self> {
        let metadata = MetadataCommand::new()
            .exec()
            .context("Failed to run cargo metadata")?;

        let mut graph = petgraph::Graph::<Node, EdgeData, Directed>::new();
        let mut pkg_map: HashMap<PackageId, NodeIndex> = HashMap::new();
        let mut rng = rand::thread_rng();

        // 1. Add Nodes
        // If resolve is present, use it for the exact dependency graph.
        // Otherwise fallback to packages (which might include dev-deps not used).
        let resolve = metadata.resolve.context("No resolve graph found")?;

        for node_resolve in &resolve.nodes {
            if let Some(package) = metadata.packages.iter().find(|p| p.id == node_resolve.id) {
                let x = rng.gen_range(0.0..width);
                let y = rng.gen_range(0.0..height);
                let is_root = metadata.workspace_members.contains(&package.id);

                // Heuristic for build cost: length of name * 10 + version length * 5 + random jitter
                let cost = (package.name.len() as u64 * 10)
                    + (package.version.to_string().len() as u64 * 5)
                    + rng.gen_range(0..20);

                let node = Node {
                    id: package.id.clone(),
                    name: package.name.clone(),
                    version: package.version.to_string(),
                    x,
                    y,
                    vx: 0.0,
                    vy: 0.0,
                    is_workspace_member: is_root,
                    build_cost: cost.max(10),
                };

                let idx = graph.add_node(node);
                pkg_map.insert(package.id.clone(), idx);
            }
        }

        // 2. Add Edges
        // Edge Direction: A -> B means "A depends on B".
        // To build A, B must be finished.
        for node_resolve in &resolve.nodes {
            if let Some(source_idx) = pkg_map.get(&node_resolve.id) {
                for dep in &node_resolve.dependencies {
                    if let Some(target_idx) = pkg_map.get(dep) {
                        // Check if edge exists to avoid dupes
                        if graph.find_edge(*source_idx, *target_idx).is_none() {
                            graph.add_edge(
                                *source_idx,
                                *target_idx,
                                EdgeData {
                                    relation: "dep".into(),
                                },
                            );
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

    /// Returns a list of NodeIndices that are ready to build.
    /// A node is ready if:
    /// 1. It is not already completed (not in `completed` set).
    /// 2. All its dependencies (outgoing edges) are in the `completed` set.
    pub fn get_ready_crates(&self, completed: &HashSet<NodeIndex>) -> Vec<NodeIndex> {
        self.graph
            .node_indices()
            .filter(|&idx| {
                if completed.contains(&idx) {
                    return false;
                }

                // Check all outgoing neighbors (dependencies)
                // If any dependency is NOT completed, then this crate is NOT ready.
                self.graph
                    .neighbors_directed(idx, petgraph::Direction::Outgoing)
                    .all(|dep_idx| completed.contains(&dep_idx))
            })
            .collect()
    }

    /// Physics update for the radar view (Force Directed Layout)
    pub fn update_layout(&mut self, dt: f64) {
        let repulsion = 2000.0;
        let attraction = 0.8;
        let damping = 0.85;
        let center_force = 0.05;

        let node_indices: Vec<NodeIndex> = self.graph.node_indices().collect();
        let len = node_indices.len();

        // 1. Repulsion (Nodes push apart)
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

        // 2. Attraction (Springs along edges)
        // Iterate edges
        let edges: Vec<(NodeIndex, NodeIndex)> = self
            .graph
            .edge_indices()
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

            // F = k * x
            let fx = dx * attraction;
            let fy = dy * attraction;

            self.graph[idx1].vx += fx * dt;
            self.graph[idx1].vy += fy * dt;
            self.graph[idx2].vx -= fx * dt;
            self.graph[idx2].vy -= fy * dt;
        }

        // 3. Center Gravity (Keep in view)
        let cx = self.width / 2.0;
        let cy = self.height / 2.0;
        for idx in &node_indices {
            let node = &mut self.graph[*idx];
            let dx = cx - node.x;
            let dy = cy - node.y;
            node.vx += dx * center_force * dt;
            node.vy += dy * center_force * dt;
        }

        // 4. Integration
        for idx in node_indices {
            let node = &mut self.graph[idx];

            node.vx *= damping;
            node.vy *= damping;

            node.x += node.vx * dt;
            node.y += node.vy * dt;

            // Bounds clamping
            node.x = node.x.clamp(0.0, self.width);
            node.y = node.y.clamp(0.0, self.height);
        }
    }
}
