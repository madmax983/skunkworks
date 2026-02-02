use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    Food,
    Water,
    Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Meaning {
    Hello,
    Trade(Resource),
    Accept,
    Reject,
    Bye,
}

impl Meaning {
    pub fn iter() -> impl Iterator<Item = Meaning> {
        use Meaning::*;
        use Resource::*;
        [
            Hello,
            Trade(Food),
            Trade(Water),
            Trade(Data),
            Accept,
            Reject,
            Bye,
        ].into_iter()
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let choices = [
            Meaning::Hello,
            Meaning::Trade(Resource::Food),
            Meaning::Trade(Resource::Water),
            Meaning::Trade(Resource::Data),
            Meaning::Accept,
            Meaning::Reject,
            Meaning::Bye,
        ];
        *choices.choose(rng).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(into = "Vec<(Meaning, String)>", from = "Vec<(Meaning, String)>")]
pub struct Lexicon {
    pub map: HashMap<Meaning, String>,
}

impl From<Lexicon> for Vec<(Meaning, String)> {
    fn from(val: Lexicon) -> Self {
        val.map.into_iter().collect()
    }
}

impl From<Vec<(Meaning, String)>> for Lexicon {
    fn from(val: Vec<(Meaning, String)>) -> Self {
        Lexicon { map: val.into_iter().collect() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub lexicon: Lexicon,
    pub score: f64,
    pub vocabulary: HashMap<String, Meaning>,
}

impl Agent {
    pub fn new_random(id: usize, x: f64, y: f64) -> Self {
        let mut lexicon = Lexicon::default();
        let mut vocabulary = HashMap::new();
        let mut rng = thread_rng();

        for meaning in Meaning::iter() {
            let word = Self::generate_word(&mut rng);
            lexicon.map.insert(meaning, word.clone());
            vocabulary.insert(word, meaning);
        }

        Self {
            id, x, y, lexicon, vocabulary, score: 0.0
        }
    }

    fn generate_word(rng: &mut impl Rng) -> String {
        let len = rng.gen_range(3..=5);
        (0..len).map(|_| (b'a' + rng.gen_range(0..26)) as char).collect()
    }

    pub fn speak(&self, meaning: Meaning) -> String {
        let mut word = self.lexicon.map.get(&meaning).cloned().unwrap_or_else(|| "???".to_string());

        let mut rng = thread_rng();
        if rng.gen_bool(0.01) {
             word = Self::mutate(&word, &mut rng);
        }
        word
    }

    fn mutate(word: &str, rng: &mut impl Rng) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        if chars.is_empty() { return "a".to_string(); }

        match rng.gen_range(0..3) {
            0 => { // Change
                let idx = rng.gen_range(0..chars.len());
                chars[idx] = (b'a' + rng.gen_range(0..26)) as char;
            },
            1 => { // Add
                let idx = rng.gen_range(0..=chars.len());
                let c = (b'a' + rng.gen_range(0..26)) as char;
                chars.insert(idx, c);
            },
            2 => { // Remove
                if chars.len() > 1 {
                    let idx = rng.gen_range(0..chars.len());
                    chars.remove(idx);
                }
            }
            _ => {}
        }
        chars.into_iter().collect()
    }

    pub fn listen(&self, word: &str) -> Option<Meaning> {
        self.vocabulary.get(word).copied()
    }

    pub fn learn(&mut self, word: String, meaning: Meaning) {
        self.lexicon.map.insert(meaning, word.clone());
        self.vocabulary.insert(word, meaning);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Network {
    pub agents: Vec<Agent>,
    pub active_links: Vec<(usize, usize, bool)>, // (u, v, success)
    pub epoch: usize,
}

impl Network {
    pub fn new(count: usize) -> Self {
        let mut rng = thread_rng();
        let agents = (0..count).map(|i| {
            Agent::new_random(i, rng.gen::<f64>() * 200.0 - 100.0, rng.gen::<f64>() * 200.0 - 100.0)
        }).collect();
        Self { agents, active_links: vec![], epoch: 0 }
    }

    pub fn step(&mut self) {
        self.epoch += 1;
        self.active_links.clear();
        let mut rng = thread_rng();
        let count = self.agents.len();
        if count < 2 { return; }

        let interactions = count / 2;

        // We need to access agents mutably, but randomly.
        // Easiest is to pick indices and assume no overlap or handle it carefully.
        // For simulation, sequential interaction is fine.

        for _ in 0..interactions {
            let idx_a = rng.gen_range(0..count);
            let mut idx_b = rng.gen_range(0..count);
            while idx_a == idx_b { idx_b = rng.gen_range(0..count); }

            // A speaks
            let meaning = Meaning::random(&mut rng);

            // To satisfy borrow checker, we extract what we need
            let word = self.agents[idx_a].speak(meaning);

            // B listens
            // In this simulation, we assume B can see the "Ground Truth" AFTER the attempt
            // to decide whether to learn.
            // Or: A says "Word". B interprets as M'.
            // If M' == Meaning, Success.
            // If M' != Meaning, Failure.
            // If Failure, B *might* learn "Word" -> Meaning (Teacher Forcing).

            let b_guess = self.agents[idx_b].listen(&word);
            let success = b_guess == Some(meaning);

            if success {
                self.agents[idx_a].score += 1.0;
                self.agents[idx_b].score += 1.0;
            } else {
                // Learning step: B adopts A's word for this meaning with some probability
                // "Assimilation"
                if rng.gen_bool(0.3) {
                    self.agents[idx_b].learn(word, meaning);
                }
            }

            self.active_links.push((idx_a, idx_b, success));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence() {
        let mut net = Network::new(2);
        // Force them to differ initially? random is likely diff.

        let m = Meaning::Hello;
        let _w1 = net.agents[0].speak(m);
        let _w2 = net.agents[1].speak(m);

        // They shouldn't understand each other perfectly at start (mostly)
        // Run simulation
        for _ in 0..1000 {
            net.step();
        }

        // Now check if they agree on words
        let w1_final = net.agents[0].lexicon.map.get(&m).unwrap();
        let w2_final = net.agents[1].lexicon.map.get(&m).unwrap();

        // Check if Agent 1 understands Agent 0
        let guess = net.agents[1].listen(w1_final);
        assert_eq!(guess, Some(m), "Agent 1 should understand Agent 0 after training");

        // They might not speak the SAME word (synonyms), but they should understand.
        // But with our logic: learn() sets the word in Lexicon too. So they should converge to same word.
         assert_eq!(w1_final, w2_final, "Agents should converge to the same word for Hello");
    }
}
