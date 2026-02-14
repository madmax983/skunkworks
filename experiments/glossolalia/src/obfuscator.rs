use crate::lexer::{tokenize, Token};
use crate::phonology::EvolutionEngine;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;

pub struct Obfuscator {
    engine: EvolutionEngine,
    dictionary: HashMap<String, String>,
    rng: StdRng,
}

impl Obfuscator {
    pub fn new(seed: u64) -> Self {
        Self {
            engine: EvolutionEngine::new(),
            dictionary: HashMap::new(),
            rng: StdRng::seed_from_u64(seed),
        }
    }

    #[allow(dead_code)]
    pub fn evolve_word(&mut self, word: &str) -> String {
        if let Some(evolved) = self.dictionary.get(word) {
            return evolved.clone();
        }

        let evolved = self.engine.evolve_word(word, &mut self.rng);
        self.dictionary.insert(word.to_string(), evolved.clone());
        evolved
    }

    pub fn transmute(&mut self, code: &str) -> String {
        let tokens = tokenize(code);
        let mut output = String::new();
        let mut last_end = 0;

        for (token, span) in tokens {
            // Append whitespace/skipped characters between tokens
            if span.start > last_end {
                output.push_str(&code[last_end..span.start]);
            }

            match token {
                Token::Identifier(s) => {
                    output.push_str(&self.evolved_or_init(&s));
                }
                Token::KwFn => output.push_str(&self.evolved_or_init("fn")),
                Token::KwLet => output.push_str(&self.evolved_or_init("let")),
                Token::KwMut => output.push_str(&self.evolved_or_init("mut")),
                Token::KwPub => output.push_str(&self.evolved_or_init("pub")),
                Token::KwStruct => output.push_str(&self.evolved_or_init("struct")),
                Token::KwEnum => output.push_str(&self.evolved_or_init("enum")),
                Token::KwImpl => output.push_str(&self.evolved_or_init("impl")),
                Token::KwUse => output.push_str(&self.evolved_or_init("use")),
                Token::KwMod => output.push_str(&self.evolved_or_init("mod")),
                Token::KwReturn => output.push_str(&self.evolved_or_init("return")),
                Token::KwIf => output.push_str(&self.evolved_or_init("if")),
                Token::KwElse => output.push_str(&self.evolved_or_init("else")),
                Token::KwMatch => output.push_str(&self.evolved_or_init("match")),
                Token::KwFor => output.push_str(&self.evolved_or_init("for")),
                Token::KwWhile => output.push_str(&self.evolved_or_init("while")),
                Token::KwLoop => output.push_str(&self.evolved_or_init("loop")),

                // Literals and Punctuation are kept as is
                Token::StringLiteral(s) => output.push_str(&format!("\"{}\"", s)), // tokenize strips quotes? No, regex keeps them.
                // Wait, my regex `r#""([^"\\]|\\.)*""#` captures the quotes too.
                // So `s` includes quotes.
                // But `Identifier(String)` regex uses `lex.slice().to_string()`.

                // Let's check lexer.rs again.
                // Identifier uses slice().to_string().
                // StringLiteral uses slice().to_string().

                // So I can just use the original slice from the span!
                // Except for Identifier and Keywords which I want to replace.
                _ => {
                    output.push_str(&code[span.clone()]);
                }
            }
            last_end = span.end;
        }

        // Append remaining whitespace
        if last_end < code.len() {
            output.push_str(&code[last_end..]);
        }

        output
    }

    fn evolved_or_init(&mut self, word: &str) -> String {
        if let Some(s) = self.dictionary.get(word) {
            s.clone()
        } else {
            // First time seeing this word, it starts as itself
            // Wait, if I call transmute multiple times, I want it to evolve progressively?
            // Or does `Obfuscator` represent a specific "Dialect"?

            // If `Obfuscator` represents the process of evolution over time,
            // then `transmute` should take the current state and evolve it.

            // BUT, `dictionary` maps Original -> Current.
            // If I want to evolve further, I need to update the dictionary values.

            self.dictionary.insert(word.to_string(), word.to_string());
            word.to_string()
        }
    }

    pub fn advance_generation(&mut self) {
        // Iterate over all entries in the dictionary and evolve them one step
        // We need to collect keys first to avoid borrow issues
        let keys: Vec<String> = self.dictionary.keys().cloned().collect();
        for key in keys {
            if let Some(current_val) = self.dictionary.get(&key) {
                let next_val = self.engine.evolve_word(current_val, &mut self.rng);
                self.dictionary.insert(key, next_val);
            }
        }
    }
}
