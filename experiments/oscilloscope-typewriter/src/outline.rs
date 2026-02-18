use macroquad::prelude::*;
use rusttype::{Font, OutlineBuilder, Point, Rect, Scale};

#[derive(Clone, Debug)]
pub struct GlyphParams {
    pub points: Vec<Vec2>,
    pub normals: Vec<Vec2>,
    pub bounds: Rect<f32>,
    pub advance_width: f32,
}

struct PathBuilder {
    points: Vec<Vec2>,
    current: Vec2,
    start: Vec2,
}

impl PathBuilder {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            current: Vec2::ZERO,
            start: Vec2::ZERO,
        }
    }

    fn add_point(&mut self, p: Vec2) {
        // Avoid duplicate points
        if let Some(last) = self.points.last() {
            if last.distance_squared(p) < 0.001 {
                return;
            }
        }
        self.points.push(p);
        self.current = p;
    }
}

impl OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.start = vec2(x, y);
        self.current = vec2(x, y);
        self.add_point(vec2(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.add_point(vec2(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let p0 = self.current;
        let p1 = vec2(x1, y1);
        let p2 = vec2(x, y);

        let steps = 10;
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let p = (1.0 - t).powi(2) * p0 + 2.0 * (1.0 - t) * t * p1 + t.powi(2) * p2;
            self.add_point(p);
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let p0 = self.current;
        let p1 = vec2(x1, y1);
        let p2 = vec2(x2, y2);
        let p3 = vec2(x, y);

        let steps = 15;
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let p = (1.0 - t).powi(3) * p0
                + 3.0 * (1.0 - t).powi(2) * t * p1
                + 3.0 * (1.0 - t) * t.powi(2) * p2
                + t.powi(3) * p3;
            self.add_point(p);
        }
    }

    fn close(&mut self) {
        let start = self.start;
        self.line_to(start.x, start.y);
    }
}

pub fn extract_outline(font: &Font, c: char, scale: Scale) -> Option<GlyphParams> {
    let glyph = font.glyph(c).scaled(scale);
    let positioned = glyph.positioned(Point { x: 0.0, y: 0.0 });

    let bounds = positioned.pixel_bounding_box()?;
    let bounds_f32 = Rect {
        min: Point {
            x: bounds.min.x as f32,
            y: bounds.min.y as f32,
        },
        max: Point {
            x: bounds.max.x as f32,
            y: bounds.max.y as f32,
        },
    };

    let advance_width = positioned.unpositioned().h_metrics().advance_width;

    let mut builder = PathBuilder::new();
    positioned.build_outline(&mut builder);

    let points = builder.points;
    if points.len() < 2 {
        return None;
    }

    let mut normals = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let p = points[i];
        let next = points[(i + 1) % points.len()];
        let prev = points[(i + points.len() - 1) % points.len()];

        let t1 = (p - prev).normalize_or_zero();
        let t2 = (next - p).normalize_or_zero();
        let tangent = (t1 + t2).normalize_or_zero();

        let normal = vec2(-tangent.y, tangent.x);
        normals.push(normal);
    }

    Some(GlyphParams {
        points,
        normals,
        bounds: bounds_f32,
        advance_width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_extract_outline() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = PathBuf::from(manifest_dir);
        path.push("assets/font.ttf");

        let font_data = fs::read(&path).expect(&format!("Failed to read font at {:?}", path));
        let font = Font::try_from_bytes(&font_data).expect("Error constructing Font");
        let scale = Scale::uniform(50.0);

        // Test with 'A'
        let result = extract_outline(&font, 'A', scale);
        assert!(result.is_some(), "Should extract outline for 'A'");
        let glyph = result.unwrap();
        assert!(!glyph.points.is_empty(), "Should have points");
        assert_eq!(glyph.points.len(), glyph.normals.len(), "Normals count should match points");

        // Test with space (should be None or handled)
        // Space usually has no outline
        let result_space = extract_outline(&font, ' ', scale);
        assert!(result_space.is_none(), "Space should have no outline");
    }
}
