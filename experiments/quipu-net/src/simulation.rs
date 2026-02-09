use crate::graph::DependencyGraph;
use crate::quipu::Quipu;
use crate::serializer::to_quipu;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Packet {
    pub payload: Quipu,
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub current_edge: Option<EdgeIndex>,
    pub progress: f32, // 0.0 to 1.0
    pub speed: f32,
    pub message: String, // Keep the original string for display
}

pub struct Simulation {
    pub graph: DependencyGraph,
    pub packets: Vec<Packet>,
}

impl Simulation {
    pub fn new(graph: DependencyGraph) -> Self {
        Self {
            graph,
            packets: Vec::new(),
        }
    }

    pub fn step(&mut self, rng: &mut impl Rng) {
        // Move packets
        let mut finished_indices = Vec::new();

        for (i, packet) in self.packets.iter_mut().enumerate() {
            // Update position
            if packet.current_edge.is_some() {
                packet.progress += packet.speed;
            } else {
                // Just spawned, find first edge
                let outgoing: Vec<_> = self.graph.graph.edges(packet.source).collect();
                if outgoing.is_empty() {
                    finished_indices.push(i);
                } else {
                    let next_edge = outgoing.choose(rng).unwrap();
                    packet.current_edge = Some(next_edge.id());
                    packet.progress = 0.0;
                }
            }

            // Check arrival
            if packet.progress >= 1.0 {
                 if let Some(edge_idx) = packet.current_edge {
                    let endpoints = self.graph.graph.edge_endpoints(edge_idx).unwrap();
                    let current_node = endpoints.1; // Target of the edge is where we are now

                    // Move to next edge
                    let outgoing: Vec<_> = self.graph.graph.edges(current_node).collect();
                    if outgoing.is_empty() {
                         finished_indices.push(i); // Reached end of line
                    } else {
                        let next_edge = outgoing.choose(rng).unwrap();
                        packet.current_edge = Some(next_edge.id());
                        packet.progress = 0.0;
                    }
                 }
            }
        }

        // Remove finished
        for i in finished_indices.into_iter().rev() {
            self.packets.swap_remove(i);
        }

        // Spawn new packets randomly
        if rng.gen_bool(0.1) {
            self.spawn_random_packet(rng);
        }
    }

    fn spawn_random_packet(&mut self, rng: &mut impl Rng) {
        // Find a node in layer 0
        let layer_0: Vec<NodeIndex> = self.graph.graph.node_indices()
            .filter(|&i| self.graph.graph[i].layer == 0)
            .collect();

        if let Some(&start_node) = layer_0.choose(rng) {
             // Create a random message (just a number for now)
             let val = rng.gen::<u64>();
             let msg = format!("DATA-{:x}", val);

             if let Ok(q) = to_quipu(&val) {
                 // Calculate speed based on complexity (number of knots)
                 let total_knots: usize = q.pendants.iter().map(|p| p.knots.len()).sum();
                 // More knots = heavier = slower
                 let speed = 0.05 / (1.0 + (total_knots as f32 * 0.1));

                 self.packets.push(Packet {
                     payload: q,
                     source: start_node,
                     target: NodeIndex::new(9999), // Dummy target
                     current_edge: None,
                     progress: 0.0,
                     speed,
                     message: msg,
                 });
             }
        }
    }
}
