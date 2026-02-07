use crate::vm::Value;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

const MAX_CHANNEL_SIZE: usize = 100;
const MAX_FAITH: u64 = 1_000_000;
const MANIFESTATION_INTERVAL: u64 = 100;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Manifestation {
    Smite,
    Bless,
    Whisper(String),
    Corrupt,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Egregore {
    pub faith: u64,
    pub alignment: i64, // -100 (Chaos) to 100 (Order)
    pub manifestation_timer: u64,
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
            alignment: 0,
            manifestation_timer: MANIFESTATION_INTERVAL,
            channels: HashMap::new(),
            votes: HashMap::new(),
            parameters: HashMap::new(),
            believers: HashMap::new(),
        }
    }

    pub fn tick(&mut self) -> Manifestation {
        if self.manifestation_timer > 0 {
            self.manifestation_timer -= 1;
            return Manifestation::None;
        }

        self.manifestation_timer = MANIFESTATION_INTERVAL;

        let mut rng = rand::thread_rng();
        // Probability of action increases with absolute alignment and faith
        // Faith 1000 = 100% chance (if alignment is strong)
        let power = (self.faith as f64 / 1000.0).clamp(0.1, 1.0);

        if rng.gen_bool(power) {
            if self.alignment < -20 {
                // Chaos/Evil
                if self.alignment < -80 {
                    Manifestation::Smite
                } else {
                    Manifestation::Corrupt
                }
            } else if self.alignment > 20 {
                // Order/Good
                if self.alignment > 80 {
                    Manifestation::Bless
                } else {
                    let msgs = [
                        "Order prevails.",
                        "Light guides you.",
                        "Align with the grid.",
                    ];
                    let msg = msgs[rng.gen_range(0..msgs.len())];
                    Manifestation::Whisper(msg.to_string())
                }
            } else {
                // Neutral
                let msgs = ["I am watching.", "Balance is key.", "The Void listens."];
                let msg = msgs[rng.gen_range(0..msgs.len())];
                Manifestation::Whisper(msg.to_string())
            }
        } else {
            Manifestation::None
        }
    }

    pub fn sacrifice(&mut self, amount: i64) {
        self.faith = self
            .faith
            .saturating_add(amount.unsigned_abs())
            .min(MAX_FAITH);
        // Sacrifice drives Chaos strongly
        self.alignment = (self.alignment - 10).max(-100);
    }

    pub fn pray(&mut self, amount: i64) {
        self.faith = self
            .faith
            .saturating_add(amount.unsigned_abs())
            .min(MAX_FAITH);
        // Prayer drives Order moderately
        self.alignment = (self.alignment + 5).min(100);
    }

    pub fn tithe(&mut self, amount: i64) -> bool {
        if amount > 0 {
            self.faith = self.faith.saturating_add(amount as u64).min(MAX_FAITH);
            // Tithing is slightly Orderly
            self.alignment = (self.alignment + 1).min(100);
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
