use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[cfg(feature = "nova")]
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{ExprLoop, File, ItemFn, ItemStruct, Local};

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeSpan {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Timbre {
    Sine,
    Triangle,
    Square,
    Sawtooth,
}

#[derive(Debug, Clone)]
pub struct MusicalEvent {
    pub frequency: f32,
    pub duration: f32,
    pub timbre: Timbre,
    pub description: String,
    #[cfg(feature = "nova")]
    pub span: Option<CodeSpan>,
}

pub struct CodeParser {
    pub events: Vec<MusicalEvent>,
    pub current_depth: usize,
}

impl CodeParser {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            current_depth: 0,
        }
    }

    pub fn parse(&mut self, source: &str) -> anyhow::Result<()> {
        let syntax: File = syn::parse_str(source)?;
        self.visit_file(&syntax);
        Ok(())
    }

    fn generate_freq(&self, input: &str) -> f32 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let hash = hasher.finish();

        // Map hash to a pentatonic scale (C major pentatonic: C, D, E, G, A)
        // MIDI notes: 60, 62, 64, 67, 69
        let scale = [
            261.63, 293.66, 329.63, 392.00, 440.00, 523.25, 587.33, 659.25, 783.99, 880.00,
        ];
        let index = (hash as usize) % scale.len();
        scale[index]
    }
}

impl<'ast> Visit<'ast> for CodeParser {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let name = i.sig.ident.to_string();
        let freq = self.generate_freq(&name);

        #[cfg(feature = "nova")]
        let span = {
            let s = i.sig.span();
            Some(CodeSpan {
                start_line: s.start().line,
                start_col: s.start().column,
                end_line: s.end().line,
                end_col: s.end().column,
            })
        };

        self.events.push(MusicalEvent {
            frequency: freq,
            duration: 0.5,
            timbre: Timbre::Sine,
            description: format!("Function: {}", name),
            #[cfg(feature = "nova")]
            span,
        });

        self.current_depth += 1;
        visit::visit_item_fn(self, i);
        self.current_depth -= 1;
    }

    fn visit_local(&mut self, i: &'ast Local) {
        #[cfg(feature = "nova")]
        let span = {
            let s = i.pat.span();
            Some(CodeSpan {
                start_line: s.start().line,
                start_col: s.start().column,
                end_line: s.end().line,
                end_col: s.end().column,
            })
        };

        self.events.push(MusicalEvent {
            frequency: 880.0, // High pitch for variable declaration
            duration: 0.1,
            timbre: Timbre::Triangle,
            description: "Let Binding".to_string(),
            #[cfg(feature = "nova")]
            span,
        });
        visit::visit_local(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let name = i.ident.to_string();
        let freq = self.generate_freq(&name) / 2.0; // Lower octave

        #[cfg(feature = "nova")]
        let span = {
            let s = i.ident.span();
            Some(CodeSpan {
                start_line: s.start().line,
                start_col: s.start().column,
                end_line: s.end().line,
                end_col: s.end().column,
            })
        };

        self.events.push(MusicalEvent {
            frequency: freq,
            duration: 1.0,
            timbre: Timbre::Square,
            description: format!("Struct: {}", name),
            #[cfg(feature = "nova")]
            span,
        });
        visit::visit_item_struct(self, i);
    }

    // Add loops for rhythm
    fn visit_expr_loop(&mut self, i: &'ast ExprLoop) {
        #[cfg(feature = "nova")]
        let span = {
            let s = i.loop_token.span();
            Some(CodeSpan {
                start_line: s.start().line,
                start_col: s.start().column,
                end_line: s.end().line,
                end_col: s.end().column,
            })
        };

        self.events.push(MusicalEvent {
            frequency: 100.0,
            duration: 0.2,
            timbre: Timbre::Sawtooth,
            description: "Loop".to_string(),
            #[cfg(feature = "nova")]
            span,
        });
        visit::visit_expr_loop(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let code = r#"
            fn main() {
                let x = 5;
            }
        "#;

        let mut parser = CodeParser::new();
        parser.parse(code).unwrap();

        assert!(!parser.events.is_empty(), "Should generate events");

        let fn_event = parser
            .events
            .iter()
            .find(|e| e.description.contains("Function"));
        assert!(fn_event.is_some(), "Should detect function");

        let let_event = parser
            .events
            .iter()
            .find(|e| e.description.contains("Let Binding"));
        assert!(let_event.is_some(), "Should detect let binding");
    }

    #[test]
    fn test_parse_struct() {
        let code = r#"
            struct MyData {
                x: i32,
            }
        "#;
        let mut parser = CodeParser::new();
        parser.parse(code).unwrap();

        let struct_event = parser
            .events
            .iter()
            .find(|e| e.description.contains("Struct"));
        assert!(struct_event.is_some());
        assert_eq!(struct_event.unwrap().timbre, Timbre::Square);
    }
}
