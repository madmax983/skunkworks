use crate::audio::SoundEvent;
use crossbeam::channel::Sender;
#[cfg(loom)]
use loom::thread;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use rand::prelude::*;
#[cfg(not(loom))]
use std::thread;
use std::time::Duration;

use crate::graph::{Arc, DepGraph};

pub struct Ant {
    id: usize,
    graph: Arc<DepGraph>,
    audio_tx: Sender<SoundEvent>,
    current_node: NodeIndex,
}

impl Ant {
    pub fn new(
        id: usize,
        graph: Arc<DepGraph>,
        audio_tx: Sender<SoundEvent>,
        start_node: NodeIndex,
    ) -> Self {
        Self {
            id,
            graph,
            audio_tx,
            current_node: start_node,
        }
    }

    pub fn spawn(mut self) {
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                self.step(&mut rng);
            }
        });
    }

    fn step(&mut self, rng: &mut impl Rng) {
        // 1. Work on current node
        self.work_on_node(rng);

        // 2. Choose next node
        let neighbors: Vec<NodeIndex> = self
            .graph
            .neighbors_directed(self.current_node, Direction::Outgoing)
            .collect();

        if neighbors.is_empty() {
            // Respawn at a root node (layer 0)
            let roots: Vec<NodeIndex> = self
                .graph
                .node_indices()
                .filter(|&i| self.graph[i].layer == 0)
                .collect();

            if !roots.is_empty() {
                self.current_node = roots[rng.gen_range(0..roots.len())];
            }
            // Small delay before restart
            #[cfg(not(loom))]
            thread::sleep(Duration::from_millis(500));
            return;
        }

        // Weighted random choice based on pheromones
        let mut weights = Vec::new();
        let mut candidates = Vec::new();

        for &next_node in &neighbors {
            if let Some(edge_idx) = self.graph.find_edge(self.current_node, next_node) {
                if let Ok(pheromone) = self.graph[edge_idx].pheromone.lock() {
                    weights.push(*pheromone as f64 + 1.0);
                    candidates.push(next_node);
                }
            }
        }

        if !weights.is_empty() {
            if let Ok(dist) = rand::distributions::WeightedIndex::new(&weights) {
                self.current_node = candidates[dist.sample(rng)];
            } else {
                self.current_node = candidates[rng.gen_range(0..candidates.len())];
            }
        } else {
            // Fallback
            self.current_node = neighbors[rng.gen_range(0..neighbors.len())];
        }

        // Travel time
        #[cfg(not(loom))]
        thread::sleep(Duration::from_millis(rng.gen_range(50..150)));
    }

    fn work_on_node(&self, _rng: &mut impl Rng) {
        let node = &self.graph[self.current_node];
        let steps = 10;
        let step_duration = node.build_cost / steps as u32;

        // 1. Acquire Logical Lock
        // Loop until we can set builder_id to self.id
        loop {
            // Use try_lock to avoid blocking if TUI is reading
            if let Ok(mut state) = node.state.try_lock() {
                if state.builder_id.is_none() {
                    // Success: We claim the node
                    state.builder_id = Some(self.id);
                    state.progress = 0.0;
                    self.audio_tx.send(SoundEvent::BuildStart(node.id)).ok();
                    break;
                } else {
                    // Logical Contention: Someone else is building
                    // Emit contention sound occasionally
                    if rand::random::<f32>() < 0.2 {
                        self.audio_tx.send(SoundEvent::Contention).ok();
                    }
                }
            } else {
                // Mutex Contention: TUI or another ant is holding the mutex momentarily
                // Just retry
            }
            #[cfg(not(loom))]
            thread::sleep(Duration::from_millis(50));
            #[cfg(loom)]
            loom::thread::yield_now();
        }

        // 2. Work Loop
        for i in 1..=steps {
            #[cfg(not(loom))]
            thread::sleep(Duration::from_millis(step_duration as u64));
            #[cfg(loom)]
            loom::thread::yield_now();

            // Update progress
            // We use lock() here because we own the logical lock, so we expect to get mutex quickly
            if let Ok(mut state) = node.state.lock() {
                // Verify we still own it (sanity check)
                if state.builder_id == Some(self.id) {
                    state.progress = i as f32 / steps as f32;
                } else {
                    // Lost ownership? Should not happen in this model.
                    return;
                }
            }
        }

        // 3. Release Logical Lock
        if let Ok(mut state) = node.state.lock() {
            if state.builder_id == Some(self.id) {
                state.builder_id = None;
                state.progress = 0.0;
            }
        }
        self.audio_tx.send(SoundEvent::BuildComplete).ok();
    }
}
