use crate::model::{CodeConcerto, Instrument, MusicalEvent, Section, SectionKind};
use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use syn::{visit::Visit, ItemEnum, ItemFn, ItemImpl, ItemStruct};

pub struct Parser {
    concerto: CodeConcerto,
    current_depth: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            concerto: CodeConcerto::default(),
            current_depth: 0,
        }
    }

    pub fn parse_file(path: &str) -> Result<CodeConcerto> {
        let content = fs::read_to_string(path)?;
        let syntax = syn::parse_file(&content)?;
        let mut parser = Parser::new();
        parser.visit_file(&syntax);
        Ok(parser.concerto)
    }

    // Pentatonic Major Scale: Root, M2, M3, P5, M6
    fn hash_to_scale_degree(&self, s: &str, octave: i32) -> f32 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let h = hasher.finish();

        let degrees = [1.0, 9.0/8.0, 5.0/4.0, 3.0/2.0, 5.0/3.0];
        let degree = degrees[(h as usize) % degrees.len()];

        let base_c4 = 261.63;
        let mult = if octave >= 0 {
            2.0f32.powi(octave)
        } else {
            1.0 / 2.0f32.powi(-octave)
        };

        base_c4 * mult * degree
    }
}

impl<'ast> Visit<'ast> for Parser {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let name = i.ident.to_string();
        let field_count = i.fields.len();

        let base_freq = self.hash_to_scale_degree(&name, -1); // Lower octave for structs (pads)
        let duration = Duration::from_millis(800 + (field_count as u64 * 200));

        let events = vec![
            MusicalEvent::NoteOn {
                instrument: Instrument::Pad,
                pitch: base_freq,
                volume: 0.5,
                duration,
            },
            MusicalEvent::NoteOn {
                instrument: Instrument::Pad,
                pitch: base_freq * 1.25,
                volume: 0.4,
                duration,
            },
            MusicalEvent::NoteOn {
                instrument: Instrument::Pad,
                pitch: base_freq * 1.5,
                volume: 0.4,
                duration,
            },
            MusicalEvent::Wait(duration),
        ];

        self.concerto.add_section(Section {
            name: name.clone(),
            kind: SectionKind::Struct,
            depth: self.current_depth,
            events,
        });

        self.current_depth += 1;
        syn::visit::visit_item_struct(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let name = i.ident.to_string();
        let variant_count = i.variants.len();

        let base_freq = self.hash_to_scale_degree(&name, -1);
        let duration = Duration::from_millis(800 + (variant_count as u64 * 200));

        let events = vec![
            MusicalEvent::NoteOn {
                instrument: Instrument::Pad,
                pitch: base_freq,
                volume: 0.5,
                duration,
            },
            MusicalEvent::NoteOn {
                instrument: Instrument::Pad,
                pitch: base_freq * 1.2,
                volume: 0.4,
                duration,
            },
            MusicalEvent::NoteOn {
                instrument: Instrument::Pad,
                pitch: base_freq * 1.5,
                volume: 0.4,
                duration,
            },
            MusicalEvent::Wait(duration),
        ];

        self.concerto.add_section(Section {
            name: name.clone(),
            kind: SectionKind::Enum,
            depth: self.current_depth,
            events,
        });

        self.current_depth += 1;
        syn::visit::visit_item_enum(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let name = i.sig.ident.to_string();
        let stmt_count = i.block.stmts.len();

        let mut events = Vec::new();
        let base_freq = self.hash_to_scale_degree(&name, 0); // Middle octave

        // Generate melody from statements
        let note_duration = Duration::from_millis(150);

        for (idx, _stmt) in i.block.stmts.iter().enumerate() {
            let pitch_mod = match idx % 4 {
                0 => 1.0,       // Root
                1 => 9.0/8.0,   // 2nd
                2 => 5.0/4.0,   // 3rd
                3 => 3.0/2.0,   // 5th
                _ => 1.0,
            };

            let pitch = base_freq * pitch_mod;

            events.push(MusicalEvent::NoteOn {
                instrument: Instrument::Lead,
                pitch,
                volume: 0.6,
                duration: note_duration,
            });
            events.push(MusicalEvent::Wait(note_duration));
        }

        if stmt_count == 0 {
             events.push(MusicalEvent::NoteOn {
                instrument: Instrument::Lead,
                pitch: base_freq,
                volume: 0.6,
                duration: note_duration * 2,
            });
            events.push(MusicalEvent::Wait(note_duration * 2));
        }

        self.concerto.add_section(Section {
            name: name.clone(),
            kind: SectionKind::Function,
            depth: self.current_depth,
            events,
        });

        self.current_depth += 1;
        syn::visit::visit_item_fn(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        let name = if let Some((_, path, _)) = &i.trait_ {
            path.segments.last().map(|s| s.ident.to_string()).unwrap_or_else(|| "Trait".to_string())
        } else if let syn::Type::Path(p) = &*i.self_ty {
             p.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_else(|| "Type".to_string())
        } else {
            "Impl".to_string()
        };

        let item_count = i.items.len();
        let duration = Duration::from_millis(400);

        let base_freq = self.hash_to_scale_degree(&name, -2); // Bass octave

        let events = vec![
            MusicalEvent::NoteOn {
                instrument: Instrument::Bass,
                pitch: base_freq,
                volume: 0.7,
                duration: duration * (item_count as u32 + 2),
            },
            MusicalEvent::Wait(duration),
        ];

        self.concerto.add_section(Section {
            name: format!("impl {}", name),
            kind: SectionKind::Impl,
            depth: self.current_depth,
            events,
        });

        self.current_depth += 1;
        syn::visit::visit_item_impl(self, i);
        self.current_depth -= 1;
    }
}
