use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::visit::Visit;
use syn::{ItemEnum, ItemFn, ItemStruct};

#[derive(Debug, PartialEq, Clone)]
pub enum MusicalEvent {
    Note { freq: f32, duration: f32 },
    Chord { freqs: Vec<f32>, duration: f32 },
    Rest { duration: f32 },
}

pub struct CodeConcerto {
    pub events: Vec<MusicalEvent>,
}

impl CodeConcerto {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn generate_from_file(content: &str) -> Vec<MusicalEvent> {
        let syntax = syn::parse_file(content).expect("Unable to parse file");
        let mut concerto = CodeConcerto::new();
        concerto.visit_file(&syntax);
        concerto.events
    }

    fn hash_to_freq(&self, s: &str) -> f32 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let hash = hasher.finish();
        // Map hash to a frequency between 200 and 800 Hz (approx C3 to G5)
        // Use a pentatonic scale or something to sound nice?
        // For now, just linear mapping for "Wild Mode" chaos or simple quantization.
        let base = 220.0;
        let offset = (hash % 12) as f32; // Chromatic scale offset
        let octave = ((hash / 12) % 3) as f32;

        let semitones = offset + (octave * 12.0);
        base * 2.0_f32.powf(semitones / 12.0)
    }
}

impl<'ast> Visit<'ast> for CodeConcerto {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let root = self.hash_to_freq(&i.ident.to_string());
        let major_third = root * 2.0_f32.powf(4.0 / 12.0);
        let fifth = root * 2.0_f32.powf(7.0 / 12.0);

        self.events.push(MusicalEvent::Chord {
            freqs: vec![root, major_third, fifth],
            duration: 1.0,
        });

        // Visit fields to generate more fine-grained structure?
        // syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let root = self.hash_to_freq(&i.ident.to_string());
        let minor_third = root * 2.0_f32.powf(3.0 / 12.0);
        let fifth = root * 2.0_f32.powf(7.0 / 12.0);

        self.events.push(MusicalEvent::Chord {
            freqs: vec![root, minor_third, fifth],
            duration: 1.0,
        });
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let freq = self.hash_to_freq(&i.sig.ident.to_string());
        self.events.push(MusicalEvent::Note {
            freq,
            duration: 0.5,
        });

        // Recurse into body
        syn::visit::visit_item_fn(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_mapping() {
        let code = "struct MyStruct { field: i32 }";
        let events = CodeConcerto::generate_from_file(code);

        assert!(!events.is_empty());
        match &events[0] {
            MusicalEvent::Chord { freqs, .. } => {
                assert_eq!(freqs.len(), 3);
            }
            _ => panic!("Expected a Chord event for struct"),
        }
    }

    #[test]
    fn test_fn_mapping() {
        let code = "fn my_func() {}";
        let events = CodeConcerto::generate_from_file(code);

        assert!(!events.is_empty());
        match &events[0] {
            MusicalEvent::Note { .. } => {}
            _ => panic!("Expected a Note event for fn"),
        }
    }
}
