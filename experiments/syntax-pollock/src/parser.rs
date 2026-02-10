use syn::{self, visit::Visit};
use crate::painter::Canvas;
use crate::musician::Musician;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct SyntaxParser<'a> {
    pub canvas: &'a mut Canvas,
    pub musician: &'a mut Musician,
    pub depth: u32,
}

impl<'a> SyntaxParser<'a> {
    fn get_color(&self, input: &str) -> [u8; 3] {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let hash = hasher.finish();
        [(hash & 0xFF) as u8, ((hash >> 8) & 0xFF) as u8, ((hash >> 16) & 0xFF) as u8]
    }

    fn get_pos(&self, input: &str) -> (u32, u32) {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        self.depth.hash(&mut hasher); // vary by depth
        let hash = hasher.finish();
        // naive mapping to canvas
        let w = self.canvas.image.width();
        let h = self.canvas.image.height();
        if w == 0 || h == 0 { return (0, 0); }
        ((hash % w as u64) as u32, ((hash >> 16) % h as u64) as u32)
    }
}

impl<'a, 'ast> Visit<'ast> for SyntaxParser<'a> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        let name = node.sig.ident.to_string();
        let color = self.get_color(&name);
        let (x, y) = self.get_pos(&name);

        // Functions are splashes
        self.canvas.splash(x, y, color, 20 + self.depth * 2);

        // Play a melody based on name
        let base_freq = (color[0] as f32 * 2.0) + 200.0;
        self.musician.play_note(base_freq, 0.2);

        self.depth += 1;
        syn::visit::visit_item_fn(self, node);
        self.depth -= 1;
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
         let name = node.ident.to_string();
         let color = self.get_color(&name);
         let (x, y) = self.get_pos(&name);

         // Structs are blocks/strokes
         self.canvas.stroke((x, y), (x + 50, y + 50), color, 5);

         // Play a chord
         let base_freq = (color[1] as f32) + 100.0;
         self.musician.play_note(base_freq, 0.5);
         self.musician.play_note(base_freq * 1.5, 0.5); // Fifth

         syn::visit::visit_item_struct(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
         let color = [255, 255, 0]; // Yellow for match
         let (x, y) = self.get_pos("match");

         // Splatter
         self.canvas.splash(x, y, color, 30);
         self.musician.play_note(800.0, 0.1); // High pitch alert

         syn::visit::visit_expr_match(self, node);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        let (x, y) = self.get_pos("local");
        let color = [100, 100, 100]; // Grey drip
        self.canvas.drip(x, y, 30, color);
        syn::visit::visit_local(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fn() {
        let mut canvas = Canvas::new(100, 100);
        let mut musician = Musician::new();
        let code = "fn main() { println!(\"Hello\"); }";
        let ast = syn::parse_file(code).unwrap();

        let mut parser = SyntaxParser {
            canvas: &mut canvas,
            musician: &mut musician,
            depth: 0,
        };

        parser.visit_file(&ast);

        // Assert canvas has some paint
        let mut painted = false;
        'outer: for y in 0..100 {
            for x in 0..100 {
                if canvas.image.get_pixel(x, y).0 != [0, 0, 0] {
                    painted = true;
                    break 'outer;
                }
            }
        }
        assert!(painted, "Canvas should have paint after parsing function");

        // Assert musician has some notes
        assert!(!musician.buffer.is_empty(), "Musician should have played notes");
    }
}
