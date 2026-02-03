use crate::graph::DepGraph;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use rand::Rng;

pub struct Ant {
    pub current_node: NodeIndex,
    pub target_edge: Option<EdgeIndex>,
    pub progress: f64, // 0.0 to 1.0 along the edge
    pub speed: f64,
}

pub struct World {
    pub dep_graph: DepGraph,
    pub ants: Vec<Ant>,
}

impl World {
    pub fn new(dep_graph: DepGraph, ant_count: usize) -> Self {
        let mut ants = Vec::new();
        let mut rng = rand::thread_rng();

        // Find leaf nodes (no outgoing edges = no dependencies)
        // In our graph A->B means A depends on B.
        // So B is a leaf dependency if it has no outgoing edges (it depends on nothing).
        let leaves: Vec<NodeIndex> = dep_graph
            .graph
            .node_indices()
            .filter(|&idx| {
                dep_graph
                    .graph
                    .neighbors_directed(idx, Direction::Outgoing)
                    .count()
                    == 0
            })
            .collect();

        if leaves.is_empty() {
            // Fallback if circular or something weird: just pick random nodes
            let all_nodes: Vec<NodeIndex> = dep_graph.graph.node_indices().collect();
            for _ in 0..ant_count {
                if let Some(&node) = all_nodes.get(rng.gen_range(0..all_nodes.len())) {
                    ants.push(Ant {
                        current_node: node,
                        target_edge: None,
                        progress: 0.0,
                        speed: rng.gen_range(0.5..1.5),
                    });
                }
            }
        } else {
            for _ in 0..ant_count {
                let node = leaves[rng.gen_range(0..leaves.len())];
                ants.push(Ant {
                    current_node: node,
                    target_edge: None,
                    progress: 0.0,
                    speed: rng.gen_range(0.5..1.5),
                });
            }
        }

        Self { dep_graph, ants }
    }

    pub fn tick(&mut self, dt: f64) {
        // Update Physics
        self.dep_graph.update_layout();

        // Update Ants
        let mut rng = rand::thread_rng();

        // We can't iterate and mutate ants easily if we also need to mutate the graph (pheromones)
        // But ants store state internally, so we can iterate ants mutably.
        // We need read access to graph to decide movement, and write access to graph for pheromones.
        // This splits the borrow.

        // Let's do pheromone decay first
        for edge_idx in self.dep_graph.graph.edge_indices() {
            if let Some(edge) = self.dep_graph.graph.edge_weight_mut(edge_idx) {
                edge.pheromone *= 0.98; // Decay
            }
        }

        let graph = &self.dep_graph.graph;

        for ant in &mut self.ants {
            if let Some(edge_idx) = ant.target_edge {
                // Moving along an edge
                ant.progress += ant.speed * dt;

                // Add pheromone trail
                // We need to use unsafe or RefCell if we want to mutate graph while iterating ants?
                // Or we can collect pheromone updates and apply them later.
                // For simplicity, let's just not update pheromone *during* movement here,
                // or use a separate loop.
                // Actually, let's skip pheromone writing inside this loop to keep borrow checker happy
                // and just assume "arrival" deposits pheromone.

                if ant.progress >= 1.0 {
                    // Arrived at destination
                    // The edge is Source -> Target (A -> B).
                    // Ant is moving Target -> Source (B -> A).
                    // So if we are traversing `edge_idx`, the 'source' of the edge is our new node.
                    let (source, _) = graph.edge_endpoints(edge_idx).unwrap();
                    ant.current_node = source;
                    ant.target_edge = None;
                    ant.progress = 0.0;

                    // Reached a root? (No incoming edges? i.e. nobody depends on me?)
                    // In A->B, A is the dependent. If A has no incoming edges, it means nobody depends on A.
                    // A is a root crate (like the workspace member).
                    // Wait, A->B means A depends on B.
                    // Ant moves B->A.
                    // If A has Incomming edges (C->A), then C depends on A. Ant continues to C.
                    // If A has NO Incoming edges, nobody depends on A. A is a final artifact.

                    if graph
                        .neighbors_directed(ant.current_node, Direction::Incoming)
                        .count()
                        == 0
                    {
                        // Respawn at a leaf
                        let leaves: Vec<NodeIndex> = graph
                            .node_indices()
                            .filter(|&idx| {
                                graph.neighbors_directed(idx, Direction::Outgoing).count() == 0
                            })
                            .collect();
                        if !leaves.is_empty() {
                            ant.current_node = leaves[rng.gen_range(0..leaves.len())];
                        }
                    }
                }
            } else {
                // Needs to pick a next edge
                // Ant is at `current_node`.
                // Needs to find a node that depends on `current_node`.
                // In A->B, A depends on B. So we look for edges ending at `current_node`.
                // Incoming edges.

                let mut incoming_edges: Vec<EdgeIndex> = Vec::new();
                for edge in graph.edges_directed(ant.current_node, Direction::Incoming) {
                    incoming_edges.push(edge.id());
                }

                if incoming_edges.is_empty() {
                    // Dead end (should be a root, but maybe handled above).
                    // Respawn
                    let leaves: Vec<NodeIndex> = graph
                        .node_indices()
                        .filter(|&idx| {
                            graph.neighbors_directed(idx, Direction::Outgoing).count() == 0
                        })
                        .collect();
                    if !leaves.is_empty() {
                        ant.current_node = leaves[rng.gen_range(0..leaves.len())];
                    }
                } else {
                    // Pick one weighted by pheromone?
                    // For now random
                    ant.target_edge = Some(incoming_edges[rng.gen_range(0..incoming_edges.len())]);
                }
            }
        }

        // Apply pheromones for active trails
        // We can do this by iterating ants again and seeing which edge they are on.
        // This avoids the borrow conflict.
        let active_edges: Vec<EdgeIndex> = self.ants.iter().filter_map(|a| a.target_edge).collect();

        for edge_idx in active_edges {
            if let Some(edge) = self.dep_graph.graph.edge_weight_mut(edge_idx) {
                edge.pheromone += 0.5; // Deposit
                if edge.pheromone > 10.0 {
                    edge.pheromone = 10.0;
                } // Cap
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::DepGraph;

    #[test]
    fn test_world_tick() {
        let graph = DepGraph::new(100.0, 100.0).expect("Graph init failed");
        let mut world = World::new(graph, 10);

        assert_eq!(world.ants.len(), 10);

        world.tick(0.1);

        // Verify at least some ants exist
        assert_eq!(world.ants.len(), 10);
    }
}
