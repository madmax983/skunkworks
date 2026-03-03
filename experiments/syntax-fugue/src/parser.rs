use anyhow::{Context, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use ratatui::style::Color;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use syn::{visit::Visit, ItemFn};

#[derive(Debug, Clone)]
pub struct MusicalToken {
    pub text: String,
    pub pitch: f32,    // Frequency in Hz
    pub duration: f32, // Duration in seconds
    pub velocity: f32, // 0.0 - 1.0
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Voice {
    pub name: String,
    pub tokens: Vec<MusicalToken>,
}

#[derive(Default)]
pub struct CodeParser {
    voices: Vec<Voice>,
}

impl CodeParser {
    pub fn new() -> Self {
        Self { voices: Vec::new() }
    }

    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Vec<Voice>> {
        let content = fs::read_to_string(path).context("Failed to read file")?;
        Self::parse_str(&content)
    }

    pub fn parse_str(content: &str) -> Result<Vec<Voice>> {
        let syntax = syn::parse_file(content).context("Failed to parse file")?;
        let mut parser = CodeParser::new();
        parser.visit_file(&syntax);
        Ok(parser.voices)
    }

    fn tokens_to_music(tokens: TokenStream) -> Vec<MusicalToken> {
        let mut music = Vec::new();
        for token in tokens {
            match token {
                TokenTree::Ident(ident) => {
                    let text = ident.to_string();
                    let h = hash_string(&text);
                    let pitch = map_hash_to_pitch(h);
                    music.push(MusicalToken {
                        text,
                        pitch,
                        duration: 0.15,
                        velocity: 0.8,
                        color: Color::Yellow,
                    });
                }
                TokenTree::Punct(punct) => {
                    let text = punct.to_string();
                    music.push(MusicalToken {
                        text,
                        pitch: 0.0, // Silence or noise
                        duration: 0.1,
                        velocity: 0.5,
                        color: Color::Cyan,
                    });
                }
                TokenTree::Literal(lit) => {
                    let text = lit.to_string();
                    let h = hash_string(&text);
                    // Shift pitch down for literals
                    let pitch = map_hash_to_pitch(h) * 0.5;
                    music.push(MusicalToken {
                        text,
                        pitch,
                        duration: 0.3,
                        velocity: 0.6,
                        color: Color::Magenta,
                    });
                }
                TokenTree::Group(group) => {
                    let (open, close) = match group.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Brace => ("{", "}"),
                        Delimiter::Bracket => ("[", "]"),
                        Delimiter::None => ("", ""),
                    };

                    if !open.is_empty() {
                        music.push(MusicalToken {
                            text: open.to_string(),
                            pitch: 600.0, // High pitch for open delimiter
                            duration: 0.1,
                            velocity: 0.7,
                            color: Color::Green,
                        });
                    }

                    let inner_music = CodeParser::tokens_to_music(group.stream());
                    music.extend(inner_music);

                    if !close.is_empty() {
                        music.push(MusicalToken {
                            text: close.to_string(),
                            pitch: 550.0, // Slightly lower for close
                            duration: 0.1,
                            velocity: 0.7,
                            color: Color::Green,
                        });
                    }
                }
            }
        }
        music
    }
}

impl<'ast> Visit<'ast> for CodeParser {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();
        let tokens = node.to_token_stream();
        let music = CodeParser::tokens_to_music(tokens);

        self.voices.push(Voice {
            name,
            tokens: music,
        });

        // Don't recurse into nested items to avoid duplication
    }
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn map_hash_to_pitch(hash: u64) -> f32 {
    // Pentatonic scale mapping would be nicer musically
    // C Major Pentatonic: C, D, E, G, A
    // Frequencies (approx): 261, 293, 329, 392, 440
    let scale = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ]; // C Major Scale
    let idx = (hash as usize) % scale.len();
    let octave_shift = ((hash / 100) % 3) as f32; // 0, 1, 2 octaves up
    scale[idx] * 2.0_f32.powf(octave_shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let code = "fn hello() { let x = 5; }";
        let voices = CodeParser::parse_str(code).unwrap();
        assert_eq!(voices.len(), 1);
        assert_eq!(voices[0].name, "hello");

        let tokens = &voices[0].tokens;
        assert!(!tokens.is_empty());

        let has_hello = tokens.iter().any(|t| t.text == "hello");
        assert!(has_hello);
    }

    #[test]
    fn test_parse_multiple_functions() {
        let code = "
            fn a() {}
            fn b() {}
        ";
        let voices = CodeParser::parse_str(code).unwrap();
        assert_eq!(voices.len(), 2);
        assert_eq!(voices[0].name, "a");
        assert_eq!(voices[1].name, "b");
    }
}
