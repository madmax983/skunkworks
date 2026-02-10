use crate::forest::{Forest, TreeID};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait Scheduler {
    fn schedule(&mut self, forest: &Forest) -> Option<TreeID>;
    fn name(&self) -> String;
}

pub struct RoundRobin {
    current_index: usize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self { current_index: 0 }
    }
}

impl Scheduler for RoundRobin {
    fn schedule(&mut self, forest: &Forest) -> Option<TreeID> {
        if forest.trees.is_empty() {
            return None;
        }

        let id = forest.trees[self.current_index % forest.trees.len()].id;
        self.current_index += 1;
        Some(id)
    }

    fn name(&self) -> String {
        "Round Robin".to_string()
    }
}

pub struct RandomScheduler;

impl Scheduler for RandomScheduler {
    fn schedule(&mut self, forest: &Forest) -> Option<TreeID> {
        if forest.trees.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        forest.trees.choose(&mut rng).map(|t| t.id)
    }

    fn name(&self) -> String {
        "Random".to_string()
    }
}

// Priority Scheduler: Picks the tree with the LEAST growth (most "starved")
pub struct PriorityScheduler;

impl Scheduler for PriorityScheduler {
    fn schedule(&mut self, forest: &Forest) -> Option<TreeID> {
        if forest.trees.is_empty() {
            return None;
        }
        // Find tree with lowest growth_cursor / full_dna.len() ratio
        forest.trees.iter()
            .min_by(|a, b| {
                let ratio_a = a.growth_cursor as f32 / a.full_dna.len() as f32;
                let ratio_b = b.growth_cursor as f32 / b.full_dna.len() as f32;
                ratio_a.partial_cmp(&ratio_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|t| t.id)
    }

    fn name(&self) -> String {
        "Fairness (Priority)".to_string()
    }
}
