#[derive(Debug, Clone)]
pub enum SoundLaw {
    GrimmsLaw,
    GreatVowelShift,
    Lenition,
    LossOfEndings,
}

impl SoundLaw {
    pub fn apply(&self, input: &str) -> String {
        match self {
            SoundLaw::GrimmsLaw => apply_grimms(input),
            SoundLaw::GreatVowelShift => apply_vowel_shift(input),
            SoundLaw::Lenition => apply_lenition(input),
            SoundLaw::LossOfEndings => apply_loss_of_endings(input),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SoundLaw::GrimmsLaw => "Grimm's Law (Germanic Shift)",
            SoundLaw::GreatVowelShift => "Great Vowel Shift",
            SoundLaw::Lenition => "Intervocalic Lenition",
            SoundLaw::LossOfEndings => "Loss of Unstressed Endings",
        }
    }
}

pub struct Evolver {
    laws: Vec<SoundLaw>,
}

impl Evolver {
    pub fn new() -> Self {
        Self { laws: vec![] }
    }

    pub fn add_law(&mut self, law: SoundLaw) {
        self.laws.push(law);
    }

    pub fn evolve(&self, word: &str) -> String {
        let mut current = word.to_string();
        for law in &self.laws {
            current = law.apply(&current);
        }
        current
    }
}

fn apply_grimms(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        let replacement = match c {
            'p' => "f".to_string(),
            't' => "th".to_string(),
            'k' => "h".to_string(),
            'b' => "p".to_string(),
            'd' => "t".to_string(),
            'g' => "k".to_string(),

            'P' => "F".to_string(),
            'T' => "Th".to_string(),
            'K' => "H".to_string(),
            'B' => "P".to_string(),
            'D' => "T".to_string(),
            'G' => "K".to_string(),

            _ => c.to_string(),
        };
        out.push_str(&replacement);
    }
    out
}

fn apply_vowel_shift(input: &str) -> String {
    // Simplified Great Vowel Shift
    // a -> ei, e -> i, i -> ai, o -> ou, u -> au
    let mut out = String::new();
    for c in input.chars() {
        let replacement = match c {
            'a' => "ei",
            'e' => "i",
            'i' => "ai",
            'o' => "ou",
            'u' => "au",

            'A' => "Ei",
            'E' => "I",
            'I' => "Ai",
            'O' => "Ou",
            'U' => "Au",
            _ => {
                out.push(c);
                continue;
            }
        };
        out.push_str(replacement);
    }
    out
}

fn apply_lenition(input: &str) -> String {
    // VcV -> VzV (Voicing of intervocalic consonants)
    // Simplified: s -> z, t -> d between vowels
    // This requires context.

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return String::new();
    }

    let mut out = String::new();

    for i in 0..chars.len() {
        let c = chars[i];
        let is_intervocalic = if i > 0 && i < chars.len() - 1 {
            is_vowel(chars[i - 1]) && is_vowel(chars[i + 1])
        } else {
            false
        };

        if is_intervocalic {
            let new_c = match c {
                's' => 'z',
                't' => 'd',
                'p' => 'b',
                'k' => 'g',
                _ => c,
            };
            out.push(new_c);
        } else {
            out.push(c);
        }
    }
    out
}

fn apply_loss_of_endings(input: &str) -> String {
    // Remove final vowels if word length > 3
    if input.len() > 3 {
        let chars: Vec<char> = input.chars().collect();
        if let Some(&last) = chars.last() {
            if is_vowel(last) {
                return chars[0..chars.len() - 1].iter().collect();
            }
        }
    }
    input.to_string()
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grimms() {
        let evolver = Evolver {
            laws: vec![SoundLaw::GrimmsLaw],
        };
        assert_eq!(evolver.evolve("pater"), "father");
        // tu -> thu (t->th, u unchanged)
        assert_eq!(evolver.evolve("tu"), "thu");
    }

    #[test]
    fn test_grimms_correction() {
        let evolver = Evolver {
            laws: vec![SoundLaw::GrimmsLaw],
        };
        assert_eq!(evolver.evolve("tu"), "thu");
    }

    #[test]
    fn test_vowel_shift() {
        let evolver = Evolver {
            laws: vec![SoundLaw::GreatVowelShift],
        };
        assert_eq!(evolver.evolve("name"), "neimi");
        // n -> n, a -> ei, m -> m, e -> i.  "neimi".
    }

    #[test]
    fn test_composition() {
        let mut evolver = Evolver::new();
        evolver.add_law(SoundLaw::GrimmsLaw);
        evolver.add_law(SoundLaw::GreatVowelShift);

        // pater -> father -> feitheir (a->ei, e->i)
        // p->f, a->a, t->th, e->e, r->r  == father
        // f->f, a->ei, t->t, h->h, e->i, r->r == feithir
        assert_eq!(evolver.evolve("pater"), "feithir");
    }
}
