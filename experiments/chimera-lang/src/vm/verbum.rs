use crate::ast::Gene;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verbum {
    pub id: usize,
    pub name: String,
    pub genes: Vec<Gene>,
    pub cost: i64,
    pub rarity: Rarity,
    pub etymology: Vec<String>, // Parent words
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbumForge {
    pub words: HashMap<String, Verbum>,
    pub next_id: usize,
}

impl Default for VerbumForge {
    fn default() -> Self {
        Self::new()
    }
}

impl VerbumForge {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn forge(
        &mut self,
        name: String,
        genes: Vec<Gene>,
        parents: Vec<String>,
    ) -> Result<usize, String> {
        if self.words.contains_key(&name) {
            return Err(format!("Word '{}' already exists", name));
        }

        let mut cost = genes.len() as i64 * 5; // Base cost
        if cost == 0 {
            cost = 1;
        }

        let rarity = match genes.len() {
            0..=5 => Rarity::Common,
            6..=10 => Rarity::Uncommon,
            11..=20 => Rarity::Rare,
            21..=50 => Rarity::Epic,
            51..=100 => Rarity::Legendary,
            _ => Rarity::Mythic,
        };

        let id = self.next_id;
        self.next_id += 1;

        let word = Verbum {
            id,
            name: name.clone(),
            genes: genes.clone(), // Clone here
            cost,
            rarity,
            etymology: parents,
        };

        self.words.insert(name, word);
        Ok(id)
    }

    pub fn get(&self, name: &str) -> Option<&Verbum> {
        self.words.get(name)
    }

    pub fn get_word_data(&self, name: &str) -> Option<(Vec<Gene>, i64)> {
        self.words.get(name).map(|w| (w.genes.clone(), w.cost))
    }
}
