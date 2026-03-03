use crate::phonology::{Rule, Word};
use rand::RngCore;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Keyword,
    Symbol,
    Literal,
    Whitespace,
    Comment,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub content: String,
    pub token_type: TokenType,
}

pub struct Lexicon {
    pub tokens: Vec<Token>,
    pub evolved_identifiers: HashMap<String, Word>,
    #[allow(dead_code)]
    keywords: HashSet<String>,
}

impl Lexicon {
    pub fn new(code: &str) -> Self {
        let keywords_list = vec![
            "fn", "let", "mut", "pub", "struct", "enum", "impl", "use", "mod", "return", "if",
            "else", "match", "for", "while", "loop", "break", "continue", "as", "const", "static",
            "trait", "type", "unsafe", "where", "crate", "super", "self", "Self", "true", "false",
        ];
        let keywords: HashSet<String> = keywords_list.into_iter().map(|s| s.to_string()).collect();

        let tokens = Self::tokenize(code, &keywords);
        let mut evolved_identifiers = HashMap::new();

        for token in &tokens {
            if token.token_type == TokenType::Identifier
                && !evolved_identifiers.contains_key(&token.content) {
                    evolved_identifiers.insert(token.content.clone(), Word::new(&token.content));
                }
        }

        Self {
            tokens,
            evolved_identifiers,
            keywords,
        }
    }

    fn tokenize(input: &str, keywords: &HashSet<String>) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                let mut s = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_whitespace() {
                        s.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    content: s,
                    token_type: TokenType::Whitespace,
                });
            } else if c.is_alphabetic() || c == '_' {
                let mut s = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        s.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let token_type = if keywords.contains(&s) {
                    TokenType::Keyword
                } else {
                    TokenType::Identifier
                };
                tokens.push(Token {
                    content: s,
                    token_type,
                });
            } else if c == '"' {
                // String literal
                let mut s = String::new();
                s.push(chars.next().unwrap()); // consume "
                while let Some(&ch) = chars.peek() {
                    let ch_val = ch;
                    s.push(ch_val);
                    chars.next();
                    if ch_val == '"' && !s.ends_with("\\\"") {
                        break;
                    }
                }
                tokens.push(Token {
                    content: s,
                    token_type: TokenType::Literal,
                });
            } else if c.is_ascii_digit() {
                // Numeric literal
                let mut s = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                        s.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    content: s,
                    token_type: TokenType::Literal,
                });
            } else if c == '/' {
                // Check for comment
                chars.next(); // consume first /
                if let Some(&next_c) = chars.peek() {
                    if next_c == '/' {
                        // Line comment
                        let mut s = "/".to_string();
                        while let Some(&ch) = chars.peek() {
                            if ch != '\n' {
                                s.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token {
                            content: s,
                            token_type: TokenType::Comment,
                        });
                    } else {
                        tokens.push(Token {
                            content: "/".to_string(),
                            token_type: TokenType::Symbol,
                        });
                    }
                } else {
                    tokens.push(Token {
                        content: "/".to_string(),
                        token_type: TokenType::Symbol,
                    });
                }
            } else {
                // Symbol
                let s = c.to_string();
                chars.next();
                tokens.push(Token {
                    content: s,
                    token_type: TokenType::Symbol,
                });
            }
        }
        tokens
    }

    pub fn evolve(&mut self, rules: &[Box<dyn Rule>], rng: &mut dyn RngCore) {
        for word in self.evolved_identifiers.values_mut() {
            for rule in rules {
                rule.apply(word, rng);
            }
        }
    }

    pub fn render(&self) -> String {
        let mut s = String::new();
        for token in &self.tokens {
            match token.token_type {
                TokenType::Identifier => {
                    if let Some(word) = self.evolved_identifiers.get(&token.content) {
                        s.push_str(&word.to_string());
                    } else {
                        s.push_str(&token.content);
                    }
                }
                _ => s.push_str(&token.content),
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization_basic() {
        let code = "fn main() { let x = 10; }";
        let lexicon = Lexicon::new(code);

        let tokens: Vec<&str> = lexicon.tokens.iter().map(|t| t.content.as_str()).collect();
        // Expected: "fn", " ", "main", "(", ")", " ", "{", " ", "let", " ", "x", " ", "=", " ", "10", ";", " ", "}"
        // But spacing might be captured in Whitespace tokens.
        // My tokenizer captures whitespace as Token.
        // So "fn" -> " " -> "main" -> "(" -> ")" -> " " -> "{" ...
        assert_eq!(tokens[0], "fn");
        // tokens[1] is " "
        assert_eq!(tokens[2], "main");
        // tokens[3] is "("
        // tokens[4] is ")"
        // tokens[5] is " "
        // tokens[6] is "{"
        // tokens[7] is " "
        // tokens[8] is "let"
    }

    #[test]
    fn test_identifier_evolution() {
        let code = "fn my_func() {}";
        let lexicon = Lexicon::new(code);

        assert!(lexicon.evolved_identifiers.contains_key("my_func"));
        assert!(!lexicon.evolved_identifiers.contains_key("fn")); // Keyword
    }

    #[test]
    fn test_render_consistency() {
        let code = "let x = x + 1;";
        let lexicon = Lexicon::new(code);
        assert_eq!(lexicon.render(), code);
    }
}
