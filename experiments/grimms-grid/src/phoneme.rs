use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manner {
    Stop,
    Fricative,
    Nasal,
    Approximant,
    Vowel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Bilabial,
    Labiodental,
    Alveolar,
    Palatal,
    Velar,
    Glottal,
    // Vowel approximations
    Front,
    Central,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phoneme {
    pub manner: Manner,
    pub place: Place,
    pub voiced: bool,
    pub symbol: char,
}

impl Phoneme {
    pub fn new(manner: Manner, place: Place, voiced: bool, symbol: char) -> Self {
        Self {
            manner,
            place,
            voiced,
            symbol,
        }
    }

    pub fn is_vowel(&self) -> bool {
        self.manner == Manner::Vowel
    }

    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }
}

impl TryFrom<char> for Phoneme {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            // Stops
            'p' => Ok(Phoneme::new(Manner::Stop, Place::Bilabial, false, 'p')),
            'b' => Ok(Phoneme::new(Manner::Stop, Place::Bilabial, true, 'b')),
            't' => Ok(Phoneme::new(Manner::Stop, Place::Alveolar, false, 't')),
            'd' => Ok(Phoneme::new(Manner::Stop, Place::Alveolar, true, 'd')),
            'k' => Ok(Phoneme::new(Manner::Stop, Place::Velar, false, 'k')),
            'g' => Ok(Phoneme::new(Manner::Stop, Place::Velar, true, 'g')),
            // Fricatives
            'f' => Ok(Phoneme::new(
                Manner::Fricative,
                Place::Labiodental,
                false,
                'f',
            )),
            'v' => Ok(Phoneme::new(
                Manner::Fricative,
                Place::Labiodental,
                true,
                'v',
            )),
            's' => Ok(Phoneme::new(Manner::Fricative, Place::Alveolar, false, 's')),
            'z' => Ok(Phoneme::new(Manner::Fricative, Place::Alveolar, true, 'z')),
            'h' => Ok(Phoneme::new(Manner::Fricative, Place::Glottal, false, 'h')),
            'θ' => Ok(Phoneme::new(Manner::Fricative, Place::Alveolar, false, 'θ')), // th
            'ð' => Ok(Phoneme::new(Manner::Fricative, Place::Alveolar, true, 'ð')),  // dh
            'ʃ' => Ok(Phoneme::new(Manner::Fricative, Place::Palatal, false, 'ʃ')),  // sh
            'ʒ' => Ok(Phoneme::new(Manner::Fricative, Place::Palatal, true, 'ʒ')),   // zh
            // Nasals
            'm' => Ok(Phoneme::new(Manner::Nasal, Place::Bilabial, true, 'm')),
            'n' => Ok(Phoneme::new(Manner::Nasal, Place::Alveolar, true, 'n')),
            'ŋ' => Ok(Phoneme::new(Manner::Nasal, Place::Velar, true, 'ŋ')), // ng
            // Approximants
            'l' => Ok(Phoneme::new(
                Manner::Approximant,
                Place::Alveolar,
                true,
                'l',
            )),
            'r' => Ok(Phoneme::new(
                Manner::Approximant,
                Place::Alveolar,
                true,
                'r',
            )),
            'w' => Ok(Phoneme::new(
                Manner::Approximant,
                Place::Bilabial,
                true,
                'w',
            )),
            'j' => Ok(Phoneme::new(Manner::Approximant, Place::Palatal, true, 'j')),
            // Vowels
            'a' => Ok(Phoneme::new(Manner::Vowel, Place::Central, true, 'a')),
            'e' => Ok(Phoneme::new(Manner::Vowel, Place::Front, true, 'e')),
            'i' => Ok(Phoneme::new(Manner::Vowel, Place::Front, true, 'i')),
            'o' => Ok(Phoneme::new(Manner::Vowel, Place::Back, true, 'o')),
            'u' => Ok(Phoneme::new(Manner::Vowel, Place::Back, true, 'u')),
            'y' => Ok(Phoneme::new(Manner::Vowel, Place::Front, true, 'y')),
            'ə' => Ok(Phoneme::new(Manner::Vowel, Place::Central, true, 'ə')), // schwa
            _ => Err(()),
        }
    }
}

impl Phoneme {
    pub fn all() -> Vec<Phoneme> {
        let chars = vec![
            'p', 'b', 't', 'd', 'k', 'g', 'f', 'v', 's', 'z', 'h', 'θ', 'ð', 'ʃ', 'ʒ', 'm', 'n',
            'ŋ', 'l', 'r', 'w', 'j', 'a', 'e', 'i', 'o', 'u', 'y', 'ə',
        ];
        chars
            .into_iter()
            .filter_map(|c| Phoneme::try_from(c).ok())
            .collect()
    }

    pub fn from_features(manner: Manner, place: Place, voiced: bool) -> Option<Phoneme> {
        Self::all()
            .into_iter()
            .find(|p| p.manner == manner && p.place == place && p.voiced == voiced)
    }
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let p = Phoneme::try_from('p').unwrap();
        assert_eq!(p.manner, Manner::Stop);
        assert_eq!(p.place, Place::Bilabial);
        assert!(!p.voiced);
    }
}
