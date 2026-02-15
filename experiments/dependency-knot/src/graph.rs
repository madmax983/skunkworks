use anyhow::{Context, Result};
use cargo_metadata::{MetadataCommand, PackageId};
use petgraph::algo::toposort;
use petgraph::Graph;
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::math::{torus_knot, Instance, Vertex};

pub struct GraphData {
    pub instances: Vec<Instance>,
    pub edge_vertices: Vec<Vertex>,
}

pub fn build_graph_data(p: f32, q: f32) -> Result<GraphData> {
    // 1. Get Metadata
    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to run cargo metadata")?;

    let mut graph = Graph::<PackageId, ()>::new();
    let mut pkg_to_node = HashMap::new();

    // 2. Build Graph Nodes
    for pkg in &metadata.packages {
        let node = graph.add_node(pkg.id.clone());
        pkg_to_node.insert(pkg.id.clone(), node);
    }

    // 3. Build Graph Edges
    if let Some(resolve) = &metadata.resolve {
        for node in &resolve.nodes {
            if let Some(source) = pkg_to_node.get(&node.id) {
                for dep in &node.dependencies {
                    if let Some(target) = pkg_to_node.get(dep) {
                        // Check if edge already exists to avoid duplicates?
                        // Petgraph allows parallel edges.
                        // cargo_metadata resolve might duplicate? No.
                        graph.update_edge(*source, *target, ());
                    }
                }
            }
        }
    }

    // 4. Layout
    // Topological sort to find linear order
    // toposort returns Vec<NodeIndex>
    let sorted_nodes = match toposort(&graph, None) {
        Ok(nodes) => nodes,
        Err(_) => {
            // Cycle detected. Fallback to arbitrary order.
            graph.node_indices().collect()
        }
    };

    let num_nodes = sorted_nodes.len();
    let mut instances = Vec::with_capacity(num_nodes);
    let mut node_positions = HashMap::new();

    // Map nodes to the knot curve
    for (i, node_idx) in sorted_nodes.iter().enumerate() {
        let t = (i as f32 / num_nodes as f32) * 2.0 * PI;
        let pos = torus_knot(p, q, t);

        node_positions.insert(*node_idx, pos);

        let pkg_id = &graph[*node_idx];
        // Color based on whether it's a workspace member
        let is_workspace = metadata.workspace_members.contains(pkg_id);

        let color = if is_workspace {
            [0.2, 1.0, 0.2, 1.0] // Green for workspace
        } else {
            [1.0, 0.2, 0.2, 1.0] // Red for external
        };

        instances.push(Instance {
            model_pos: [pos.x, pos.y, pos.z],
            color,
        });
    }

    // Generate edges
    let mut edge_vertices = Vec::new();

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        // If node was not in sorted_nodes (shouldn't happen), skip
        if !node_positions.contains_key(&source) || !node_positions.contains_key(&target) {
            continue;
        }

        let p0 = node_positions[&source];
        let p1 = node_positions[&target];

        // Draw a line
        let v0 = Vertex {
            position: [p0.x, p0.y, p0.z],
            normal: [0.0, 1.0, 0.0],
            uv: [0.0, 0.0],
        };
        let v1 = Vertex {
            position: [p1.x, p1.y, p1.z],
            normal: [0.0, 1.0, 0.0],
            uv: [1.0, 0.0],
        };

        edge_vertices.push(v0);
        edge_vertices.push(v1);
    }

    Ok(GraphData {
        instances,
        edge_vertices,
    })
}
