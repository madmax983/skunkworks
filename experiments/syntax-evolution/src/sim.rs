use crate::lang::{Meaning, Word, Case, WordOrder, Grammar};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Sentence {
    pub words: Vec<Word>,
}

impl Sentence {
    pub fn to_string(&self) -> String {
        self.words.iter().map(|w| w.raw.clone()).collect::<Vec<_>>().join(" ")
    }

    pub fn from_string(s: &str) -> Self {
        let words = s.split_whitespace()
            .map(|w| Word::from_raw(w))
            .collect();
        Self { words }
    }
}

pub struct Agent {
    pub grammar: Grammar,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            grammar: Grammar::default(),
        }
    }

    pub fn speak(&self, meaning: &Meaning) -> Sentence {
        let mut words = Vec::new();

        // Determine Case based on morphology strength
        let use_case = rand::thread_rng().gen::<f32>() < self.grammar.morphology_strength;

        let sub_case = if use_case { Case::Nominative } else { Case::None };
        let obj_case = if use_case { Case::Accusative } else { Case::None };

        let w_actor = Word::new(&meaning.actor, sub_case);
        let w_target = Word::new(&meaning.target, obj_case);
        let w_action = Word::new(&meaning.action, Case::None); // Verbs don't have case in this simplifiction

        // Determine Word Order
        // If grammar is Free, pick random. If SVO, stick to it.
        let order = match self.grammar.word_order {
            WordOrder::Free => {
                let r = rand::thread_rng().gen_range(0..3);
                match r {
                    0 => WordOrder::SVO,
                    1 => WordOrder::SOV,
                    _ => WordOrder::VSO,
                }
            },
            o => o,
        };

        match order {
            WordOrder::SVO => {
                words.push(w_actor);
                words.push(w_action);
                words.push(w_target);
            },
            WordOrder::SOV => {
                words.push(w_actor);
                words.push(w_target);
                words.push(w_action);
            },
            WordOrder::VSO => {
                words.push(w_action);
                words.push(w_actor);
                words.push(w_target);
            },
            WordOrder::Free => unreachable!(),
        }

        Sentence { words }
    }

    pub fn comprehend(&self, sentence: &Sentence) -> Result<Meaning, String> {
        // Try to identify roles by Case first
        let mut actor = None;
        let mut target = None;
        let mut action = None;

        for w in &sentence.words {
            match w.case {
                Case::Nominative => {
                    if actor.is_some() { return Err("Double Subject".to_string()); }
                    actor = Some(w.root.clone());
                },
                Case::Accusative => {
                    if target.is_some() { return Err("Double Object".to_string()); }
                    target = Some(w.root.clone());
                },
                _ => {
                    // Could be action or an un-cased noun
                    // For simplification, assume anything not ending in us/um is the action if we found others
                    // Or if we haven't found action yet.
                    if action.is_none() {
                        action = Some(w.root.clone());
                    } else if actor.is_none() {
                        actor = Some(w.root.clone());
                    } else if target.is_none() {
                        target = Some(w.root.clone());
                    }
                }
            }
        }

        // If we found everything via cases, great!
        if let (Some(a), Some(t), Some(act)) = (&actor, &target, &action) {
             return Ok(Meaning::new(a, act, t));
        }

        // Fallback: Word Order
        // If cases failed (e.g. all Case::None), use internal grammar preference to decode.
        // Assuming the speaker used OUR preferred word order (or the dominant one).
        if sentence.words.len() == 3 {
             let w1 = &sentence.words[0].root;
             let w2 = &sentence.words[1].root;
             let w3 = &sentence.words[2].root;

             return match self.grammar.word_order {
                 WordOrder::SVO | WordOrder::Free => Ok(Meaning::new(w1, w2, w3)), // Default Free to SVO for comprehension
                 WordOrder::SOV => Ok(Meaning::new(w1, w3, w2)),
                 WordOrder::VSO => Ok(Meaning::new(w2, w1, w3)),
             };
        }

        Err("Incomplete Sentence".to_string())
    }

    pub fn learn(&mut self, success: bool) {
        if !success {
            // Punish current strategy
            // If we are Free order and failed, maybe become SVO?
            if self.grammar.word_order == WordOrder::Free {
                 self.grammar.word_order = WordOrder::SVO;
            }
        }
    }
}

pub fn erode(sentence: &Sentence) -> Sentence {
    let mut new_words = Vec::new();
    let mut rng = rand::thread_rng();

    for w in &sentence.words {
        let mut raw = w.raw.clone();

        // 20% chance to lose the last character
        if raw.len() > 1 && rng.gen::<f32>() < 0.2 {
            raw.pop();
        }

        // Re-parse the word from the eroded string
        // This effectively updates the Case (e.g. 'puerus' -> 'pueru' (None))
        new_words.push(Word::from_raw(&raw));
    }

    Sentence { words: new_words }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speak_and_comprehend() {
        let agent = Agent::new();
        let meaning = Meaning::new("puer", "amat", "puellam");

        // Test Speaking
        let s = agent.speak(&meaning);
        println!("Spoken: {:?}", s.to_string());

        // Test Comprehension (Perfect channel)
        let parsed = agent.comprehend(&s).expect("Should comprehend self");
        assert_eq!(parsed.actor, "puer");
        assert_eq!(parsed.target, "puellam");
        // Note: target in Meaning is "puellam", but speak() adds suffix?
        // Wait, Meaning stores the concept root or the full string?
        // My implementation of Meaning::new takes string slices.
        // Word::new takes root and adds suffix.
        // So Meaning should probably store ROOTS ("puer", "puella").
    }

    #[test]
    fn test_erosion() {
        let w = Word::new("puer", Case::Nominative); // puerus
        let s = Sentence { words: vec![w] };

        // Force erosion for test? erode uses rng.
        // Let's just run it 100 times and ensure some change.
        let mut changed = false;
        for _ in 0..100 {
            let e = erode(&s);
            if e.to_string() != "puerus" {
                changed = true;
                break;
            }
        }
        assert!(changed);
    }
}
