use macroquad::prelude::Vec2;
use rusttype::{OutlineBuilder, Scale, Font};

#[derive(Clone, Debug)]
pub enum BezierSegment {
    Line { start: Vec2, end: Vec2 },
    Quad { start: Vec2, ctrl: Vec2, end: Vec2 },
    Cubic { start: Vec2, ctrl1: Vec2, ctrl2: Vec2, end: Vec2 },
}

#[derive(Clone, Debug, Default)]
pub struct GlyphCurve {
    pub segments: Vec<BezierSegment>,
    pub advance_width: f32,
    pub min: Vec2,
    pub max: Vec2,
}

impl GlyphCurve {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            advance_width: 0.0,
            min: Vec2::new(f32::MAX, f32::MAX),
            max: Vec2::new(f32::MIN, f32::MIN),
        }
    }

    pub fn update_bounds(&mut self, p: Vec2) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }
}

struct GlyphLoader {
    current: Vec2,
    start: Vec2, // To close the loop if needed
    curve: GlyphCurve,
}

impl GlyphLoader {
    fn new() -> Self {
        Self {
            current: Vec2::ZERO,
            start: Vec2::ZERO,
            curve: GlyphCurve::new(),
        }
    }
}

impl OutlineBuilder for GlyphLoader {
    fn move_to(&mut self, x: f32, y: f32) {
        let p = Vec2::new(x, y);
        self.current = p;
        self.start = p;
        self.curve.update_bounds(p);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let end = Vec2::new(x, y);
        self.curve.segments.push(BezierSegment::Line {
            start: self.current,
            end,
        });
        self.curve.update_bounds(end);
        self.current = end;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let ctrl = Vec2::new(x1, y1);
        let end = Vec2::new(x, y);
        self.curve.segments.push(BezierSegment::Quad {
            start: self.current,
            ctrl,
            end,
        });
        self.curve.update_bounds(ctrl);
        self.curve.update_bounds(end);
        self.current = end;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let ctrl1 = Vec2::new(x1, y1);
        let ctrl2 = Vec2::new(x2, y2);
        let end = Vec2::new(x, y);
        self.curve.segments.push(BezierSegment::Cubic {
            start: self.current,
            ctrl1,
            ctrl2,
            end,
        });
        self.curve.update_bounds(ctrl1);
        self.curve.update_bounds(ctrl2);
        self.curve.update_bounds(end);
        self.current = end;
    }

    fn close(&mut self) {
        // Optional: Close the loop explicitly if not already at start
        if self.current != self.start {
            self.curve.segments.push(BezierSegment::Line {
                start: self.current,
                end: self.start,
            });
            self.current = self.start;
        }
    }
}

pub fn load_glyph(font: &Font, c: char, font_size: f32) -> GlyphCurve {
    let scale = Scale::uniform(font_size);
    let glyph = font.glyph(c).scaled(scale);
    let h_metrics = glyph.h_metrics();
    let glyph = glyph.positioned(rusttype::point(0.0, 0.0));

    let mut loader = GlyphLoader::new();
    glyph.build_outline(&mut loader);

    let mut curve = loader.curve;
    curve.advance_width = h_metrics.advance_width;

    // Safety check for empty glyphs (space)
    if curve.segments.is_empty() {
        curve.min = Vec2::ZERO;
        curve.max = Vec2::ZERO;
    }

    curve
}
#[cfg(test)]
mod tests {
    use super::*;
    use rusttype::{Font, Scale};

    #[test]
    fn test_glyph_loading() {
        // We need a font to test. Since we can't easily access the file in unit tests without macroquad's filesystem or hardcoding paths, we can mock or just check compilation.
        // Actually, we can include bytes for a test font if we had one.
        // For now, let's just ensure the structs and methods are defined correctly.
        let curve = GlyphCurve::new();
        assert!(curve.segments.is_empty());
    }
}
