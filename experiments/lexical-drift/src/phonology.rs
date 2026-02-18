use rand::Rng;

#[derive(Clone, Debug)]
pub enum SoundChange {
    GrimmsLaw,
    GreatVowelShift,
    Syncope, // Vowel loss
    Lenition, // Weakening
    Palatalization, // k -> ch before front vowels
}

pub struct PhonologyEngine {
    pub changes: Vec<SoundChange>,
}

impl PhonologyEngine {
    pub fn new() -> Self {
        Self {
            changes: vec![
                SoundChange::GrimmsLaw,
                SoundChange::GreatVowelShift,
                SoundChange::Syncope,
                SoundChange::Lenition,
                SoundChange::Palatalization,
            ],
        }
    }

    pub fn with_changes(changes: Vec<SoundChange>) -> Self {
        Self { changes }
    }

    pub fn evolve(&self, word: &str, rng: &mut impl Rng) -> String {
        let mut current_word = word.to_string();

        for change in &self.changes {
            current_word = apply_change(&current_word, change, rng);
        }

        current_word
    }
}

fn apply_change(word: &str, change: &SoundChange, rng: &mut impl Rng) -> String {
    let chars: Vec<char> = word.chars().collect();
    let mut new_chars = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let next = if i + 1 < chars.len() { Some(chars[i+1]) } else { None };

        match change {
            SoundChange::GrimmsLaw => {
                match c {
                    'p' => new_chars.push('f'),
                    't' => {
                        new_chars.push('t');
                        new_chars.push('h');
                    },
                    'k' | 'c' => new_chars.push('h'), // Treat 'c' as hard 'k' for simplicity
                    'b' => new_chars.push('p'),
                    'd' => new_chars.push('t'),
                    'g' => new_chars.push('k'),
                    _ => new_chars.push(c),
                }
            },
            SoundChange::GreatVowelShift => {
                match c {
                    'a' => new_chars.push('e'),
                    'e' => new_chars.push('i'),
                    'i' => {
                        new_chars.push('a');
                        new_chars.push('i');
                    },
                    'o' => new_chars.push('u'),
                    'u' => {
                        new_chars.push('a');
                        new_chars.push('u');
                    },
                    _ => new_chars.push(c),
                }
            },
            SoundChange::Syncope => {
                if is_vowel(c) && i > 0 && rng.gen_bool(0.1) {
                    // Drop it
                } else {
                    new_chars.push(c);
                }
            },
            SoundChange::Lenition => {
                if i > 0 && i + 1 < chars.len() && is_vowel(chars[i-1]) && is_vowel(chars[i+1]) {
                     match c {
                        'p' => new_chars.push('b'),
                        't' => new_chars.push('d'),
                        'k' | 'c' => new_chars.push('g'),
                        'b' => new_chars.push('v'),
                        'd' => {
                             new_chars.push('t');
                             new_chars.push('h');
                        },
                        'g' => {
                             if rng.gen_bool(0.5) {
                                 new_chars.push('y');
                             } else {
                                 new_chars.push('w');
                             }
                        },
                        _ => new_chars.push(c),
                     }
                } else {
                    new_chars.push(c);
                }
            },
             SoundChange::Palatalization => {
                if let Some(n) = next {
                    if is_front_vowel(n) {
                         match c {
                            'k' | 'c' => {
                                new_chars.push('c');
                                new_chars.push('h');
                            },
                            'g' => new_chars.push('j'),
                            't' => {
                                new_chars.push('s');
                                new_chars.push('h');
                            },
                            'd' => {
                                new_chars.push('z');
                            },
                            's' => {
                                new_chars.push('s');
                                new_chars.push('h');
                            },
                             _ => new_chars.push(c),
                         }
                    } else {
                        new_chars.push(c);
                    }
                } else {
                    new_chars.push(c);
                }
            }
        }
        i += 1;
    }

    new_chars.into_iter().collect()
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'A' | 'E' | 'I' | 'O' | 'U' | 'Y')
}

fn is_front_vowel(c: char) -> bool {
    matches!(c, 'e' | 'i' | 'y' | 'E' | 'I' | 'Y')
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_grimms_law() {
        let mut rng = StdRng::seed_from_u64(42);
        let engine = PhonologyEngine::with_changes(vec![SoundChange::GrimmsLaw]);

        assert_eq!(engine.evolve("pater", &mut rng), "father");
        assert_eq!(engine.evolve("tu", &mut rng), "thu");
        assert_eq!(engine.evolve("cord", &mut rng), "hort"); // c->h, d->t
    }

    #[test]
    fn test_great_vowel_shift() {
        let mut rng = StdRng::seed_from_u64(42);
        let engine = PhonologyEngine::with_changes(vec![SoundChange::GreatVowelShift]);

        assert_eq!(engine.evolve("name", &mut rng), "nemi"); // a->e, e->i
        assert_eq!(engine.evolve("bite", &mut rng), "baiti"); // i->ai, e->i
    }

    #[test]
    fn test_palatalization() {
        let mut rng = StdRng::seed_from_u64(42);
        let engine = PhonologyEngine::with_changes(vec![SoundChange::Palatalization]);

        assert_eq!(engine.evolve("kettle", &mut rng), "chettle"); // k before e
        assert_eq!(engine.evolve("gift", &mut rng), "jift"); // g before i
        assert_eq!(engine.evolve("cut", &mut rng), "cut"); // c/k before u (no change)
    }

    #[test]
    fn test_lenition() {
         let mut rng = StdRng::seed_from_u64(42);
         let engine = PhonologyEngine::with_changes(vec![SoundChange::Lenition]);

         // 'aba' -> 'ava'
         assert_eq!(engine.evolve("aba", &mut rng), "ava");

         // 'ata' -> 'ada'
         assert_eq!(engine.evolve("ata", &mut rng), "ada");
    }
}
