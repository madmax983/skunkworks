use crate::phonology::{Phonotactics, Word};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Concept {
    Food,
    Water,
    Danger,
    Shelter,
    Friend,
}

impl Concept {
    pub fn all() -> Vec<Concept> {
        vec![
            Concept::Food,
            Concept::Water,
            Concept::Danger,
            Concept::Shelter,
            Concept::Friend,
        ]
    }

    pub fn random() -> Self {
        let all = Self::all();
        let mut rng = rand::thread_rng();
        all[rng.gen_range(0..all.len())]
    }
}

#[derive(Debug, Clone)]
pub struct Lexicon {
    pub map: HashMap<Concept, Word>,
    pub weights: HashMap<Concept, f32>, // How confident are we?
}

impl Lexicon {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            weights: HashMap::new(),
        }
    }

    pub fn add(&mut self, concept: Concept, word: Word, weight: f32) {
        let current_weight = self.weights.get(&concept).copied().unwrap_or(0.0);
        // If the new word is more weighted, or we don't have one, take it.
        // Or if it's the SAME word, reinforce it.
        if let Some(existing) = self.map.get(&concept) {
            if *existing == word {
                self.weights.insert(concept, current_weight + weight);
                return;
            }
        }

        if weight > current_weight {
            self.map.insert(concept, word);
            self.weights.insert(concept, weight);
        }
    }

    pub fn get(&self, concept: &Concept) -> Option<&Word> {
        self.map.get(concept)
    }
}

pub struct Agent {
    pub lexicon: Lexicon,
    pub phonotactics: Phonotactics,
    // Visuals
    pub position: (f32, f32),
    pub color: (u8, u8, u8), // rgb
}

impl Agent {
    pub fn new(phonotactics: Phonotactics, pos: (f32, f32), color: (u8, u8, u8)) -> Self {
        Self {
            lexicon: Lexicon::new(),
            phonotactics,
            position: pos,
            color,
        }
    }

    pub fn generate_native_lexicon(&mut self) {
        for concept in Concept::all() {
            let word = self.phonotactics.generate();
            self.lexicon.add(concept, word, 1.0);
        }
    }

    pub fn speak(&mut self, concept: Concept) -> Word {
        if let Some(word) = self.lexicon.get(&concept) {
            // Chance to mutate (Drift)
            // For now, minimal drift
            word.clone()
        } else {
            let word = self.phonotactics.generate();
            self.lexicon.add(concept, word.clone(), 1.0);
            word
        }
    }

    // Returns true if understood (trade successful)
    pub fn listen(&mut self, concept: Concept, word: &Word) -> bool {
        if let Some(known) = self.lexicon.get(&concept) {
            if known == word {
                // Reinforce
                let w = self.lexicon.weights.get_mut(&concept).unwrap();
                *w += 0.5;
                return true;
            }
        }

        // Did not understand.
        // If we don't know the concept at all, learn it.
        if !self.lexicon.map.contains_key(&concept) {
            self.lexicon.add(concept, word.clone(), 1.0);
        } else {
            // Chance to adopt loanword if current word is weak
            let mut rng = rand::thread_rng();
            let current_weight = self.lexicon.weights.get(&concept).copied().unwrap_or(0.0);
            // If current weight is low, we might switch
            if current_weight < 5.0 && rng.gen_bool(0.3) {
                self.lexicon
                    .add(concept, word.clone(), current_weight + 0.5);
            }
        }

        false
    }
}
