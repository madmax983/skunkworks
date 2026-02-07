#![cfg(feature = "nova")]

use crate::vm::Value;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

const MAX_CHANNEL_SIZE: usize = 100;
const MAX_FAITH: u64 = 1_000_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Egregore {
    pub faith: u64,
    pub channels: HashMap<String, VecDeque<Value>>,
    pub votes: HashMap<String, HashMap<String, usize>>,
    pub parameters: HashMap<String, Value>,
    pub believers: HashMap<usize, bool>, // strand_idx -> is_linked
}

impl Default for Egregore {
    fn default() -> Self {
        Self::new()
    }
}

impl Egregore {
    pub fn new() -> Self {
        Self {
            faith: 0,
            channels: HashMap::new(),
            votes: HashMap::new(),
            parameters: HashMap::new(),
            believers: HashMap::new(),
        }
    }

    pub fn tithe(&mut self, amount: i64) -> bool {
        if amount > 0 {
            self.faith = self.faith.saturating_add(amount as u64).min(MAX_FAITH);
            true
        } else {
            false
        }
    }

    pub fn link(&mut self, strand_idx: usize) {
        self.believers.insert(strand_idx, true);
    }

    pub fn push_channel(&mut self, channel: String, value: Value) {
        let queue = self.channels.entry(channel).or_default();
        if queue.len() >= MAX_CHANNEL_SIZE {
            queue.pop_front();
        }
        queue.push_back(value);
    }

    pub fn read_channel(&self, channel: &str) -> Option<Value> {
        self.channels.get(channel).and_then(|q| q.back()).cloned()
    }

    pub fn vote(&mut self, param: String, choice: Value) {
        let choice_str = format!("{}", choice);
        let param_votes = self.votes.entry(param).or_default();
        *param_votes.entry(choice_str).or_insert(0) += 1;
    }

    pub fn get_param(&self, param: &str) -> Option<Value> {
        self.parameters.get(param).cloned()
    }

    pub fn set_param(&mut self, param: String, value: Value) {
        self.parameters.insert(param, value);
    }

    pub fn resolve_vote(&mut self, param: &str) -> Option<String> {
        if let Some(votes) = self.votes.get(param) {
            votes
                .iter()
                .max_by_key(|entry| entry.1)
                .map(|(k, _)| k.clone())
        } else {
            None
        }
    }
}
