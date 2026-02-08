use resonance_audio::audio::AudioCommand;
use std::fs;
use std::path::Path;
use syn::visit::Visit;
use walkdir::WalkDir;
pub struct Event {
    pub time_offset: f32, // Time in seconds from start
    pub command: AudioCommand,
    pub label: String,
    pub file: String,
}

pub struct CodeWalker {
    pub events: Vec<Event>,
    pub current_time: f32,
    pub current_file: String,
    pub width: usize,
    pub height: usize,
}

impl CodeWalker {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            events: Vec::new(),
            current_time: 0.0,
            current_file: String::new(),
            width,
            height,
        }
    }

    fn get_pos(&self, s: &str) -> (usize, usize) {
        let h1 = stable_hash(&format!("{}{}", s, self.current_file));
        let x = (h1 as usize) % self.width;

        let h2 = stable_hash(&format!("{}y{}", s, self.current_file));
        let y = (h2 as usize) % self.height;
        (x, y)
    }

    fn add_event(&mut self, command: AudioCommand, label: String, duration: f32) {
        self.events.push(Event {
            time_offset: self.current_time,
            command,
            label,
            file: self.current_file.clone(),
        });
        self.current_time += duration;
    }
}

impl<'ast> Visit<'ast> for CodeWalker {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let name = i.sig.ident.to_string();
        let (x, y) = self.get_pos(&name);

        // Complexity metric: number of args
        let strength = (i.sig.inputs.len() as f32 * 0.2).clamp(0.1, 1.0);

        self.add_event(
            AudioCommand::Pluck { x, y, strength },
            format!("fn {}", name),
            0.1, // Functions take 100ms to "announce"
        );

        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let name = i.ident.to_string();
        let (x, y) = self.get_pos(&name);

        self.add_event(
            AudioCommand::AddWall { x, y },
            format!("struct {}", name),
            0.05,
        );

        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        let name = i.ident.to_string();
        let (x, y) = self.get_pos(&name);

        self.add_event(
            AudioCommand::AddWall { x, y },
            format!("enum {}", name),
            0.05,
        );
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        let loop_id = format!("loop_{}", self.events.len());
        let (x, y) = self.get_pos(&loop_id);

        // Loops create a temporary tone (hum)
        self.add_event(
            AudioCommand::Tone {
                x,
                y,
                frequency: 220.0,
                strength: 0.1,
                duration_ms: 500,
            },
            "loop".to_string(),
            0.1,
        );

        syn::visit::visit_expr_loop(self, i);
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        let if_id = format!("if_{}", self.events.len());
        let (x, y) = self.get_pos(&if_id);

        self.add_event(
            AudioCommand::Pluck {
                x,
                y,
                strength: 0.05,
            },
            "if".to_string(),
            0.05,
        );
        syn::visit::visit_expr_if(self, i);
    }
}

fn stable_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.bytes() {
        hash = hash ^ (byte as u64);
        hash = hash.wrapping_mul(0x1099511628211625);
    }
    hash
}

pub fn walk_path(root: &Path, width: usize, height: usize) -> Vec<Event> {
    let mut walker = CodeWalker::new(width, height);

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            let path_str = entry.path().display().to_string();
            walker.current_file = path_str.clone();

            // Announce file
            walker.add_event(
                AudioCommand::ClearWaves, // Clear waves to silence the "room" for new file? No, let it ring.
                format!("File: {}", path_str),
                0.2,
            );

            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(ast) = syn::parse_file(&content) {
                    walker.visit_file(&ast);
                }
            }
        }
    }
    walker.events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walker() {
        let code = r#"
            struct Foo {}
            fn bar() {
                loop {}
            }
        "#;

        // We can't easily mock WalkDir without file system.
        // But we can test CodeWalker directly on parsed AST.
        let mut walker = CodeWalker::new(100, 100);
        walker.current_file = "test.rs".to_string();

        let ast = syn::parse_file(code).unwrap();
        walker.visit_file(&ast);

        assert!(!walker.events.is_empty());

        // Check struct event
        assert!(walker.events.iter().any(|e| e.label == "struct Foo"));
        // Check fn event
        assert!(walker.events.iter().any(|e| e.label == "fn bar"));
        // Check loop event
        assert!(walker.events.iter().any(|e| e.label == "loop"));
    }
}
