use rand::Rng;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PhonemeType {
    Vowel,
    Consonant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phoneme {
    // Vowels
    A, E, I, O, U, Y,
    // Consonants
    B, C, D, F, G, H, J, K, L, M, N, P, Q, R, S, T, V, W, X, Z,
    // Special
    #[allow(dead_code)]
    Null, // For deletion
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Phoneme::A => "a", Phoneme::E => "e", Phoneme::I => "i", Phoneme::O => "o", Phoneme::U => "u", Phoneme::Y => "y",
            Phoneme::B => "b", Phoneme::C => "c", Phoneme::D => "d", Phoneme::F => "f", Phoneme::G => "g",
            Phoneme::H => "h", Phoneme::J => "j", Phoneme::K => "k", Phoneme::L => "l", Phoneme::M => "m",
            Phoneme::N => "n", Phoneme::P => "p", Phoneme::Q => "q", Phoneme::R => "r", Phoneme::S => "s",
            Phoneme::T => "t", Phoneme::V => "v", Phoneme::W => "w", Phoneme::X => "x", Phoneme::Z => "z",
            Phoneme::Null => "",
        };
        write!(f, "{}", s)
    }
}

impl Phoneme {
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'a' => Some(Phoneme::A), 'e' => Some(Phoneme::E), 'i' => Some(Phoneme::I), 'o' => Some(Phoneme::O), 'u' => Some(Phoneme::U), 'y' => Some(Phoneme::Y),
            'b' => Some(Phoneme::B), 'c' => Some(Phoneme::C), 'd' => Some(Phoneme::D), 'f' => Some(Phoneme::F), 'g' => Some(Phoneme::G),
            'h' => Some(Phoneme::H), 'j' => Some(Phoneme::J), 'k' => Some(Phoneme::K), 'l' => Some(Phoneme::L), 'm' => Some(Phoneme::M),
            'n' => Some(Phoneme::N), 'p' => Some(Phoneme::P), 'q' => Some(Phoneme::Q), 'r' => Some(Phoneme::R), 's' => Some(Phoneme::S),
            't' => Some(Phoneme::T), 'v' => Some(Phoneme::V), 'w' => Some(Phoneme::W), 'x' => Some(Phoneme::X), 'z' => Some(Phoneme::Z),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn is_vowel(&self) -> bool {
        matches!(self, Phoneme::A | Phoneme::E | Phoneme::I | Phoneme::O | Phoneme::U | Phoneme::Y)
    }
}

pub struct SoundChange {
    #[allow(dead_code)]
    pub name: String,
    pub pattern: fn(&[Phoneme], usize) -> bool, // Check if pattern matches at index
    pub replacement: fn(&[Phoneme], usize) -> Vec<Phoneme>, // Return replacement phonemes
    pub probability: f64,
}

pub struct EvolutionEngine {
    changes: Vec<SoundChange>,
}

