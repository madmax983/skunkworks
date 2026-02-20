use rand::{Rng, RngCore};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Voice {
    Voiced,
    Voiceless,
    Neutral, // For vowels/liquids that don't contrast voicing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Manner {
    Stop,
    Fricative,
    Nasal,
    Liquid,
    Vowel,
    Approximant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Place {
    Labial,   // p, b, m, f, v
    Alveolar, // t, d, n, s, z, l, r
    Palatal,  // j, ch, sh
    Velar,    // k, g, ng
    Glottal,  // h
    Front,    // i, e
    Central,  // a
    Back,     // u, o
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phoneme {
    pub symbol: char,
    pub voice: Voice,
    pub manner: Manner,
    pub place: Place,
}

impl Phoneme {
    pub fn new(symbol: char, voice: Voice, manner: Manner, place: Place) -> Self {
        Self {
            symbol,
            voice,
            manner,
            place,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            // Stops
            'p' => Some(Self::new(
                'p',
                Voice::Voiceless,
                Manner::Stop,
                Place::Labial,
            )),
            'b' => Some(Self::new('b', Voice::Voiced, Manner::Stop, Place::Labial)),
            't' => Some(Self::new(
                't',
                Voice::Voiceless,
                Manner::Stop,
                Place::Alveolar,
            )),
            'd' => Some(Self::new('d', Voice::Voiced, Manner::Stop, Place::Alveolar)),
            'k' => Some(Self::new('k', Voice::Voiceless, Manner::Stop, Place::Velar)),
            'g' => Some(Self::new('g', Voice::Voiced, Manner::Stop, Place::Velar)),
            'c' => Some(Self::new('c', Voice::Voiceless, Manner::Stop, Place::Velar)), // Hard C
            'q' => Some(Self::new('q', Voice::Voiceless, Manner::Stop, Place::Velar)), // Like k
            // Fricatives
            'f' => Some(Self::new(
                'f',
                Voice::Voiceless,
                Manner::Fricative,
                Place::Labial,
            )),
            'v' => Some(Self::new(
                'v',
                Voice::Voiced,
                Manner::Fricative,
                Place::Labial,
            )),
            's' => Some(Self::new(
                's',
                Voice::Voiceless,
                Manner::Fricative,
                Place::Alveolar,
            )),
            'z' => Some(Self::new(
                'z',
                Voice::Voiced,
                Manner::Fricative,
                Place::Alveolar,
            )),
            'h' => Some(Self::new(
                'h',
                Voice::Voiceless,
                Manner::Fricative,
                Place::Glottal,
            )),
            'x' => Some(Self::new(
                'x',
                Voice::Voiceless,
                Manner::Fricative,
                Place::Velar,
            )), // Pseudo-phoneme
            // Nasals
            'm' => Some(Self::new('m', Voice::Voiced, Manner::Nasal, Place::Labial)),
            'n' => Some(Self::new(
                'n',
                Voice::Voiced,
                Manner::Nasal,
                Place::Alveolar,
            )),
            // Liquids/Approximants
            'l' => Some(Self::new(
                'l',
                Voice::Voiced,
                Manner::Liquid,
                Place::Alveolar,
            )),
            'r' => Some(Self::new(
                'r',
                Voice::Voiced,
                Manner::Liquid,
                Place::Alveolar,
            )),
            'j' => Some(Self::new(
                'j',
                Voice::Voiced,
                Manner::Approximant,
                Place::Palatal,
            )),
            'w' => Some(Self::new(
                'w',
                Voice::Voiced,
                Manner::Approximant,
                Place::Labial,
            )),
            // Vowels
            'a' => Some(Self::new('a', Voice::Voiced, Manner::Vowel, Place::Central)),
            'e' => Some(Self::new('e', Voice::Voiced, Manner::Vowel, Place::Front)),
            'i' => Some(Self::new('i', Voice::Voiced, Manner::Vowel, Place::Front)),
            'o' => Some(Self::new('o', Voice::Voiced, Manner::Vowel, Place::Back)),
            'u' => Some(Self::new('u', Voice::Voiced, Manner::Vowel, Place::Back)),
            'y' => Some(Self::new('y', Voice::Voiced, Manner::Vowel, Place::Front)), // simplified
            _ => None,
        }
    }

    pub fn is_vowel(&self) -> bool {
        self.manner == Manner::Vowel
    }

    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub phonemes: Vec<Phoneme>,
}

impl Word {
    pub fn new(s: &str) -> Self {
        let phonemes = s.chars().filter_map(Phoneme::from_char).collect();
        Self { phonemes }
    }

    pub fn to_string(&self) -> String {
        self.phonemes.iter().map(|p| p.symbol).collect()
    }
}

pub trait Rule {
    fn apply(&self, word: &mut Word, rng: &mut dyn RngCore) -> bool;
}

// Implement some concrete rules

pub struct GrimmsLaw;

impl Rule for GrimmsLaw {
    fn apply(&self, word: &mut Word, rng: &mut dyn RngCore) -> bool {
        let mut changed = false;
        for i in 0..word.phonemes.len() {
            let p = &mut word.phonemes[i];

            // Voiceless Stop -> Voiceless Fricative (p->f, t->θ(th), k->h/x)
            if p.voice == Voice::Voiceless && p.manner == Manner::Stop {
                if rng.gen_bool(0.3) {
                    match p.place {
                        Place::Labial => *p = Phoneme::from_char('f').unwrap(),
                        Place::Alveolar => {
                            // Simplified: t -> th (represented as 'T' or just 'th' digraph?
                            // Let's stick to single chars for simplicity in this moonshot: t -> s/z or special char?
                            // Let's map t -> s (spirantization) or introduce 'θ' if supported.
                            // For ASCII code, maybe use 'T' for Theta?
                            // Let's use 'z' or 's' for simplicity or 'h'.
                            // Real Grimm's law: t -> θ. Let's use 's' as a proxy for fricative.
                            *p = Phoneme::from_char('s').unwrap();
                        }
                        Place::Velar => *p = Phoneme::from_char('h').unwrap(),
                        _ => {}
                    }
                    changed = true;
                }
            }
            // Voiced Stop -> Voiceless Stop (b->p, d->t, g->k)
            else if p.voice == Voice::Voiced && p.manner == Manner::Stop {
                if rng.gen_bool(0.3) {
                    match p.place {
                        Place::Labial => *p = Phoneme::from_char('p').unwrap(),
                        Place::Alveolar => *p = Phoneme::from_char('t').unwrap(),
                        Place::Velar => *p = Phoneme::from_char('k').unwrap(),
                        _ => {}
                    }
                    changed = true;
                }
            }
        }
        changed
    }
}

pub struct VowelShift;

impl Rule for VowelShift {
    fn apply(&self, word: &mut Word, rng: &mut dyn RngCore) -> bool {
        let mut changed = false;
        for p in &mut word.phonemes {
            if p.manner == Manner::Vowel {
                if rng.gen_bool(0.2) {
                    match p.symbol {
                        'a' => *p = Phoneme::from_char('e').unwrap(),
                        'e' => *p = Phoneme::from_char('i').unwrap(),
                        'i' => *p = Phoneme::from_char('o').unwrap(), // stylized shift
                        'o' => *p = Phoneme::from_char('u').unwrap(),
                        'u' => *p = Phoneme::from_char('a').unwrap(),
                        _ => {}
                    }
                    changed = true;
                }
            }
        }
        changed
    }
}
