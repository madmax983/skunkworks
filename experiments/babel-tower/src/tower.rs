use std::time::Duration;
use crate::phonology::EvolutionEngine;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct Tower {
    script: Vec<String>,
    transcript: Vec<(String, f32)>, // Word, Intensity
    cursor: usize,
    evolution_engine: EvolutionEngine,
    rng: StdRng,
}

impl Tower {
    pub fn new(script_text: &str) -> Self {
        let script: Vec<String> = script_text.split_whitespace().map(|s| s.to_string()).collect();
        Self {
            script,
            transcript: Vec::new(),
            cursor: 0,
            evolution_engine: EvolutionEngine::new(),
            rng: StdRng::from_entropy(),
        }
    }

    pub fn speak(&mut self, wait_time: Duration) -> String {
        // Calculate intensity based on wait time (e.g., 0.0 - 5.0)
        // Adjust scaling factor as needed. 100ms wait -> 1.0 intensity seems reasonable?
        // Threads might contend for much longer if many threads.
        let intensity = (wait_time.as_millis() as f64 / 50.0).clamp(0.0, 5.0);

        if self.script.is_empty() {
             return String::new();
        }

        if self.cursor >= self.script.len() {
             self.cursor = 0; // Loop script
        }

        let word = &self.script[self.cursor];
        let evolved_word = self.evolution_engine.evolve_word(word, &mut self.rng, intensity);

        self.transcript.push((evolved_word.clone(), intensity as f32));
        self.cursor += 1;

        evolved_word
    }

    pub fn get_transcript(&self) -> &Vec<(String, f32)> {
        &self.transcript
    }

    pub fn reset_transcript(&mut self) {
        self.transcript.clear();
        self.cursor = 0;
    }
}
