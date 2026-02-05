use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use rand::prelude::*;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeDynamicState {
    pub progress: f32, // 0.0 to 1.0
    pub builder_id: Option<usize>, // Ant ID
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub layer: usize,
    pub x: f32,
    pub y: f32,
    pub build_cost: u32, // Time in ms to build
    pub state: Mutex<NodeDynamicState>,
}

#[derive(Debug)]
pub struct Edge {
    pub pheromone: Mutex<f32>,
}

pub type DepGraph = Graph<Node, Edge, Directed>;

pub fn generate_layered_dag(layers: usize, nodes_per_layer: usize, rng: &mut impl Rng) -> DepGraph {
    let mut graph = Graph::new();
    let mut layer_nodes: Vec<Vec<NodeIndex>> = Vec::new();

    // Create Nodes
    let mut id_counter = 0;
    for l in 0..layers {
        let mut current_layer = Vec::new();
        for _ in 0..nodes_per_layer {
            let node = Node {
                id: id_counter,
                layer: l,
                x: 0.0,
                y: 0.0,
                build_cost: rng.gen_range(100..500), // 100-500ms build time
                state: Mutex::new(NodeDynamicState {
                    progress: 0.0,
                    builder_id: None,
                }),
            };
            id_counter += 1;
            current_layer.push(graph.add_node(node));
        }
        layer_nodes.push(current_layer);
    }

    // Create Edges (Dependencies)
    // Connect to next layer
    for l in 0..layers - 1 {
        for &node_idx in &layer_nodes[l] {
            let num_deps = rng.gen_range(1..=3);
            let potential_deps = &layer_nodes[l + 1];

            let chosen = potential_deps.choose_multiple(rng, num_deps);
            for dep_idx in chosen {
                graph.add_edge(node_idx, *dep_idx, Edge { pheromone: Mutex::new(0.0) });
            }
        }
    }

    // Calculate Layout
    let mut layer_counts = HashMap::new();
    let mut node_indices_by_layer = HashMap::new();

    for idx in graph.node_indices() {
        let node = &graph[idx];
        *layer_counts.entry(node.layer).or_insert(0) += 1;
        node_indices_by_layer.entry(node.layer).or_insert(Vec::new()).push(idx);
    }

    let max_layer = layer_counts.keys().max().cloned().unwrap_or(0);

    for (layer, indices) in node_indices_by_layer.iter() {
        let count = indices.len();
        // Sort indices by id to keep stable layout
        // (Actually indices are random access, but iteration order might vary if using keys, but here it's Vec)
        for (i, &idx) in indices.iter().enumerate() {
            let node = &mut graph[idx];
            node.x = (i as f32 + 0.5) / count as f32;
            node.y = (*layer as f32 + 0.5) / (max_layer as f32 + 1.0);
        }
    }

    graph
}
