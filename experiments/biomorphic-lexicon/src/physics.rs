use crate::phonology::{GrimmsLaw, Phoneme, Rule, VowelShift, Word};
use glam::Vec3;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct LexicalNode {
    pub pos: Vec3,
    pub vel: Vec3,
    pub force: Vec3,
    pub fixed: bool,
    pub mass: f32,
    pub phoneme: Phoneme,
    pub mutated: bool, // For visual feedback
}

impl LexicalNode {
    pub fn new(pos: Vec3, fixed: bool, phoneme: Phoneme) -> Self {
        // Mass depends on phoneme type: Vowels light, Consonants heavy
        let mass = if phoneme.is_vowel() { 0.5 } else { 1.5 };
        Self {
            pos,
            vel: Vec3::ZERO,
            force: Vec3::ZERO,
            fixed,
            mass,
            phoneme,
            mutated: false,
        }
    }
}

pub struct LexicalString {
    pub nodes: Vec<LexicalNode>,
    pub word: Word, // Source of truth for phonemes
    pub rest_length: f32,
    pub tension: f32,
    pub damping: f32,
    pub mutation_threshold: f32,
}

impl LexicalString {
    pub fn new(text: &str, start: Vec3, end: Vec3, tension: f32, damping: f32) -> Self {
        let word = Word::new(text);
        let len = word.phonemes.len();
        let mut nodes = Vec::with_capacity(len);

        if len > 1 {
            let step = (end - start) / (len - 1) as f32;
            let rest_length = step.length();

            for (i, p) in word.phonemes.iter().enumerate() {
                let pos = start + step * i as f32;
                // Fix the ends
                let fixed = i == 0 || i == len - 1;
                nodes.push(LexicalNode::new(pos, fixed, p.clone()));
            }

            Self {
                nodes,
                word,
                rest_length,
                tension,
                damping,
                mutation_threshold: 50.0, // Energy threshold
            }
        } else {
            // Handle single char word?
            let nodes = vec![LexicalNode::new(start, true, word.phonemes[0].clone())];
            Self {
                nodes,
                word,
                rest_length: 1.0,
                tension,
                damping,
                mutation_threshold: 50.0,
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_physics(dt);
        self.check_mutation();
    }

    fn update_physics(&mut self, dt: f32) {
        let len = self.nodes.len();
        if len < 2 {
            return;
        }

        let mut forces = vec![Vec3::ZERO; len];

        // Spring forces
        for i in 0..len - 1 {
            let p1 = self.nodes[i].pos;
            let p2 = self.nodes[i + 1].pos;

            let delta = p2 - p1;
            let dist = delta.length();

            if dist > 1e-6 {
                let dir = delta / dist;
                let stretch = dist - self.rest_length;
                let f = dir * (self.tension * stretch);

                forces[i] += f;
                forces[i + 1] -= f;
            }
        }

        // Integration
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.fixed {
                node.vel = Vec3::ZERO;
                continue;
            }

            node.force = forces[i];
            node.force -= node.vel * self.damping; // Damping

            let acc = node.force / node.mass;
            node.vel += acc * dt;
            node.pos += node.vel * dt;
        }
    }

    fn check_mutation(&mut self) {
        // Calculate total kinetic energy
        let total_ke: f32 = self
            .nodes
            .iter()
            .map(|n| 0.5 * n.mass * n.vel.length_squared())
            .sum();

        // If energy is high, trigger mutation
        if total_ke > self.mutation_threshold {
            let mut rng = rand::thread_rng();
            // Chance to mutate per frame if threshold exceeded
            if rng.gen_bool(0.05) {
                self.mutate();
            }
        }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let rules: Vec<Box<dyn Rule>> = vec![Box::new(GrimmsLaw), Box::new(VowelShift)];

        let rule_idx = rng.gen_range(0..rules.len());
        let changed = rules[rule_idx].apply(&mut self.word, &mut rng);

        if changed {
            // Sync nodes to word
            // Assuming length hasn't changed (Grimm's Law / Vowel Shift are 1:1)
            for (i, p) in self.word.phonemes.iter().enumerate() {
                if i < self.nodes.len() {
                    if self.nodes[i].phoneme != *p {
                        self.nodes[i].phoneme = p.clone();
                        self.nodes[i].mutated = true; // Flag for rendering
                        self.nodes[i].mass = if p.is_vowel() { 0.5 } else { 1.5 };
                        // Update mass
                    }
                }
            }
        }
    }

    pub fn pluck(&mut self) {
        let mut rng = rand::thread_rng();
        let len = self.nodes.len();
        if len > 2 {
            // Pluck a few random nodes
            for _ in 0..3 {
                let idx = rng.gen_range(1..len - 1);
                if !self.nodes[idx].fixed {
                    let force = Vec3::new(0.0, rng.gen_range(-20.0..20.0), 0.0);
                    self.nodes[idx].vel += force;
                }
            }
        }
    }

    pub fn reset_mutation_flags(&mut self) {
        for node in &mut self.nodes {
            node.mutated = false;
        }
    }
}
