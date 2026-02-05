use crate::graph::DependencyGraph;
use crate::ant::Ant;
use petgraph::graph::EdgeIndex;
use petgraph::Direction;
use petgraph::visit::EdgeRef;
use rand::prelude::*;
use std::collections::HashMap;

pub struct Simulation {
    pub graph: DependencyGraph,
    pub ants: Vec<Ant>,
    pub pheromones: HashMap<EdgeIndex, f32>,
}

impl Simulation {
    pub fn new(graph: DependencyGraph, num_ants: usize, _rng: &mut impl Rng) -> Self {
        // Find a root node (layer 0). If multiple, pick first.
        let root_node = graph.graph.node_indices()
            .find(|&i| graph.graph[i].layer == 0)
            .expect("Graph must have a layer 0 node");

        let mut ants = Vec::new();
        for _ in 0..num_ants {
            ants.push(Ant::new(root_node));
        }

        Self {
            graph,
            ants,
            pheromones: HashMap::new(),
        }
    }

    pub fn step(&mut self, rng: &mut impl Rng) {
        let Simulation { graph, ants, pheromones, .. } = self;

        // Evaporation
        for p in pheromones.values_mut() {
            *p *= 0.95;
            if *p < 0.01 { *p = 0.0; }
        }

        for ant in ants {
            if ant.carrying_artifact {
                // Return home logic
                if let Some(edge_idx) = ant.path_history.pop() {
                     // Get endpoints to verify movement
                     if let Some((source, _target)) = graph.graph.edge_endpoints(edge_idx) {
                         // We are at target, moving to source (backtracking)
                         ant.current_node = source;

                         // Deposit Pheromone
                         *pheromones.entry(edge_idx).or_insert(0.0) += 2.0;
                     }
                } else {
                    // Back at root with no history
                    ant.carrying_artifact = false;
                }
            } else {
                // Foraging logic
                let neighbors: Vec<_> = graph.graph
                    .edges_directed(ant.current_node, Direction::Outgoing)
                    .collect();

                if neighbors.is_empty() {
                    // Leaf reached
                    ant.carrying_artifact = true;
                } else {
                    // Probabilistic selection
                    let epsilon = 0.1;
                    let weights: Vec<f32> = neighbors.iter()
                        .map(|e| {
                            let p = *pheromones.get(&e.id()).unwrap_or(&0.0);
                            let target_node = &graph.graph[e.target()];
                            let heuristic = 10.0 / (target_node.build_cost as f32).max(1.0);
                            p + heuristic + epsilon
                        })
                        .collect();

                    if let Ok(dist) = rand::distributions::WeightedIndex::new(&weights) {
                        let chosen_idx = dist.sample(rng);
                        let chosen_edge = neighbors[chosen_idx];

                        ant.path_history.push(chosen_edge.id());
                        ant.current_node = chosen_edge.target();
                    } else {
                        // Fallback if weights are weird
                         let chosen_idx = rng.gen_range(0..neighbors.len());
                         let chosen_edge = neighbors[chosen_idx];
                         ant.path_history.push(chosen_edge.id());
                         ant.current_node = chosen_edge.target();
                    }
                }
            }
        }
    }
}
