use anyhow::{Context, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use ratatui::style::Color;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use syn::{visit::Visit, ItemFn};

use crate::audio::{Waveform, Adsr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextType {
    Global,
    Function,
    Loop,
    Condition,
    Block,
}

#[derive(Debug, Clone)]
pub struct MusicalToken {
    pub text: String,
    pub pitch: f32,    // Frequency in Hz
    pub duration: f32, // Duration in seconds
    pub velocity: f32, // 0.0 - 1.0
    pub color: Color,
    pub waveform: Waveform,
    pub adsr: Adsr,
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

    fn tokens_to_music(
        tokens: TokenStream,
        depth: usize,
        current_context: ContextType,
    ) -> Vec<MusicalToken> {
        let mut music = Vec::new();
        let mut tokens_iter = tokens.into_iter().peekable();

        // Simple state machine to guess context of next group
        let mut next_context_hint = None;

        while let Some(token) = tokens_iter.next() {
            // Default params
            let mut waveform = match current_context {
                ContextType::Loop => Waveform::Saw,
                ContextType::Condition => Waveform::Square,
                ContextType::Function => Waveform::Sine,
                _ => Waveform::Sine,
            };
            let mut adsr = Adsr::default();

            // Adjust ADSR based on context
            if current_context == ContextType::Loop {
                adsr.decay = 0.05; // Staccato
                adsr.release = 0.05;
            }

            match token {
                TokenTree::Ident(ident) => {
                    let text = ident.to_string();
                    let h = hash_string(&text);
                    let mut pitch = map_hash_to_pitch(h);

                    // Shift pitch by depth
                    pitch *= 1.0 + (depth as f32 * 0.1);

                    // Detect Keywords to set context hint
                    match text.as_str() {
                        "loop" | "for" | "while" => {
                            next_context_hint = Some(ContextType::Loop);
                            waveform = Waveform::Triangle;
                            adsr.attack = 0.05;
                        },
                        "if" | "match" | "else" => {
                            next_context_hint = Some(ContextType::Condition);
                            waveform = Waveform::Square;
                        },
                        "fn" => {
                            waveform = Waveform::Square; // Strong start
                        },
                        "let" | "mut" => {
                            waveform = Waveform::Saw; // Sharp
                            pitch *= 0.8;
                        }
                        _ => {}
                    }

                    music.push(MusicalToken {
                        text,
                        pitch,
                        duration: 0.15,
                        velocity: 0.8,
                        color: Color::Yellow,
                        waveform,
                        adsr,
                    });
                }
                TokenTree::Punct(punct) => {
                    let text = punct.to_string();
                    music.push(MusicalToken {
                        text,
                        pitch: 0.0, // Silence/Noise (handled in audio if needed, or 0 freq)
                        duration: 0.1,
                        velocity: 0.5,
                        color: Color::Cyan,
                        waveform: Waveform::Sine, // Silence anyway
                        adsr,
                    });
                }
                TokenTree::Literal(lit) => {
                    let text = lit.to_string();
                    let h = hash_string(&text);
                    let pitch = map_hash_to_pitch(h) * 0.5 * (1.0 + depth as f32 * 0.1);

                    music.push(MusicalToken {
                        text,
                        pitch,
                        duration: 0.3,
                        velocity: 0.6,
                        color: Color::Magenta,
                        waveform: Waveform::Triangle,
                        adsr,
                    });
                }
                TokenTree::Group(group) => {
                    let (open, close) = match group.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Brace => ("{", "}"),
                        Delimiter::Bracket => ("[", "]"),
                        Delimiter::None => ("", ""),
                    };

                    // Determine context for inner group
                    let inner_context = next_context_hint.take().unwrap_or(
                        if group.delimiter() == Delimiter::Brace {
                            ContextType::Block
                        } else {
                            current_context
                        }
                    );

                    if !open.is_empty() {
                        music.push(MusicalToken {
                            text: open.to_string(),
                            pitch: 600.0 + (depth as f32 * 50.0),
                            duration: 0.1,
                            velocity: 0.7,
                            color: Color::Green,
                            waveform: Waveform::Sine,
                            adsr,
                        });
                    }

                    let inner_music = CodeParser::tokens_to_music(
                        group.stream(),
                        depth + 1,
                        inner_context
                    );
                    music.extend(inner_music);

                    if !close.is_empty() {
                        music.push(MusicalToken {
                            text: close.to_string(),
                            pitch: 550.0 + (depth as f32 * 50.0),
                            duration: 0.1,
                            velocity: 0.7,
                            color: Color::Green,
                            waveform: Waveform::Sine,
                            adsr,
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
        let music = CodeParser::tokens_to_music(tokens, 0, ContextType::Function);

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
    // Pentatonic scale mapping
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
    fn test_context_detection() {
        let code = "fn loopy() { loop { let x = 1; } }";
        let voices = CodeParser::parse_str(code).unwrap();
        let tokens = &voices[0].tokens;

        // Find 'let' inside the loop
        // It should have Waveform::Saw (default for let) but also staccato ADSR (from loop context)
        // Wait, logic says:
        // if context == Loop { adsr.decay = 0.05 }

        let let_token = tokens.iter().find(|t| t.text == "let").unwrap();

        // Check if decay is staccato (0.05) vs default (0.1)
        assert_eq!(let_token.adsr.decay, 0.05);

        // Verify waveform
        // "let" sets Waveform::Saw explicitly
        // Loop context default is Saw
        match let_token.waveform {
            Waveform::Saw => {},
            _ => panic!("Expected Saw waveform for 'let'"),
        }
    }
}
