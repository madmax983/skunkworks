use anyhow::{Context, Result};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use ratatui::style::Color;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use syn::{visit::Visit, ItemEnum, ItemFn, ItemImpl, ItemStruct};

use crate::audio::{Adsr, Waveform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextType {
    Global,
    Function,
    Loop,
    Condition,
    Block,
    Struct,
    Enum,
    Impl,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoiceType {
    Soprano, // Functions (Main Melody)
    Alto,    // Impl Blocks (Counter-melody / Arpeggios)
    Tenor,   // Enums (Rhythmic Ostinato)
    Bass,    // Structs (Harmonic Foundation)
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
    pub voice_type: VoiceType,
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
        base_pitch_shift: f32,
    ) -> Vec<MusicalToken> {
        let mut music = Vec::new();
        let tokens_iter = tokens.into_iter().peekable();

        // Simple state machine to guess context of next group
        let mut next_context_hint = None;

        for token in tokens_iter {
            // Default params based on context
            let (mut waveform, mut duration_mod, velocity) = match current_context {
                ContextType::Loop => (Waveform::Saw, 0.5, 0.9),
                ContextType::Condition => (Waveform::Square, 0.7, 0.8),
                ContextType::Function => (Waveform::Sine, 1.0, 0.8),
                ContextType::Struct => (Waveform::Sine, 4.0, 0.6), // Long, soft notes
                ContextType::Enum => (Waveform::Square, 0.2, 0.7), // Fast, rhythmic
                ContextType::Impl => (Waveform::Triangle, 1.0, 0.7),
                _ => (Waveform::Sine, 1.0, 0.5),
            };

            let mut adsr = Adsr::default();

            // Adjust ADSR based on context
            match current_context {
                ContextType::Loop => {
                    adsr.decay = 0.05; // Staccato
                    adsr.release = 0.05;
                }
                ContextType::Struct => {
                    adsr.attack = 0.5; // Slow attack
                    adsr.decay = 0.5;
                    adsr.release = 1.0; // Long release
                    adsr.sustain_level = 0.9;
                }
                ContextType::Enum => {
                    adsr.attack = 0.01;
                    adsr.decay = 0.1;
                    adsr.release = 0.0; // Very short
                }
                _ => {}
            }

            match token {
                TokenTree::Ident(ident) => {
                    let text = ident.to_string();
                    let h = hash_string(&text);
                    let mut pitch = map_hash_to_pitch(h);

                    // Apply context pitch shift (e.g., Bass is lower)
                    pitch *= base_pitch_shift;

                    // Shift pitch by depth
                    pitch *= 1.0 + (depth as f32 * 0.05);

                    // Detect Keywords
                    match text.as_str() {
                        "loop" | "for" | "while" => {
                            next_context_hint = Some(ContextType::Loop);
                            waveform = Waveform::Triangle;
                        }
                        "if" | "match" | "else" => {
                            next_context_hint = Some(ContextType::Condition);
                            waveform = Waveform::Square;
                        }
                        "pub" | "fn" | "struct" | "enum" | "impl" => {
                            // Structural keywords
                            duration_mod *= 1.5;
                        }
                        _ => {}
                    }

                    music.push(MusicalToken {
                        text,
                        pitch,
                        duration: 0.2 * duration_mod,
                        velocity,
                        color: match current_context {
                            ContextType::Struct => Color::Blue,
                            ContextType::Enum => Color::Red,
                            ContextType::Impl => Color::Magenta,
                            _ => Color::Yellow,
                        },
                        waveform,
                        adsr,
                    });
                }
                TokenTree::Punct(punct) => {
                    let text = punct.to_string();
                    // Punctuation is percussive or silent logic
                    music.push(MusicalToken {
                        text,
                        pitch: 0.0,
                        duration: 0.1 * duration_mod,
                        velocity: 0.4,
                        color: Color::Cyan,
                        waveform: Waveform::Sine,
                        adsr,
                    });
                }
                TokenTree::Literal(lit) => {
                    let text = lit.to_string();
                    let h = hash_string(&text);
                    let pitch = map_hash_to_pitch(h) * base_pitch_shift;

                    music.push(MusicalToken {
                        text,
                        pitch,
                        duration: 0.3 * duration_mod,
                        velocity: 0.6,
                        color: Color::Green,
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

                    let inner_context = next_context_hint.take().unwrap_or(
                        match (current_context, group.delimiter()) {
                            (_, Delimiter::Brace) => ContextType::Block,
                            _ => current_context,
                        },
                    );

                    // Propagate context for special types like Struct/Enum
                    let effective_context = match current_context {
                        ContextType::Struct | ContextType::Enum | ContextType::Impl => {
                            current_context
                        }
                        _ => inner_context,
                    };

                    if !open.is_empty() {
                        music.push(MusicalToken {
                            text: open.to_string(),
                            pitch: 600.0 * base_pitch_shift,
                            duration: 0.1,
                            velocity: 0.5,
                            color: Color::White,
                            waveform: Waveform::Sine,
                            adsr,
                        });
                    }

                    let inner_music = CodeParser::tokens_to_music(
                        group.stream(),
                        depth + 1,
                        effective_context,
                        base_pitch_shift,
                    );
                    music.extend(inner_music);

                    if !close.is_empty() {
                        music.push(MusicalToken {
                            text: close.to_string(),
                            pitch: 550.0 * base_pitch_shift,
                            duration: 0.1,
                            velocity: 0.5,
                            color: Color::White,
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
        // Soprano: Normal pitch (1.0)
        let music = CodeParser::tokens_to_music(tokens, 0, ContextType::Function, 1.0);

        self.voices.push(Voice {
            name,
            voice_type: VoiceType::Soprano,
            tokens: music,
        });
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let name = node.ident.to_string();
        let tokens = node.to_token_stream();
        // Bass: Low pitch (0.5 = octave down)
        let music = CodeParser::tokens_to_music(tokens, 0, ContextType::Struct, 0.5);

        self.voices.push(Voice {
            name,
            voice_type: VoiceType::Bass,
            tokens: music,
        });
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let name = node.ident.to_string();
        let tokens = node.to_token_stream();
        // Tenor: Slightly lower (0.75)
        let music = CodeParser::tokens_to_music(tokens, 0, ContextType::Enum, 0.75);

        self.voices.push(Voice {
            name,
            voice_type: VoiceType::Tenor,
            tokens: music,
        });
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let name = if let Some((_, trait_path, _)) = &node.trait_ {
            trait_path.to_token_stream().to_string()
        } else {
            node.self_ty.to_token_stream().to_string()
        };

        let tokens = node.to_token_stream();
        // Alto: Slight shift (0.8 or 1.2?) Let's go 0.8
        let music = CodeParser::tokens_to_music(tokens, 0, ContextType::Impl, 0.8);

        self.voices.push(Voice {
            name: format!("impl {}", name),
            voice_type: VoiceType::Alto,
            tokens: music,
        });
    }
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn map_hash_to_pitch(hash: u64) -> f32 {
    // Pentatonic scale mapping (C Major Pentatonic)
    // C4, D4, E4, G4, A4
    let scale = [
        261.63, 293.66, 329.63, 392.00, 440.00, 523.25, 587.33, 659.25, 783.99, 880.00,
    ];
    let idx = (hash as usize) % scale.len();
    scale[idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_struct() {
        let code = "struct Point { x: i32, y: i32 }";
        let voices = CodeParser::parse_str(code).unwrap();
        assert_eq!(voices.len(), 1);
        assert_eq!(voices[0].name, "Point");
        assert_eq!(voices[0].voice_type, VoiceType::Bass);

        // Struct should have long notes (sine)
        let tokens = &voices[0].tokens;
        let ident_token = tokens.iter().find(|t| t.text == "Point").unwrap();

        match ident_token.waveform {
            Waveform::Sine => {} // Struct context defaults to Sine
            _ => panic!("Expected Sine for Struct"),
        }
    }

    #[test]
    fn test_parse_enum() {
        let code = "enum Color { Red, Green }";
        let voices = CodeParser::parse_str(code).unwrap();
        assert_eq!(voices.len(), 1);
        assert_eq!(voices[0].voice_type, VoiceType::Tenor);
    }

    #[test]
    fn test_parse_mixed() {
        let code = r#"
            struct Data { val: i32 }
            fn process() {}
        "#;
        let voices = CodeParser::parse_str(code).unwrap();
        assert_eq!(voices.len(), 2);
        // Order depends on visit traversal, usually sequential
        assert!(voices.iter().any(|v| v.voice_type == VoiceType::Bass));
        assert!(voices.iter().any(|v| v.voice_type == VoiceType::Soprano));
    }
}
