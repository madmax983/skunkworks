#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Phoneme {
    Vowel(Vowel),
    Consonant(Consonant),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Vowel {
    A, E, I, O, U, // Basic 5-vowel system for Proto-Code
    // Add more for evolution
    Ae, // Ash (cat)
    Schwa, // uh
    Y, // u-umlaut
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Consonant {
    P, B, M, // Bilabial
    F, V, // Labiodental
    Th, Dh, // Dental
    T, D, N, // Alveolar
    S, Z, // Alveolar Fricative
    L, R, // Liquid
    Sh, Zh, // Post-alveolar
    Ch, Jh, // Affricate
    K, G, Ng, // Velar
    H, // Glottal
    W, Y, // Semivowels
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub phonemes: Vec<Phoneme>,
    pub original: String,
}

impl Word {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for p in &self.phonemes {
            match p {
                Phoneme::Vowel(v) => match v {
                    Vowel::A => s.push('a'),
                    Vowel::E => s.push('e'),
                    Vowel::I => s.push('i'),
                    Vowel::O => s.push('o'),
                    Vowel::U => s.push('u'),
                    Vowel::Ae => s.push_str("ae"),
                    Vowel::Schwa => s.push('e'),
                    Vowel::Y => s.push('y'),
                },
                Phoneme::Consonant(c) => match c {
                    Consonant::P => s.push('p'),
                    Consonant::B => s.push('b'),
                    Consonant::M => s.push('m'),
                    Consonant::F => s.push('f'),
                    Consonant::V => s.push('v'),
                    Consonant::Th => s.push_str("th"),
                    Consonant::Dh => s.push_str("dh"),
                    Consonant::T => s.push('t'),
                    Consonant::D => s.push('d'),
                    Consonant::N => s.push('n'),
                    Consonant::S => s.push('s'),
                    Consonant::Z => s.push('z'),
                    Consonant::L => s.push('l'),
                    Consonant::R => s.push('r'),
                    Consonant::Sh => s.push_str("sh"),
                    Consonant::Zh => s.push_str("zh"),
                    Consonant::Ch => s.push_str("ch"),
                    Consonant::Jh => s.push('j'),
                    Consonant::K => s.push('k'),
                    Consonant::G => s.push('g'),
                    Consonant::Ng => s.push_str("ng"),
                    Consonant::H => s.push('h'),
                    Consonant::W => s.push('w'),
                    Consonant::Y => s.push('y'),
                },
            }
        }
        s
    }
}

