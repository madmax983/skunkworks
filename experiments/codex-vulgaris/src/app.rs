use crate::lexer::Token;
use logos::Logos;
use crate::phonology::{parse_identifier, evolve, RuleType};
use std::collections::HashMap;

pub struct App {
    pub source_code: String,
    pub evolved_code: String,
    pub evolved_map: Vec<(String, String)>,
    pub era: usize,
    pub should_quit: bool,
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        // Load its own source code as the example
        let source = include_str!("main.rs").to_string();
        let mut app = Self {
            source_code: source,
            evolved_code: String::new(),
            evolved_map: Vec::new(),
            era: 0,
            should_quit: false,
            show_help: false,
        };
        app.update_evolution();
        app
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn next_era(&mut self) {
        self.era += 1;
        self.update_evolution();
    }

    pub fn prev_era(&mut self) {
        if self.era > 0 {
            self.era -= 1;
            self.update_evolution();
        }
    }

    pub fn update_evolution(&mut self) {
        let mut lexer = Token::lexer(&self.source_code);
        let mut new_code = String::new();
        let mut last_end = 0;
        let mut map = HashMap::new();

        while let Some(token_res) = lexer.next() {
            let range = lexer.span();
            // Append skipped whitespace/comments
            new_code.push_str(&self.source_code[last_end..range.start]);

            match token_res {
                Ok(Token::Identifier(text)) => {
                    let evolved = self.evolve_identifier(&text);
                    new_code.push_str(&evolved);
                    map.insert(text, evolved);
                },
                Ok(Token::Unknown(_)) => {
                     // Just keep it as is
                     new_code.push_str(&self.source_code[range.clone()]);
                },
                _ => {
                    // Keywords, punctuation, numbers - keep as is
                    new_code.push_str(&self.source_code[range.clone()]);
                }
            }
            last_end = range.end;
        }
        // Append remaining text
        new_code.push_str(&self.source_code[last_end..]);
        self.evolved_code = new_code;

        // Update map for display
        let mut vec: Vec<_> = map.into_iter().collect();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        self.evolved_map = vec;
    }

    fn evolve_identifier(&self, text: &str) -> String {
        let mut word = parse_identifier(text);

        for i in 1..=self.era {
            let rule = match (i - 1) % 4 {
                0 => RuleType::Grimm,
                1 => RuleType::VowelShift,
                2 => RuleType::Lenition,
                3 => RuleType::Assimilation,
                _ => unreachable!(),
            };
            evolve(&mut word, rule);
        }

        word.to_string()
    }
}
