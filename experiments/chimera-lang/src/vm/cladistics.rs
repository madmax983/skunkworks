#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::HashMap;

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CladeNode {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub birth_tick: u64,
    pub death_tick: Option<u64>,
    pub event: String,
    pub mutation_count: usize,
    pub children: Vec<usize>,
    pub strand_idx: usize, // The slot it occupied
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cladistics {
    pub nodes: HashMap<usize, CladeNode>,
    pub active_map: HashMap<usize, usize>, // strand_idx -> node_id
    pub next_node_id: usize,
}

#[cfg(feature = "nova")]
impl Default for Cladistics {
    fn default() -> Self {
        Self::new()
    }
}

impl Cladistics {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            active_map: HashMap::new(),
            next_node_id: 0,
        }
    }

    pub fn register_strand(
        &mut self,
        strand_idx: usize,
        parent_strand_idx: Option<usize>,
        tick: u64,
        event: String,
    ) {
        let id = self.next_node_id;
        self.next_node_id += 1;

        let parent_node_id = if let Some(pidx) = parent_strand_idx {
            self.active_map.get(&pidx).cloned()
        } else {
            None
        };

        let node = CladeNode {
            id,
            parent_id: parent_node_id,
            birth_tick: tick,
            death_tick: None,
            event,
            mutation_count: 0,
            children: Vec::new(),
            strand_idx,
        };

        if let Some(pid) = parent_node_id {
            if let Some(parent) = self.nodes.get_mut(&pid) {
                parent.children.push(id);
            }
        }

        self.nodes.insert(id, node);
        self.active_map.insert(strand_idx, id);
    }

    pub fn kill_strand(&mut self, strand_idx: usize, tick: u64) {
        if let Some(&node_id) = self.active_map.get(&strand_idx) {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.death_tick = Some(tick);
            }
        }
    }

    pub fn mutate_strand(&mut self, strand_idx: usize) {
        if let Some(&node_id) = self.active_map.get(&strand_idx) {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.mutation_count += 1;
            }
        }
    }

    // For rendering, we might want to get roots (nodes with no parents)
    pub fn get_roots(&self) -> Vec<usize> {
        self.nodes
            .values()
            .filter(|n| n.parent_id.is_none())
            .map(|n| n.id)
            .collect()
    }
}
