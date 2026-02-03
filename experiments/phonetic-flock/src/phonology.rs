#[derive(Debug, Clone)]
pub enum SoundLaw {
    GrimmsLaw,
    GreatVowelShift,
    Lenition,
    LossOfEndings,
    Palatalization,
    Metathesis,
    Rhotacism,
    ClusterSimplification,
    HDropping,
}

impl SoundLaw {
    pub fn apply(&self, input: &str) -> String {
        match self {
            SoundLaw::GrimmsLaw => apply_grimms(input),
            SoundLaw::GreatVowelShift => apply_vowel_shift(input),
            SoundLaw::Lenition => apply_lenition(input),
            SoundLaw::LossOfEndings => apply_loss_of_endings(input),
            SoundLaw::Palatalization => apply_palatalization(input),
            SoundLaw::Metathesis => apply_metathesis(input),
            SoundLaw::Rhotacism => apply_rhotacism(input),
            SoundLaw::ClusterSimplification => apply_cluster_simplification(input),
            SoundLaw::HDropping => apply_h_dropping(input),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SoundLaw::GrimmsLaw => "Grimm's Law (Germanic Shift)",
            SoundLaw::GreatVowelShift => "Great Vowel Shift",
            SoundLaw::Lenition => "Intervocalic Lenition",
            SoundLaw::LossOfEndings => "Loss of Unstressed Endings",
            SoundLaw::Palatalization => "Palatalization (Softening)",
            SoundLaw::Metathesis => "Metathesis (Sound Swap)",
            SoundLaw::Rhotacism => "Rhotacism (s -> r)",
            SoundLaw::ClusterSimplification => "Cluster Simplification",
            SoundLaw::HDropping => "H-Dropping",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvolutionTrace {
    pub steps: Vec<(String, String)>, // (Rule applied, Resulting word)
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

    pub fn evolve_with_trace(&self, word: &str) -> (String, EvolutionTrace) {
        let mut current = word.to_string();
        let mut steps = vec![];

        for law in &self.laws {
            let next = law.apply(&current);
            if next != current {
                steps.push((law.description().to_string(), next.clone()));
                current = next;
            }
        }
        (current, EvolutionTrace { steps })
    }

    pub fn evolve(&self, word: &str) -> String {
        self.evolve_with_trace(word).0
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

fn apply_palatalization(input: &str) -> String {
    // k, g, t, d -> ch, j, ch, j before front vowels (i, e, y)
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next_is_front = if i + 1 < chars.len() {
            matches!(chars[i + 1], 'i' | 'e' | 'y' | 'I' | 'E' | 'Y')
        } else {
            false
        };

        if next_is_front {
            match c {
                'k' | 'K' => {
                    out.push_str(if c.is_uppercase() { "Ch" } else { "ch" });
                    i += 1;
                    continue;
                }
                'g' | 'G' => {
                    out.push(if c.is_uppercase() { 'J' } else { 'j' });
                    i += 1;
                    continue;
                }
                't' | 'T' => {
                    out.push_str(if c.is_uppercase() { "Ch" } else { "ch" });
                    i += 1;
                    continue;
                }
                'd' | 'D' => {
                    out.push(if c.is_uppercase() { 'J' } else { 'j' });
                    i += 1;
                    continue;
                }
                _ => out.push(c),
            }
        } else {
            out.push(c);
        }
        i += 1;
    }
    out
}

fn apply_metathesis(input: &str) -> String {
    // Swap r/l with adjacent vowel: bird -> brid, ask -> aks
    // Focusing on r-metathesis: V + r -> r + V (burn -> brun) or r + V -> V + r
    // Let's do a simple one: if we find 'r' followed by a vowel, 50% chance to swap,
    // or if we find vowel followed by 'r', swap.
    // To be deterministic (since we don't pass RNG here), let's just do it for specific patterns.
    // Let's swap 'r' + vowel if the vowel is 'i' or 'u'.  "run" -> "urn". "ring" -> "irng" (weird but okay).
    // Or Vowel + r -> r + Vowel. "burn" -> "brun".

    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if i + 1 < chars.len() {
            let next = chars[i + 1];
            // Swap 'r' + vowel -> vowel + 'r' (brid -> bird style)
            // But let's check for Vowel + r -> r + Vowel (burn -> brun)
            if is_vowel(c) && (next == 'r' || next == 'l') {
                out.push(next);
                out.push(c);
                i += 2;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

fn apply_rhotacism(input: &str) -> String {
    // s/z -> r between vowels
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();
    for i in 0..chars.len() {
        let c = chars[i];
        let is_intervocalic = if i > 0 && i < chars.len() - 1 {
            is_vowel(chars[i - 1]) && is_vowel(chars[i + 1])
        } else {
            false
        };

        if is_intervocalic && (c == 's' || c == 'z') {
            out.push('r');
        } else if is_intervocalic && (c == 'S' || c == 'Z') {
            out.push('R');
        } else {
            out.push(c);
        }
    }
    out
}

fn apply_cluster_simplification(input: &str) -> String {
    // kn -> n, gn -> n, wr -> r, mb -> m (at end)
    let s = input.to_string();
    let s = s.replace("kn", "n").replace("Kn", "N");
    let s = s.replace("gn", "n").replace("Gn", "N");
    let s = s.replace("wr", "r").replace("Wr", "R");

    // mb at end requires checking
    if s.ends_with("mb") {
        return s[..s.len() - 1].to_string();
    }
    s
}

fn apply_h_dropping(input: &str) -> String {
    // Drop 'h' at start of word
    if input.starts_with('h') {
        return input[1..].to_string();
    }
    if input.starts_with('H') {
        return input[1..].to_string();
    }
    input.to_string()
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}