pub fn parse_identifier(text: &str) -> Word {
    // A very rough grapheme-to-phoneme converter for "Code-English"
    let mut phonemes = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i].to_ascii_lowercase();
        let next = if i + 1 < chars.len() { Some(chars[i+1].to_ascii_lowercase()) } else { None };

        match c {
            'a' => phonemes.push(Phoneme::Vowel(Vowel::A)),
            'e' => phonemes.push(Phoneme::Vowel(Vowel::E)),
            'i' => phonemes.push(Phoneme::Vowel(Vowel::I)),
            'o' => phonemes.push(Phoneme::Vowel(Vowel::O)),
            'u' => phonemes.push(Phoneme::Vowel(Vowel::U)),
            'y' => {
                 // Y is tricky. Treating as vowel at end or consonant at start?
                 // Let's say Consonant Y for now, or Vowel I.
                 phonemes.push(Phoneme::Consonant(Consonant::Y));
            },

            'p' => {
                if next == Some('h') {
                    phonemes.push(Phoneme::Consonant(Consonant::F));
                    i += 1;
                } else {
                    phonemes.push(Phoneme::Consonant(Consonant::P));
                }
            },
            'b' => phonemes.push(Phoneme::Consonant(Consonant::B)),
            'm' => phonemes.push(Phoneme::Consonant(Consonant::M)),
            'f' => phonemes.push(Phoneme::Consonant(Consonant::F)),
            'v' => phonemes.push(Phoneme::Consonant(Consonant::V)),

            't' => {
                if next == Some('h') {
                    phonemes.push(Phoneme::Consonant(Consonant::Th));
                    i += 1;
                } else {
                    phonemes.push(Phoneme::Consonant(Consonant::T));
                }
            },
            'd' => phonemes.push(Phoneme::Consonant(Consonant::D)),
            'n' => {
                if next == Some('g') {
                    phonemes.push(Phoneme::Consonant(Consonant::Ng));
                    i += 1;
                } else {
                    phonemes.push(Phoneme::Consonant(Consonant::N));
                }
            },

            's' => {
                if next == Some('h') {
                    phonemes.push(Phoneme::Consonant(Consonant::Sh));
                    i += 1;
                } else {
                    phonemes.push(Phoneme::Consonant(Consonant::S));
                }
            },
            'z' => phonemes.push(Phoneme::Consonant(Consonant::Z)),

            'c' => {
                if next == Some('h') {
                    phonemes.push(Phoneme::Consonant(Consonant::Ch));
                    i += 1;
                } else if matches!(next, Some('e') | Some('i') | Some('y')) {
                    phonemes.push(Phoneme::Consonant(Consonant::S));
                } else {
                    phonemes.push(Phoneme::Consonant(Consonant::K));
                }
            },
            'k' => phonemes.push(Phoneme::Consonant(Consonant::K)),
            'g' => {
                // Hard G default for code (get, git, go)
                 phonemes.push(Phoneme::Consonant(Consonant::G));
            },
            'q' => {
                phonemes.push(Phoneme::Consonant(Consonant::K));
                if next == Some('u') {
                    phonemes.push(Phoneme::Consonant(Consonant::W));
                    i += 1;
                }
            },
            'x' => {
                phonemes.push(Phoneme::Consonant(Consonant::K));
                phonemes.push(Phoneme::Consonant(Consonant::S));
            },

            'l' => phonemes.push(Phoneme::Consonant(Consonant::L)),
            'r' => phonemes.push(Phoneme::Consonant(Consonant::R)),
            'j' => phonemes.push(Phoneme::Consonant(Consonant::Jh)),
            'w' => phonemes.push(Phoneme::Consonant(Consonant::W)),
            'h' => phonemes.push(Phoneme::Consonant(Consonant::H)),

            '_' => {
                // Ignore underscore or treat as break?
                // Let's ignore for sound, maybe it implies a syllable break?
            },

            _ => {}, // Ignore numbers/symbols in phonology for now
        }
        i += 1;
    }

    Word {
        phonemes,
        original: text.to_string(),
    }
}

// Evolution Engine

#[derive(Clone, Copy)]
pub enum RuleType {
    Grimm,
    VowelShift,
    Lenition,
    Assimilation,
}

