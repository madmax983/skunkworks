use macroquad::prelude::*;
use crate::phonology::{Phoneme, Word, Rule, GrimmsLaw, VowelShift};
use ::rand::Rng;

pub const G: f32 = 5000.0; // Gravity constant for Word-to-Word interaction

#[derive(Debug, Clone)]
pub struct PhonemeNode {
    pub pos: Vec2,
    pub vel: Vec2,
    pub force: Vec2,
    pub mass: f32,
    pub phoneme: Phoneme,
    pub mutated: bool,
}

impl PhonemeNode {
    pub fn new(pos: Vec2, phoneme: Phoneme) -> Self {
        let mass = if phoneme.is_vowel() { 10.0 } else { 30.0 };
        Self {
            pos,
            vel: Vec2::ZERO,
            force: Vec2::ZERO,
            mass,
            phoneme,
            mutated: false,
        }
    }
}

pub struct WordBody {
    pub nodes: Vec<PhonemeNode>,
    pub word: Word,
    pub center_of_mass: Vec2,
    pub total_mass: f32,
    pub color: Color,
    // Physics params
    pub rest_length: f32,
    pub tension: f32,
    pub damping: f32,
}

impl WordBody {
    pub fn new(text: &str, pos: Vec2, color: Color) -> Self {
        let word = Word::new(text);
        let len = word.phonemes.len();
        let mut nodes = Vec::with_capacity(len);
        let spacing = 20.0;

        // Arrange phonemes horizontally centered on pos
        let start_x = pos.x - (len as f32 - 1.0) * spacing * 0.5;

        let mut total_mass = 0.0;

        for (i, p) in word.phonemes.iter().enumerate() {
            let node_pos = vec2(start_x + i as f32 * spacing, pos.y);
            let node = PhonemeNode::new(node_pos, p.clone());
            total_mass += node.mass;
            nodes.push(node);
        }

        Self {
            nodes,
            word,
            center_of_mass: pos,
            total_mass,
            color,
            rest_length: spacing,
            tension: 50.0,
            damping: 0.98, // Air resistance
        }
    }

    pub fn update_internal(&mut self, dt: f32) {
        let len = self.nodes.len();
        if len < 2 {
             for node in &mut self.nodes {
                let acc = node.force / node.mass;
                node.vel += acc * dt;
                node.pos += node.vel * dt;
                node.vel *= self.damping;
                node.force = Vec2::ZERO;
            }
            self.calculate_properties();
            return;
        }

        let mut forces = vec![Vec2::ZERO; len];

        // Spring forces
        for i in 0..len - 1 {
            let p1 = self.nodes[i].pos;
            let p2 = self.nodes[i+1].pos;

            let delta = p2 - p1;
            let dist = delta.length();

            if dist > 0.001 {
                let dir = delta / dist;
                let stretch = dist - self.rest_length;
                let force_mag = self.tension * stretch;
                let force = dir * force_mag;

                forces[i] += force;
                forces[i+1] -= force;
            }
        }

        // Integration
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.force += forces[i]; // Add spring forces to external forces

            let acc = node.force / node.mass;
            node.vel += acc * dt;
            node.pos += node.vel * dt;
            node.vel *= self.damping;
            node.force = Vec2::ZERO; // Reset force for next frame
        }

        self.calculate_properties();
    }

    fn calculate_properties(&mut self) {
        let mut sum_pos = Vec2::ZERO;
        let mut sum_mass = 0.0;
        for node in &self.nodes {
            sum_pos += node.pos * node.mass;
            sum_mass += node.mass;
        }
        if sum_mass > 0.0 {
            self.center_of_mass = sum_pos / sum_mass;
            self.total_mass = sum_mass;
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        for node in &mut self.nodes {
            // F = ma. We want same acceleration for all nodes if gravity acts on the whole body?
            // Gravity acts on mass. So F_node = F_total * (m_node / m_total).
            // This results in uniform acceleration a = F_total / m_total.
            // Yes.
            if self.total_mass > 0.0 {
                node.force += force * (node.mass / self.total_mass);
            }
        }
    }

    pub fn apply_force_to_node(&mut self, idx: usize, force: Vec2) {
        if idx < self.nodes.len() {
            self.nodes[idx].force += force;
        }
    }

    pub fn mutate(&mut self) {
        let mut rng = ::rand::thread_rng();
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(GrimmsLaw),
            Box::new(VowelShift),
        ];

        let rule_idx = rng.gen_range(0..rules.len());
        // Since we have the word structure separately, we apply rule to it.
        // But nodes also have phonemes.
        // We should sync them.
        let changed = rules[rule_idx].apply(&mut self.word, &mut rng);

        if changed {
            for (i, p) in self.word.phonemes.iter().enumerate() {
                if i < self.nodes.len() {
                    if self.nodes[i].phoneme != *p {
                        self.nodes[i].phoneme = p.clone();
                        self.nodes[i].mutated = true;
                        self.nodes[i].mass = if p.is_vowel() { 10.0 } else { 30.0 };
                    }
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

pub fn update_gravity(bodies: &mut [WordBody]) {
    let len = bodies.len();
    // Calculate forces
    let mut forces = vec![Vec2::ZERO; len];

    for i in 0..len {
        for j in i + 1..len {
            let pos_i = bodies[i].center_of_mass;
            let pos_j = bodies[j].center_of_mass;
            let mass_i = bodies[i].total_mass;
            let mass_j = bodies[j].total_mass;

            let delta = pos_j - pos_i;
            let dist_sq = delta.length_squared();
            let dist = dist_sq.sqrt();

            if dist > 50.0 { // Minimum distance to avoid singularity
                let force_mag = (G * mass_i * mass_j) / dist_sq;
                let dir = delta / dist;
                let force = dir * force_mag;

                forces[i] += force;
                forces[j] -= force;
            }
        }
    }

    // Apply forces
    for (i, body) in bodies.iter_mut().enumerate() {
        body.apply_force(forces[i]);
    }
}
