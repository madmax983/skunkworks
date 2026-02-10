use crate::music::{FugueEvent, Mode, Note};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use syn::{visit, ExprIf, ExprLoop, ExprMatch, ExprWhile, ItemFn};

pub struct SyntaxListener {
    pub events: Vec<FugueEvent>,
    _rng: StdRng,
    current_voice: usize,
}

impl SyntaxListener {
    pub fn new(seed: u64) -> Self {
        Self {
            events: Vec::new(),
            _rng: StdRng::seed_from_u64(seed),
            current_voice: 0,
        }
    }

    fn generate_theme(&mut self, name: &str, length: usize) -> Vec<Note> {
        let mut notes = Vec::new();
        // Use name to seed a local RNG for consistent themes
        let seed = name.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        let mut theme_rng = StdRng::seed_from_u64(seed);

        let base_pitch = 60 + (theme_rng.gen::<u8>() % 12); // Middle C + octave

        for _ in 0..length {
            let offset = theme_rng.gen::<i8>() % 7; // Diatonic-ish steps
            let pitch = (base_pitch as i8 + offset * 2) as u8;
            notes.push(Note {
                pitch,
                velocity: 90 + (theme_rng.gen::<u8>() % 30),
                duration_ms: 250, // 16th notes at 120bpm approx
            });
        }
        notes
    }
}

impl<'ast> visit::Visit<'ast> for SyntaxListener {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();
        let notes = self.generate_theme(&name, 8);
        self.events.push(FugueEvent::SubjectEntry {
            name: name.clone(),
            voice_id: self.current_voice,
            notes,
        });

        // Enter function body
        self.current_voice += 1;
        visit::visit_item_fn(self, node);
        self.current_voice -= 1;

        self.events.push(FugueEvent::Silence { duration_ms: 500 });
    }

    fn visit_expr_loop(&mut self, node: &'ast ExprLoop) {
        let notes = self.generate_theme("loop", 4);
        self.events.push(FugueEvent::Ostinato {
            voice_id: self.current_voice,
            pattern: notes,
        });
        visit::visit_expr_loop(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast ExprWhile) {
        let notes = self.generate_theme("while", 4);
        self.events.push(FugueEvent::Ostinato {
            voice_id: self.current_voice,
            pattern: notes,
        });
        visit::visit_expr_while(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast ExprMatch) {
        self.events.push(FugueEvent::Modulation {
            to_mode: Mode::Dorian,
        }); // Example
        visit::visit_expr_match(self, node);
        self.events.push(FugueEvent::Modulation {
            to_mode: Mode::Ionian,
        }); // Return
    }

    fn visit_expr_if(&mut self, node: &'ast ExprIf) {
        self.events.push(FugueEvent::Episode { intensity: 0.5 });
        visit::visit_expr_if(self, node);
    }
}
