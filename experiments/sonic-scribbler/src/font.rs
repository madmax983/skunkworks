use ttf_parser::{Face, OutlineBuilder};
use crate::transform::Point;
use glam::Vec2;

pub struct FontLoader<'a> {
    face: Face<'a>,
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point), // ctrl, end
    CurveTo(Point, Point, Point), // ctrl1, ctrl2, end
    Close,
}

pub struct PathExtractor {
    pub commands: Vec<PathCommand>,
}

impl PathExtractor {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
}

impl OutlineBuilder for PathExtractor {
    fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::MoveTo(Vec2::new(x, y)));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::LineTo(Vec2::new(x, y)));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::QuadTo(Vec2::new(x1, y1), Vec2::new(x, y)));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::CurveTo(
            Vec2::new(x1, y1),
            Vec2::new(x2, y2),
            Vec2::new(x, y),
        ));
    }

    fn close(&mut self) {
        self.commands.push(PathCommand::Close);
    }
}

impl<'a> FontLoader<'a> {
    pub fn new(data: &'a [u8]) -> Option<Self> {
        Face::parse(data, 0).ok().map(|face| Self { face })
    }

    pub fn get_glyph_path(&self, c: char) -> Vec<PathCommand> {
        if let Some(gid) = self.face.glyph_index(c) {
            let mut extractor = PathExtractor::new();
            if self.face.outline_glyph(gid, &mut extractor).is_some() {
                return extractor.commands;
            }
        }
        vec![]
    }

    pub fn get_glyph_advance(&self, c: char) -> f32 {
        if let Some(gid) = self.face.glyph_index(c) {
            self.face.glyph_hor_advance(gid).unwrap_or(0) as f32
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_font_loading_and_extraction() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets/Roboto-Regular.ttf");
        let font_data = std::fs::read(&d).expect("failed to read font");
        let font = FontLoader::new(&font_data).expect("failed to parse font");
        let path = font.get_glyph_path('A');
        assert!(!path.is_empty(), "Path for 'A' should not be empty");

        // Check if we have some commands
        let has_moves = path.iter().any(|c| matches!(c, PathCommand::MoveTo(_)));
        assert!(has_moves, "Path should have MoveTo");
    }
}
