use ttf_parser::{Face, OutlineBuilder};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Line(Point),
    Quad(Point, Point),
    Cubic(Point, Point, Point),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contour {
    pub start: Point,
    pub segments: Vec<Segment>,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlyphOutline {
    pub contours: Vec<Contour>,
    pub advance_width: f32,
}

struct Builder {
    contours: Vec<Contour>,
    current_contour: Option<Contour>,
}

impl Builder {
    fn new() -> Self {
        Self {
            contours: Vec::new(),
            current_contour: None,
        }
    }

    fn finish(mut self) -> Vec<Contour> {
        if let Some(c) = self.current_contour {
            self.contours.push(c);
        }
        self.contours
    }
}

impl OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        if let Some(c) = self.current_contour.take() {
            self.contours.push(c);
        }
        self.current_contour = Some(Contour {
            start: Point::new(x, y),
            segments: Vec::new(),
            closed: false,
        });
    }

    fn line_to(&mut self, x: f32, y: f32) {
        if let Some(ref mut c) = self.current_contour {
            c.segments.push(Segment::Line(Point::new(x, y)));
        }
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        if let Some(ref mut c) = self.current_contour {
            c.segments
                .push(Segment::Quad(Point::new(x1, y1), Point::new(x, y)));
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        if let Some(ref mut c) = self.current_contour {
            c.segments.push(Segment::Cubic(
                Point::new(x1, y1),
                Point::new(x2, y2),
                Point::new(x, y),
            ));
        }
    }

    fn close(&mut self) {
        if let Some(ref mut c) = self.current_contour {
            c.closed = true;
        }
    }
}

pub fn load_glyph(face: &Face, char_code: char) -> Option<GlyphOutline> {
    let glyph_id = face.glyph_index(char_code)?;
    let mut builder = Builder::new();

    // Check if the glyph has an outline
    let _ = face.outline_glyph(glyph_id, &mut builder)?;

    let contours = builder.finish();
    let advance_width = face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32;

    Some(GlyphOutline {
        contours,
        advance_width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttf_parser::Face;

    #[test]
    fn test_load_glyph() {
        // Load the font bytes from assets.
        // We assume the test is run from the project root or we can use a relative path.
        // Cargo tests run in the package directory.
        let font_data = include_bytes!("../assets/font.ttf");
        let face = Face::parse(font_data, 0).expect("Failed to parse font");

        let outline = load_glyph(&face, 'A');
        assert!(outline.is_some());
        let outline = outline.unwrap();
        assert!(!outline.contours.is_empty());

        // 'A' usually has 2 contours (outer and inner).
        // Let's print to see.
        println!("Contours: {}", outline.contours.len());
        // assert!(outline.contours.len() >= 1);
    }
}
