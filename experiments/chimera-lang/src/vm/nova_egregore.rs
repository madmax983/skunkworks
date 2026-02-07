#[cfg(feature = "nova")]
use crate::vm::Value;
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Egregore {
    pub id: usize,
    pub name: String,
    pub energy: i64,
    pub members: HashSet<usize>,
    pub memory: HashMap<String, Value>,
}

#[cfg(feature = "nova")]
impl Egregore {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            energy: 0,
            members: HashSet::new(),
            memory: HashMap::new(),
        }
    }

    pub fn join(&mut self, strand_idx: usize) -> bool {
        self.members.insert(strand_idx)
    }

    pub fn leave(&mut self, strand_idx: usize) -> bool {
        self.members.remove(&strand_idx)
    }

    pub fn is_member(&self, strand_idx: usize) -> bool {
        self.members.contains(&strand_idx)
    }

    pub fn tithe(&mut self, amount: i64) {
        if amount > 0 {
            self.energy = self.energy.saturating_add(amount);
        }
    }

    pub fn channel(&mut self, amount: i64) -> bool {
        if amount > 0 && self.energy >= amount {
            self.energy -= amount;
            true
        } else {
            false
        }
    }

    pub fn set_memory(&mut self, key: String, value: Value) {
        self.memory.insert(key, value);
    }

    pub fn get_memory(&self, key: &str) -> Option<Value> {
        self.memory.get(key).cloned()
    }
}
