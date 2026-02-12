use crate::phoneme::{Manner, Phoneme, Place};
use rand::Rng;

pub fn evolve(center: Option<Phoneme>, neighbors: &[Option<Phoneme>]) -> Option<Phoneme> {
    let mut rng = rand::thread_rng();

    // Neighbors indices:
    // 0: NW, 1: N, 2: NE
    // 3: W,       4: E
    // 5: SW, 6: S, 7: SE

    let left = neighbors.get(3).cloned().flatten();
    let right = neighbors.get(4).cloned().flatten();
    let up = neighbors.get(1).cloned().flatten();
    let down = neighbors.get(6).cloned().flatten();

    match center {
        None => {
            // Epenthesis (Birth)
            // If squeezed between two consonants, spawn a schwa.
            if let (Some(l), Some(r)) = (left, right) {
                if l.is_consonant() && r.is_consonant() && rng.gen_bool(0.05) {
                    return Phoneme::try_from('ə').ok();
                }
            }
            // Spontaneous generation (Noise)
            if rng.gen_bool(0.0001) {
                // Random vowel
                if rng.gen_bool(0.5) {
                    return Phoneme::try_from('a').ok();
                } else {
                    return Phoneme::try_from('i').ok();
                }
            }
            None
        }
        Some(p) => {
            let mut new_p = p;

            // 1. Assimilation (Voicing)
            // If surrounded by voiced segments (including vowels), become voiced.
            if !p.voiced && rng.gen_bool(0.05) {
                if let (Some(l), Some(r)) = (left, right) {
                    if l.voiced && r.voiced {
                        if let Some(voiced) = Phoneme::from_features(p.manner, p.place, true) {
                            new_p = voiced;
                        }
                    }
                }
            }

            // 2. Palatalization
            // If followed by front vowel (i, e, y)
            if (p.place == Place::Velar || p.place == Place::Alveolar) && rng.gen_bool(0.05) {
                if let Some(r) = right {
                    if r.is_vowel() && (r.place == Place::Front) {
                        let target_manner = if p.manner == Manner::Stop {
                            Manner::Fricative
                        } else {
                            p.manner
                        };
                        if let Some(pal) =
                            Phoneme::from_features(target_manner, Place::Palatal, p.voiced)
                        {
                            new_p = pal;
                        }
                    }
                }
            }

            // 3. Lenition (Weakening)
            // Intervocalic: Stop -> Fricative
            if p.manner == Manner::Stop && rng.gen_bool(0.05) {
                if let (Some(l), Some(r)) = (left, right) {
                    if l.is_vowel() && r.is_vowel() {
                        if let Some(fric) =
                            Phoneme::from_features(Manner::Fricative, p.place, p.voiced)
                        {
                            new_p = fric;
                        }
                    }
                }
            }

            // 4. Deletion (Death)
            // If in a cluster of 3 consonants, die.
            if p.is_consonant() && rng.gen_bool(0.1) {
                if let (Some(l), Some(r)) = (left, right) {
                    if l.is_consonant() && r.is_consonant() {
                        return None;
                    }
                }
            }

            // 5. Vertical Diffusion (Dialect contact)
            // If Up or Down is different, maybe copy them?
            if rng.gen_bool(0.01) {
                if rng.gen_bool(0.5) {
                    if let Some(u) = up {
                        return Some(u);
                    }
                } else if let Some(d) = down {
                    return Some(d);
                }
            }

            Some(new_p)
        }
    }
}