pub fn evolve(word: &mut Word, rule: RuleType) {
    let old_phonemes = word.phonemes.clone();
    let mut new_phonemes = Vec::new();

    match rule {
        RuleType::Grimm => {
            // Grimm's Law (simplified)
            // P -> F
            // T -> Th
            // K -> H
            // B -> P
            // D -> T
            // G -> K
            // Bh -> B, Dh -> D, Gh -> G (Input doesn't have Bh/Dh/Gh usually, but let's assume voiced fricatives/stops map)

            for p in old_phonemes {
                match p {
                    Phoneme::Consonant(c) => match c {
                        Consonant::P => new_phonemes.push(Phoneme::Consonant(Consonant::F)),
                        Consonant::T => new_phonemes.push(Phoneme::Consonant(Consonant::Th)),
                        Consonant::K => new_phonemes.push(Phoneme::Consonant(Consonant::H)),

                        Consonant::B => new_phonemes.push(Phoneme::Consonant(Consonant::P)),
                        Consonant::D => new_phonemes.push(Phoneme::Consonant(Consonant::T)),
                        Consonant::G => new_phonemes.push(Phoneme::Consonant(Consonant::K)),

                        _ => new_phonemes.push(p),
                    },
                    _ => new_phonemes.push(p),
                }
            }
        },
        RuleType::VowelShift => {
            // Great Vowel Shift (Very simplified)
            // A -> E
            // E -> I
            // I -> Ai
            // O -> U
            // U -> Au
            for p in old_phonemes {
                match p {
                    Phoneme::Vowel(v) => match v {
                        Vowel::A => new_phonemes.push(Phoneme::Vowel(Vowel::E)),
                        Vowel::E => new_phonemes.push(Phoneme::Vowel(Vowel::I)),
                        Vowel::I => {
                            new_phonemes.push(Phoneme::Vowel(Vowel::A));
                            new_phonemes.push(Phoneme::Vowel(Vowel::I));
                        },
                        Vowel::O => new_phonemes.push(Phoneme::Vowel(Vowel::U)),
                        Vowel::U => {
                            new_phonemes.push(Phoneme::Vowel(Vowel::A));
                            new_phonemes.push(Phoneme::Vowel(Vowel::U)); // Au
                        },
                        _ => new_phonemes.push(p),
                    },
                    _ => new_phonemes.push(p),
                }
            }
        },
        RuleType::Lenition => {
             // Intervocalic voicing/softening
             // VCV -> VFV / VDV
             // P -> B between vowels
             // T -> D between vowels
             // K -> G between vowels
             for i in 0..old_phonemes.len() {
                 let p = old_phonemes[i];
                 let prev = if i > 0 { Some(old_phonemes[i-1]) } else { None };
                 let next = if i + 1 < old_phonemes.len() { Some(old_phonemes[i+1]) } else { None };

                 let is_vowel = |ph: Option<Phoneme>| matches!(ph, Some(Phoneme::Vowel(_)));

                 if is_vowel(prev) && is_vowel(next) {
                     match p {
                         Phoneme::Consonant(c) => match c {
                             Consonant::P => new_phonemes.push(Phoneme::Consonant(Consonant::B)),
                             Consonant::T => new_phonemes.push(Phoneme::Consonant(Consonant::D)),
                             Consonant::K => new_phonemes.push(Phoneme::Consonant(Consonant::G)),
                             Consonant::S => new_phonemes.push(Phoneme::Consonant(Consonant::Z)),
                             _ => new_phonemes.push(p),
                         },
                         _ => new_phonemes.push(p),
                     }
                 } else {
                     new_phonemes.push(p);
                 }
             }
        },
         RuleType::Assimilation => {
            // N -> M before P/B/M
            // N -> Ng before K/G
             for i in 0..old_phonemes.len() {
                 let p = old_phonemes[i];
                 let next = if i + 1 < old_phonemes.len() { Some(old_phonemes[i+1]) } else { None };

                 match p {
                     Phoneme::Consonant(Consonant::N) => {
                         match next {
                             Some(Phoneme::Consonant(c)) => match c {
                                 Consonant::P | Consonant::B | Consonant::M =>
                                     new_phonemes.push(Phoneme::Consonant(Consonant::M)),
                                 Consonant::K | Consonant::G =>
                                     new_phonemes.push(Phoneme::Consonant(Consonant::Ng)),
                                 _ => new_phonemes.push(p),
                             },
                             _ => new_phonemes.push(p),
                         }
                     },
                     _ => new_phonemes.push(p),
                 }
             }
        }
    }

    word.phonemes = new_phonemes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grimms_law() {
        let mut w = parse_identifier("pater");
        // p -> f, t -> th. pater -> father
        evolve(&mut w, RuleType::Grimm);
        assert_eq!(w.to_string(), "father");

        let mut w2 = parse_identifier("cord");
        // c -> k -> h. d -> t. cord -> hort
        evolve(&mut w2, RuleType::Grimm);
        assert_eq!(w2.to_string(), "hort");
    }

    #[test]
    fn test_lenition() {
        let mut w = parse_identifier("pata");
        // t between a/a -> d. pata -> pada
        evolve(&mut w, RuleType::Lenition);
        assert_eq!(w.to_string(), "pada");
    }

    #[test]
    fn test_vowel_shift() {
        let mut w = parse_identifier("name");
        // a -> e. e -> i. name -> nemi
        evolve(&mut w, RuleType::VowelShift);
        assert_eq!(w.to_string(), "nemi");
    }
}
