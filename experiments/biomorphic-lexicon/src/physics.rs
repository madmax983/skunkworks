use glam::Vec3;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct LexicalNode {
    pub pos: Vec3,
    pub vel: Vec3,
    pub force: Vec3,
    pub fixed: bool,
    pub mass: f32,
    pub char: char,
    pub mutated: bool, // For visual feedback
}

impl LexicalNode {
    pub fn new(pos: Vec3, fixed: bool, c: char) -> Self {
        // Mass depends on phoneme type: Vowels light, Consonants heavy
        let mass = if is_vowel(c) { 0.5 } else { 1.5 };
        Self {
            pos,
            vel: Vec3::ZERO,
            force: Vec3::ZERO,
            fixed,
            mass,
            char: c,
            mutated: false,
        }
    }
}

pub struct LexicalString {
    pub nodes: Vec<LexicalNode>,
    pub rest_length: f32,
    pub tension: f32,
    pub damping: f32,
    pub mutation_threshold: f32,
}

impl LexicalString {
    pub fn new(text: &str, start: Vec3, end: Vec3, tension: f32, damping: f32) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        let mut nodes = Vec::with_capacity(len);

        if len > 1 {
            let step = (end - start) / (len - 1) as f32;
            let rest_length = step.length();

            for (i, &c) in chars.iter().enumerate() {
                let pos = start + step * i as f32;
                // Fix the ends
                let fixed = i == 0 || i == len - 1;
                nodes.push(LexicalNode::new(pos, fixed, c));
            }

            Self {
                nodes,
                rest_length,
                tension,
                damping,
                mutation_threshold: 50.0, // Energy threshold
            }
        } else {
            // Handle single char word?
            let nodes = vec![LexicalNode::new(start, true, chars[0])];
            Self {
                nodes,
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

        // Pick a random node to mutate
        let len = self.nodes.len();
        if len == 0 {
            return;
        }

        // Maybe mutate multiple? Or just one? Original logic mutated the *word* via rule.
        // Let's iterate and mutate with small chance
        for node in &mut self.nodes {
            if rng.gen_bool(0.1) {
                let old_char = node.char;
                let new_char = if is_vowel(old_char) {
                    mutate_vowel(old_char)
                } else {
                    mutate_consonant(old_char)
                };

                if new_char != old_char {
                    node.char = new_char;
                    node.mutated = true;
                    // Update mass
                    node.mass = if is_vowel(new_char) { 0.5 } else { 1.5 };
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

pub fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

fn mutate_consonant(c: char) -> char {
    match c.to_ascii_lowercase() {
        // Voiceless Stop -> Voiceless Fricative (Grimm's Law simplified)
        'p' => 'f',
        't' => 's', // or 'θ'
        'k' => 'h',
        // Voiced Stop -> Voiceless Stop
        'b' => 'p',
        'd' => 't',
        'g' => 'k',
        // Fricatives -> Maybe change place?
        'f' => 'v', // Voicing
        's' => 'z',
        _ => c,
    }
}

fn mutate_vowel(c: char) -> char {
    match c.to_ascii_lowercase() {
        // Great Vowel Shift simplified
        'a' => 'e',
        'e' => 'i',
        'i' => 'o', // approximate
        'o' => 'u',
        'u' => 'a',
        _ => c,
    }
}

impl std::fmt::Display for LexicalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            write!(f, "{}", node.char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('E'));
        assert!(!is_vowel('b'));
    }

    #[test]
    fn test_mutate_consonant() {
        // Grimm's law examples
        assert_eq!(mutate_consonant('p'), 'f');
        assert_eq!(mutate_consonant('t'), 's');
        assert_eq!(mutate_consonant('k'), 'h');
        assert_eq!(mutate_consonant('b'), 'p');
        assert_eq!(mutate_consonant('d'), 't');
        assert_eq!(mutate_consonant('g'), 'k');
        // No change
        assert_eq!(mutate_consonant('m'), 'm');
    }

    #[test]
    fn test_mutate_vowel() {
        assert_eq!(mutate_vowel('a'), 'e');
        assert_eq!(mutate_vowel('e'), 'i');
        assert_eq!(mutate_vowel('i'), 'o');
        assert_eq!(mutate_vowel('o'), 'u');
        assert_eq!(mutate_vowel('u'), 'a');
    }

    #[test]
    fn test_lexical_string_creation() {
        let s = LexicalString::new("test", Vec3::ZERO, Vec3::X, 1.0, 0.1);
        assert_eq!(s.nodes.len(), 4);
        assert_eq!(s.to_string(), "test");
        assert!(s.nodes[0].fixed);
        assert!(s.nodes[3].fixed);
        assert!(!s.nodes[1].fixed);
    }
}
