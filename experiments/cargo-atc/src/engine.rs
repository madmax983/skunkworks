use crate::graph::{DepGraph, Node};
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Runway {
    #[allow(dead_code)]
    pub id: usize,
    pub assigned_crate: Option<NodeIndex>,
    pub progress: u64,
    pub total_cost: u64,
}

impl Runway {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            assigned_crate: None,
            progress: 0,
            total_cost: 0,
        }
    }
}

pub struct GameEngine {
    pub graph: DepGraph,
    pub completed: HashSet<NodeIndex>,
    pub runways: Vec<Runway>,
    pub ready_queue: Vec<NodeIndex>,
    pub score: u64,
    pub ticks: u64,
    pub game_over: bool,
}

impl GameEngine {
    pub fn new(graph: DepGraph, runway_count: usize) -> Self {
        let mut engine = Self {
            graph,
            completed: HashSet::new(),
            runways: (0..runway_count).map(Runway::new).collect(),
            ready_queue: Vec::new(),
            score: 0,
            ticks: 0,
            game_over: false,
        };
        engine.update_ready_queue();
        engine
    }

    pub fn update_ready_queue(&mut self) {
        // Find crates that are ready AND not already in queue or on runway or completed
        let candidates = self.graph.get_ready_crates(&self.completed);

        for idx in candidates {
            // Check if already in queue
            if self.ready_queue.contains(&idx) {
                continue;
            }
            // Check if on runway
            if self.runways.iter().any(|r| r.assigned_crate == Some(idx)) {
                continue;
            }
            // Add to queue
            self.ready_queue.push(idx);
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        self.score += 1; // Time is score (lower is better? or survive longer?)
                         // Let's make score = completed crates.

        let mut finished_crates = Vec::new();

        // Update runways
        for runway in &mut self.runways {
            if let Some(crate_idx) = runway.assigned_crate {
                runway.progress += 1;
                if runway.progress >= runway.total_cost {
                    // Completed!
                    finished_crates.push(crate_idx);
                    runway.assigned_crate = None;
                    runway.progress = 0;
                    runway.total_cost = 0;
                }
            }
        }

        // Handle completions
        for idx in finished_crates {
            self.completed.insert(idx);
        }

        if !self.completed.is_empty() {
            self.update_ready_queue();
        }

        // Physics update
        self.graph.update_layout(0.1);

        // Check win condition
        // If all nodes are completed
        if self.completed.len() == self.graph.graph.node_count() {
            self.game_over = true;
        }
    }

    pub fn assign_crate(&mut self, queue_index: usize, runway_id: usize) -> bool {
        if runway_id >= self.runways.len() {
            return false;
        }
        if queue_index >= self.ready_queue.len() {
            return false;
        }

        if self.runways[runway_id].assigned_crate.is_some() {
            return false; // Runway occupied
        }

        let crate_idx = self.ready_queue.remove(queue_index);
        let cost = self.graph.graph[crate_idx].build_cost;

        let runway = &mut self.runways[runway_id];
        runway.assigned_crate = Some(crate_idx);
        runway.total_cost = cost;
        runway.progress = 0;

        true
    }

    pub fn get_node(&self, idx: NodeIndex) -> &Node {
        &self.graph.graph[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Mocking DepGraph is hard because it needs cargo_metadata.
    // We'll trust the integration tests or simple logic tests if we abstract graph?
    // For now, minimal tests.

    #[test]
    fn test_runway_logic() {
        let r = Runway::new(0);
        assert!(r.assigned_crate.is_none());
    }
}
