use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub _id: usize,
    pub layer: usize,
    pub x: f32, // Normalized 0.0 - 1.0
    pub y: f32, // Normalized 0.0 - 1.0
    pub build_cost: u32,
}

pub struct DependencyGraph {
    pub graph: Graph<Node, f32, Directed>, // Edge weight is pheromone
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    pub fn generate_layered_dag(&mut self, layers: usize, nodes_per_layer: usize, rng: &mut impl Rng) {
        let mut layer_nodes: Vec<Vec<NodeIndex>> = Vec::new();

        // Create Nodes
        let mut id_counter = 0;
        for l in 0..layers {
            let mut current_layer = Vec::new();
            for _ in 0..nodes_per_layer {
                let node = Node {
                    _id: id_counter,
                    layer: l,
                    x: 0.0,
                    y: 0.0,
                    build_cost: rng.gen_range(1..10),
                };
                id_counter += 1;
                current_layer.push(self.graph.add_node(node));
            }
            layer_nodes.push(current_layer);
        }

        // Create Edges (Dependencies)
        // Each node depends on 1-3 nodes from the NEXT layer (deeper dependencies).
        for l in 0..layers - 1 {
            for &node_idx in &layer_nodes[l] {
                let num_deps = rng.gen_range(1..=3);
                let potential_deps = &layer_nodes[l + 1];

                // Randomly select dependencies
                let chosen = potential_deps.choose_multiple(rng, num_deps);
                for dep_idx in chosen {
                    self.graph.add_edge(node_idx, *dep_idx, 0.0); // 0.0 initial pheromone
                }
            }
        }
    }

    pub fn calculate_layout(&mut self) {
        // Simple Layered Layout
        // X = (index_in_layer + 0.5) / layer_size
        // Y = (layer + 0.5) / num_layers

        let mut layer_counts = HashMap::new();
        let mut node_indices_by_layer = HashMap::new();

        for idx in self.graph.node_indices() {
            let node = &self.graph[idx];
            *layer_counts.entry(node.layer).or_insert(0) += 1;
            node_indices_by_layer.entry(node.layer).or_insert(Vec::new()).push(idx);
        }

        let max_layer = layer_counts.keys().max().cloned().unwrap_or(0);

        for (layer, indices) in node_indices_by_layer.iter() {
            let count = indices.len();
            for (i, &idx) in indices.iter().enumerate() {
                let node = &mut self.graph[idx];
                node.x = (i as f32 + 0.5) / count as f32;
                node.y = (*layer as f32 + 0.5) / (max_layer + 1) as f32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_graph_generation() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut graph = DependencyGraph::new();
        graph.generate_layered_dag(3, 2, &mut rng);

        assert_eq!(graph.graph.node_count(), 6);
        assert!(graph.graph.edge_count() > 0);
    }
}
