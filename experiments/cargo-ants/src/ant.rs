use glam::Vec2;
use petgraph::graph::{NodeIndex, EdgeIndex};
use petgraph::visit::EdgeRef;
use rand::Rng;
use crate::graph::DependencyGraph;
use crate::pheromone::PheromoneMap;

#[derive(Clone, PartialEq, Debug)]
pub enum AntState {
    Searching,
    Returning,
    Dead,
}

pub struct Ant {
    pub position: Vec2,
    pub current_node: NodeIndex,
    pub target_node: Option<NodeIndex>,
    pub target_edge: Option<EdgeIndex>,
    pub path_history: Vec<EdgeIndex>,
    pub state: AntState,
    pub progress: f32, // 0.0 to 1.0 along the edge
    pub speed: f32,
}

impl Ant {
    pub fn new(start_node: NodeIndex, start_pos: Vec2) -> Self {
        Ant {
            position: start_pos,
            current_node: start_node,
            target_node: None,
            target_edge: None,
            path_history: Vec::new(),
            state: AntState::Searching,
            progress: 0.0,
            speed: 3.0, // Tuned for better visualization
        }
    }

    pub fn update(&mut self, graph: &DependencyGraph, pheromones: &PheromoneMap) -> Option<(EdgeIndex, f32)> {
        match self.state {
            AntState::Searching => self.update_searching(graph, pheromones),
            AntState::Returning => self.update_returning(graph),
            AntState::Dead => None,
        }
    }

    fn update_searching(&mut self, graph: &DependencyGraph, pheromones: &PheromoneMap) -> Option<(EdgeIndex, f32)> {
        if let Some(target) = self.target_node {
            // Move towards target
            let start_pos = graph.graph[self.current_node].position;
            let end_pos = graph.graph[target].position;
            let dist = start_pos.distance(end_pos);

            if dist < 0.001 {
                self.progress = 1.0;
            } else {
                self.progress += self.speed / dist;
            }
            self.position = start_pos.lerp(end_pos, self.progress.min(1.0));

            if self.progress >= 1.0 {
                // Arrived
                // Store edge in history
                if let Some(edge) = self.target_edge {
                    self.path_history.push(edge);
                }

                self.current_node = target;
                self.target_node = None;
                self.target_edge = None;
                self.progress = 0.0;

                // Check if conflict
                if graph.graph[self.current_node].is_conflict {
                    self.state = AntState::Dead;
                    return None;
                }

                // Check if leaf (no outgoing edges)
                let outgoing_count = graph.graph.edges(self.current_node).count();
                if outgoing_count == 0 {
                    self.state = AntState::Returning;
                }
            }
        } else {
            // Choose next node
            let mut rng = rand::thread_rng();
            let edges: Vec<_> = graph.graph.edges(self.current_node).collect();

            if edges.is_empty() {
                // Already checked leaf, but purely defensive
                self.state = AntState::Returning;
                return None;
            }

            // Calculate probabilities
            let mut total_weight = 0.0;
            let weights: Vec<f32> = edges.iter().map(|edge| {
                let p = pheromones.get(edge.id());
                let base = 0.1;
                let w = (base + p).powf(2.0); // Pheromone importance
                total_weight += w;
                w
            }).collect();

            if total_weight <= 0.0 {
                 // Should not happen with base > 0
                 self.target_node = Some(edges[0].target());
                 self.target_edge = Some(edges[0].id());
            } else {
                let choice = rng.gen_range(0.0..total_weight);
                let mut current = 0.0;
                let mut selected = false;

                for (i, w) in weights.iter().enumerate() {
                    current += w;
                    if current >= choice {
                        self.target_node = Some(edges[i].target());
                        self.target_edge = Some(edges[i].id());
                        selected = true;
                        break;
                    }
                }
                if !selected {
                     self.target_node = Some(edges[0].target());
                     self.target_edge = Some(edges[0].id());
                }
            }
        }
        None
    }

    fn update_returning(&mut self, graph: &DependencyGraph) -> Option<(EdgeIndex, f32)> {
        if let Some(target) = self.target_node {
             let start_pos = graph.graph[self.current_node].position;
             let end_pos = graph.graph[target].position;
             let dist = start_pos.distance(end_pos);

             if dist < 0.001 {
                 self.progress = 1.0;
             } else {
                 self.progress += (self.speed * 2.0) / dist; // Return faster
             }
             self.position = start_pos.lerp(end_pos, self.progress.min(1.0));

             if self.progress >= 1.0 {
                 self.current_node = target;
                 self.target_node = None;
                 self.progress = 0.0;

                 // Deposit pheromone on the edge we just traversed
                 if let Some(edge) = self.target_edge {
                     self.target_edge = None;
                     return Some((edge, 10.0)); // Deposit 10.0
                 }
                 self.target_edge = None;
             }
        } else {
            // Pop from history
            if let Some(edge_idx) = self.path_history.pop() {
                // We are at the target of this edge, want to go to source
                if let Some((source, target)) = graph.graph.edge_endpoints(edge_idx) {
                    if target != self.current_node {
                        // Recover: Teleport to source
                        self.current_node = source;
                    } else {
                        self.target_node = Some(source);
                        self.target_edge = Some(edge_idx);
                    }
                } else {
                    // Edge vanished?
                    self.state = AntState::Searching;
                }
            } else {
                // History empty, we are back at root
                self.state = AntState::Searching;
            }
        }
        None
    }
}