impl EvolutionEngine {
    pub fn new() -> Self {
        Self {
            changes: vec![
                // Grimm's Law (simplified)
                SoundChange {
                    name: "Grimm: P->F".to_string(),
                    pattern: |p, i| p[i] == Phoneme::P,
                    replacement: |_, _| vec![Phoneme::F],
                    probability: 0.3,
                },
                SoundChange {
                    name: "Grimm: T->Th".to_string(),
                    pattern: |p, i| p[i] == Phoneme::T,
                    replacement: |_, _| vec![Phoneme::T, Phoneme::H],
                    probability: 0.3,
                },
                SoundChange {
                    name: "Grimm: K->H".to_string(),
                    pattern: |p, i| p[i] == Phoneme::K,
                    replacement: |_, _| vec![Phoneme::H],
                    probability: 0.3,
                },
                 SoundChange {
                    name: "Grimm: D->T".to_string(),
                    pattern: |p, i| p[i] == Phoneme::D,
                    replacement: |_, _| vec![Phoneme::T],
                    probability: 0.3,
                },
                SoundChange {
                    name: "Grimm: G->K".to_string(),
                    pattern: |p, i| p[i] == Phoneme::G,
                    replacement: |_, _| vec![Phoneme::K],
                    probability: 0.3,
                },

                // Vowel Shift (A -> E -> I -> O -> U -> A)
                SoundChange {
                    name: "Vowel Shift A->E".to_string(),
                    pattern: |p, i| p[i] == Phoneme::A,
                    replacement: |_, _| vec![Phoneme::E],
                    probability: 0.2,
                },
                SoundChange {
                    name: "Vowel Shift E->I".to_string(),
                    pattern: |p, i| p[i] == Phoneme::E,
                    replacement: |_, _| vec![Phoneme::I],
                    probability: 0.2,
                },
                SoundChange {
                    name: "Vowel Shift I->O".to_string(),
                    pattern: |p, i| p[i] == Phoneme::I,
                    replacement: |_, _| vec![Phoneme::O],
                    probability: 0.2,
                },

                // Lenition (Intervocalic voicing)
                SoundChange {
                    name: "Lenition S->Z".to_string(),
                    pattern: |p, i| {
                        if i > 0 && i < p.len() - 1 {
                             p[i] == Phoneme::S && p[i-1].is_vowel() && p[i+1].is_vowel()
                        } else {
                            false
                        }
                    },
                    replacement: |_, _| vec![Phoneme::Z],
                    probability: 0.4,
                },
                 // Palatalization K->C before I/E
                SoundChange {
                    name: "Palatalization K->C".to_string(),
                    pattern: |p, i| {
                         p[i] == Phoneme::K && i < p.len() - 1 && (p[i+1] == Phoneme::I || p[i+1] == Phoneme::E)
                    },
                    replacement: |_, _| vec![Phoneme::C],
                    probability: 0.5,
                },
            ]
        }
    }

    pub fn evolve_word(&self, word: &str, rng: &mut impl Rng) -> String {
        let phonemes: Vec<Phoneme> = word.chars()
            .filter_map(Phoneme::from_char)
            .collect();

        // Apply changes
        // Since replacements can change length, we construct a new vector
        let mut new_phonemes = Vec::new();
        let mut i = 0;
        while i < phonemes.len() {
            let mut applied = false;
            for change in &self.changes {
                if (change.pattern)(&phonemes, i) {
                    if rng.gen::<f64>() < change.probability {
                        let replacement = (change.replacement)(&phonemes, i);
                        new_phonemes.extend(replacement);
                        i += 1;
                        applied = true;
                        break;
                    }
                }
            }
            if !applied {
                new_phonemes.push(phonemes[i].clone());
                i += 1;
            }
        }

        new_phonemes.iter().map(|p| p.to_string()).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_phoneme_conversion() {
        assert_eq!(Phoneme::from_char('a'), Some(Phoneme::A));
        assert_eq!(Phoneme::from_char('z'), Some(Phoneme::Z));
        assert_eq!(Phoneme::from_char('!'), None);
    }

    #[test]
    fn test_grimm_p_to_f() {
        let engine = EvolutionEngine::new();
        // Force probability?
        // We can just iterate until it happens or check the logic.
        // Or we can mock the RNG if we make the function generic.
        // For now, let's just check that it *can* happen.

        let mut rng = StdRng::seed_from_u64(42);
        let mut changed = false;
        for _ in 0..100 {
            let res = engine.evolve_word("pater", &mut rng);
            if res.starts_with("f") {
                changed = true;
                break;
            }
        }
        assert!(changed, "P should eventually evolve to F");
    }

    #[test]
    fn test_vowel_shift() {
        let engine = EvolutionEngine::new();
        let mut rng = StdRng::seed_from_u64(123);
        let mut changed = false;
        for _ in 0..100 {
            let res = engine.evolve_word("aaa", &mut rng);
            if res.contains("e") {
                changed = true;
                break;
            }
        }
        assert!(changed, "A should eventually evolve to E");
    }
}
