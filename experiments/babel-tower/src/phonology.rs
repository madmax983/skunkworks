use regex::Regex;

#[derive(Clone, Debug)]
pub struct SoundChange {
    pub pattern: Regex,
    pub replacement: String,
    #[allow(dead_code)]
    pub description: String,
}

pub struct PhoneticEngine {
    pub rules: Vec<SoundChange>,
}

impl PhoneticEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, pattern: &str, replacement: &str, description: &str) {
        let pattern = Regex::new(pattern).expect("Invalid regex pattern");
        self.rules.push(SoundChange {
            pattern,
            replacement: replacement.to_string(),
            description: description.to_string(),
        });
    }

    pub fn apply(&self, input: &str) -> String {
        let mut result = input.to_string();
        for rule in &self.rules {
            result = rule.pattern.replace_all(&result, &rule.replacement).to_string();
        }
        result
    }

    /// Grimm's Law: First Germanic Sound Shift
    /// Note: This is an approximation applied to orthography.
    pub fn grimm() -> Self {
        let mut engine = Self::new();

        // 1. Voiceless stops -> Voiceless fricatives
        // p -> f
        // t -> θ (th)
        // k -> h (or x)
        engine.add_rule(r"p", "f", "p -> f");
        engine.add_rule(r"t", "th", "t -> th");
        engine.add_rule(r"k", "h", "k -> h");

        // 2. Voiced stops -> Voiceless stops
        // b -> p
        // d -> t
        // g -> k
        engine.add_rule(r"b", "p", "b -> p");
        engine.add_rule(r"d", "t", "d -> t");
        engine.add_rule(r"g", "k", "g -> k");

        // 3. Voiced aspirated stops -> Voiced stops
        // bh -> b
        // dh -> d
        // gh -> g
        // Since we are processing ASCII code, we likely don't have 'bh',
        // but if we did (e.g. from a previous pass or weird variable naming), we'd handle it.
        // We'll skip this for now as it's rare in Rust code.

        engine
    }

    /// Great Vowel Shift (English, approx. 1350-1700)
    /// A chain shift affecting long vowels.
    pub fn great_vowel_shift() -> Self {
        let mut engine = Self::new();
        // Since we can't easily distinguish long/short vowels in spelling,
        // we'll apply it to ALL vowels for maximum obfuscation.

        // i -> ai (bite)
        // u -> au (out)
        // e -> i (meet)
        // o -> u (boot)
        // a -> ei (make)

        // To prevent immediate re-application (chain reaction in one step),
        // we use temporary placeholders.

        engine.add_rule(r"i", "##AI##", "i -> AI (temp)");
        engine.add_rule(r"u", "##AU##", "u -> AU (temp)");

        engine.add_rule(r"e", "i", "e -> i");
        engine.add_rule(r"o", "u", "o -> u");

        engine.add_rule(r"a", "ei", "a -> ei");

        engine.add_rule(r"##AI##", "ai", "Restore AI");
        engine.add_rule(r"##AU##", "au", "Restore AU");

        engine
    }

    /// Rhotacism: s -> r between vowels
    pub fn rhotacism() -> Self {
        let mut engine = Self::new();
        // Vowel regex: [aeiouy]
        engine.add_rule(r"([aeiouy])s([aeiouy])", "${1}r${2}", "s -> r / V_V");
        engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grimm_pater() {
        let engine = PhoneticEngine::grimm();
        // pater -> fater (p->f) -> father (t->th)
        assert_eq!(engine.apply("pater"), "father");
    }

    #[test]
    fn test_grimm_chain() {
        let engine = PhoneticEngine::grimm();
        // d -> t -> th
        // dental -> den-th-al (t->th) -> ten-th-al (d->t)
        // Grimm's law is a push chain: t->th happens first, so new t's from d don't shift further.
        assert_eq!(engine.apply("dental"), "tenthal");
    }

    #[test]
    fn test_vowel_shift() {
        let engine = PhoneticEngine::great_vowel_shift();
        // bite -> b-ai-t-i (e -> i)
        assert_eq!(engine.apply("bite"), "baiti");
        // meet -> mit
        assert_eq!(engine.apply("meet"), "miit"); // ee -> ii
        // make -> meik
        assert_eq!(engine.apply("make"), "meiki"); // a->ei, e->i. Wait: make -> m ei k i
    }

    #[test]
    fn test_rhotacism() {
        let engine = PhoneticEngine::rhotacism();
        // honos -> honor (Latin)
        // rust: virus -> virus (no change if not flanked?)
        // let's try 'asa' -> 'ara'
        assert_eq!(engine.apply("asa"), "ara");
        assert_eq!(engine.apply("virus"), "virus"); // u is vowel, but s is final.
        assert_eq!(engine.apply("visus"), "virus");
    }
}
