use regex::Regex;

#[derive(Clone, Debug)]
pub struct SoundChange {
    pub name: String,
    pub pattern: Regex,
    pub replacement: String,
}

pub struct PhonologyEngine {
    pub rules: Vec<SoundChange>,
}

impl PhonologyEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, name: &str, pattern: &str, replacement: &str) {
        self.rules.push(SoundChange {
            name: name.to_string(),
            pattern: Regex::new(pattern).expect("Invalid regex pattern"),
            replacement: replacement.to_string(),
        });
    }

    pub fn evolve(&self, word: &str) -> String {
        let mut result = word.to_string();
        for rule in &self.rules {
            result = rule.pattern.replace_all(&result, &rule.replacement).to_string();
        }
        result
    }

    pub fn grimms_law() -> Self {
        let mut engine = Self::new();
        // Voiceless stops -> Voiceless fricatives
        engine.add_rule("p->f", "p", "f");
        engine.add_rule("t->th", "t", "th");
        engine.add_rule("k->h", "k", "h");

        // Voiced stops -> Voiceless stops
        // (Use lookahead/behind to avoid chaining if possible, or just accept the chain for chaos)
        // Ideally: b->p, d->t, g->k.
        // But if p->f happened first, b->p->f?
        // Real sound changes happen over time.
        // If I run them sequentially:
        // p->f
        // b->p
        // So b ends up as p.
        engine.add_rule("b->p", "b", "p");
        engine.add_rule("d->t", "d", "t");
        engine.add_rule("g->k", "g", "k");

        // Voiced aspirated stops (bh, dh, gh) -> Voiced stops (b, d, g)
        // Rust identifiers don't have aspiration markers usually, but we can simulate it
        // if we assume 'h' indicates aspiration.
        engine.add_rule("bh->b", "bh", "b");
        engine.add_rule("dh->d", "dh", "d");
        engine.add_rule("gh->g", "gh", "g");

        engine
    }

    pub fn vowel_shift() -> Self {
        let mut engine = Self::new();
        // Great Vowel Shift (approximate)
        // a -> ei
        // e -> i
        // i -> ai
        // o -> ou
        // u -> au

        // Use temp markers to avoid double shifting?
        // Or just let it chain?
        // For a chaotic obfuscator, chaining is fine.
        // But to be "Philological", we should try to do it right.
        // Regex replacement is simultaneous for one rule, but sequential for list.

        // Let's use capital letters as temp if input is lowercase?
        // But input can be MixedCase.

        // I will just add rules that shift things around.
        engine.add_rule("a->ei", "a", "ei");
        engine.add_rule("e->i", "e", "i");
        engine.add_rule("i->ai", "i", "ai");
        engine.add_rule("o->ou", "o", "ou");
        engine.add_rule("u->au", "u", "au");

        engine
    }

    pub fn erosion() -> Self {
        let mut engine = Self::new();
        // Syncope: Loss of unstressed medial vowels.
        // Hard to know stress. Just kill some vowels.
        // Remove 'e' at end of words?
        engine.add_rule("final-e-drop", "e$", "");
        // Remove vowels between consonants?
        // cVc -> cc
        engine.add_rule("syncope", "([bcdfghjklmnpqrstvwxyz])[aeiou]([bcdfghjklmnpqrstvwxyz])", "${1}${2}");
        engine
    }
}
