#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
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
            .saturating_add(amount.abs() as u64)
            .min(MAX_FAITH);
        // Sacrifice drives Chaos strongly
        self.alignment = (self.alignment - 10).max(-100);
    }

    pub fn pray(&mut self, amount: i64) {
        self.faith = self
            .faith
            .saturating_add(amount.abs() as u64)
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

// --- VM Execution Logic ---

pub fn exec_egregore_link(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(name) = val {
            vm.egregore.link(vm.ip.0);
            vm.output.push(format!("EGREGORE: Linked to {}", name));
        } else {
            vm.output
                .push("Error: Type mismatch for egregore_link".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for egregore_link".to_string());
    }
    None
}

pub fn exec_pray(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(amount) = val {
            if amount > 0 && vm.energy >= amount {
                vm.energy -= amount;
                vm.egregore.pray(amount);
                vm.output.push(format!("PRAY: Donated {} energy", amount));
            } else {
                vm.output.push("PRAY: Insufficient energy".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for pray".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for pray".to_string());
    }
    None
}

pub fn exec_egregore_tithe(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(amount) = val {
            if amount > 0 && vm.energy >= amount {
                vm.energy -= amount;
                vm.egregore.tithe(amount);
                vm.output.push(format!("EGREGORE: Tithed {}", amount));
            } else {
                vm.output
                    .push("EGREGORE: Insufficient energy to tithe".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for egregore_tithe".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for egregore_tithe".to_string());
    }
    None
}

pub fn exec_egregore_channel(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let val = vm.stack.pop().unwrap();
        let name_val = vm.stack.pop().unwrap();
        if let Value::Str(name) = name_val {
            vm.egregore.push_channel(name.clone(), val.clone());
            vm.output
                .push(format!("EGREGORE: Sent to channel {}", name));
        } else {
            vm.output
                .push("Error: Type mismatch for egregore_channel".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for egregore_channel".to_string());
    }
    None
}

pub fn exec_egregore_dictate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let val = vm.stack.pop().unwrap();
        let name_val = vm.stack.pop().unwrap();
        if let Value::Str(name) = name_val {
            vm.egregore.vote(name.clone(), val.clone());
            vm.output.push(format!("EGREGORE: Voted on {}", name));
        } else {
            vm.output
                .push("Error: Type mismatch for egregore_dictate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for egregore_dictate".to_string());
    }
    None
}

pub fn exec_egregore_query(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(key) = val {
            if let Some(v) = vm.egregore.get_param(&key) {
                vm.stack.push(v);
            } else if let Some(v) = vm.egregore.read_channel(&key) {
                vm.stack.push(v);
            } else {
                vm.stack.push(Value::Int(0)); // Not found
            }
        } else {
            vm.output
                .push("Error: Type mismatch for egregore_query".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for egregore_query".to_string());
    }
    None
}

pub fn exec_egregore_summon(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(ritual) = val {
            let cost = 100;
            if vm.egregore.faith >= cost {
                vm.egregore.faith -= cost;
                match ritual.as_str() {
                    "Rain" => {
                        for row in vm.moisture_grid.iter_mut() {
                            for cell in row.iter_mut() {
                                *cell = cell.saturating_add(50);
                            }
                        }
                        vm.output.push("EGREGORE: Summoned RAIN".to_string());
                    }
                    "Dawn" => {
                        for row in vm.light_grid.iter_mut() {
                            for cell in row.iter_mut() {
                                *cell = 100;
                            }
                        }
                        vm.output.push("EGREGORE: Summoned DAWN".to_string());
                    }
                    "Apocalypse" => {
                        vm.organelles.retain(|_| rand::random::<bool>());
                        vm.output.push("EGREGORE: Summoned APOCALYPSE".to_string());
                    }
                    _ => {
                        vm.egregore.faith += cost; // Refund
                        vm.output
                            .push(format!("EGREGORE: Unknown ritual {}", ritual));
                    }
                }
            } else {
                vm.output.push("EGREGORE: Insufficient faith".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for egregore_summon".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for egregore_summon".to_string());
    }
    None
}

pub fn exec_sacrifice(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let s_idx = vm.ip.0;
    if s_idx < vm.dna.helix.strands.len() {
        vm.egregore.sacrifice(100);

        vm.dna.helix.strands[s_idx].genes.clear();
        vm.epigenome.retain(|(s, _)| *s != s_idx);
        vm.cladistics.kill_strand(s_idx, vm.tick_counter);

        vm.output
            .push(format!("SACRIFICE: Strand {} given to the Void", s_idx));
        vm.halted = true;
    }
    None
}
