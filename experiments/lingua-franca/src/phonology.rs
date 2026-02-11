use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Phoneme {
    Vowel(char),
    Consonant(char),
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Phoneme::Vowel(c) => write!(f, "{}", c),
            Phoneme::Consonant(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    pub phonemes: Vec<Phoneme>,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.phonemes {
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PhonemeType {
    V,
    C,
}

#[derive(Debug, Clone)]
pub struct Phonotactics {
    pub vowels: Vec<char>,
    pub consonants: Vec<char>,
    pub pattern: Vec<PhonemeType>,
}

impl Phonotactics {
    pub fn new(vowels: Vec<char>, consonants: Vec<char>, pattern: Vec<PhonemeType>) -> Self {
        Self {
            vowels,
            consonants,
            pattern,
        }
    }

    pub fn generate(&self) -> Word {
        let mut rng = rand::thread_rng();
        let mut phonemes = Vec::new();

        for pt in &self.pattern {
            match pt {
                PhonemeType::V => {
                    if !self.vowels.is_empty() {
                        let c = self.vowels[rng.gen_range(0..self.vowels.len())];
                        phonemes.push(Phoneme::Vowel(c));
                    }
                }
                PhonemeType::C => {
                    if !self.consonants.is_empty() {
                        let c = self.consonants[rng.gen_range(0..self.consonants.len())];
                        phonemes.push(Phoneme::Consonant(c));
                    }
                }
            }
        }
        Word { phonemes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonotactics_generation() {
        let nords = Phonotactics::new(
            vec!['a', 'i', 'u'],
            vec!['p', 't', 'k', 'r', 's'],
            vec![PhonemeType::C, PhonemeType::V, PhonemeType::C],
        );

        let word = nords.generate();

        assert_eq!(word.phonemes.len(), 3);
        match word.phonemes[0] {
            Phoneme::Consonant(_) => {}
            _ => panic!("Expected Consonant at index 0"),
        }
        match word.phonemes[1] {
            Phoneme::Vowel(_) => {}
            _ => panic!("Expected Vowel at index 1"),
        }
        match word.phonemes[2] {
            Phoneme::Consonant(_) => {}
            _ => panic!("Expected Consonant at index 2"),
        }
    }
}
