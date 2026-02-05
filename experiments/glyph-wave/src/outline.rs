use rusttype::{Point, OutlineBuilder};

#[derive(Debug, Clone)]
pub enum PathOp {
    MoveTo(Point<f32>),
    LineTo(Point<f32>),
    QuadTo(Point<f32>, Point<f32>), // control, to
    CurveTo(Point<f32>, Point<f32>, Point<f32>), // control1, control2, to
    Close,
}

pub struct GlyphOutline {
    pub ops: Vec<PathOp>,
}

impl GlyphOutline {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
}

impl OutlineBuilder for GlyphOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.ops.push(PathOp::MoveTo(Point { x, y }));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.ops.push(PathOp::LineTo(Point { x, y }));
    }
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.ops.push(PathOp::QuadTo(Point { x: cx, y: cy }, Point { x, y }));
    }
    fn curve_to(&mut self, cx1: f32, cy1: f32, cx2: f32, cy2: f32, x: f32, y: f32) {
        self.ops.push(PathOp::CurveTo(
            Point { x: cx1, y: cy1 },
            Point { x: cx2, y: cy2 },
            Point { x, y },
        ));
    }
    fn close(&mut self) {
        self.ops.push(PathOp::Close);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusttype::{Font, Scale};

    #[test]
    fn test_extract_outline() {
        // Load font
        let font_data = include_bytes!("../assets/font.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        // Get glyph for 'O'
        let glyph = font.glyph('O').scaled(Scale::uniform(100.0));
        let glyph = glyph.positioned(rusttype::point(0.0, 0.0));

        // Build outline
        let mut outline = GlyphOutline::new();
        glyph.build_outline(&mut outline);

        // Assert
        assert!(!outline.ops.is_empty(), "Outline should not be empty");

        // Debug print to see what we got
        println!("Captured {} ops", outline.ops.len());
        for op in &outline.ops {
            println!("{:?}", op);
        }
    }
}
