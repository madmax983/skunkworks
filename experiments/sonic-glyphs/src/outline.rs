use macroquad::prelude::Vec2;
use rusttype::{OutlineBuilder, Point};

#[derive(Debug, Clone)]
pub enum PathOp {
    MoveTo(Point<f32>),
    LineTo(Point<f32>),
    QuadTo(Point<f32>, Point<f32>),              // control, to
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
        self.ops
            .push(PathOp::QuadTo(Point { x: cx, y: cy }, Point { x, y }));
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

/// Tessellates the outline into a list of contours, where each contour is a list of points.
/// `step_distance` is roughly the distance between points.
pub fn tessellate(outline: &GlyphOutline, steps: usize) -> Vec<Vec<Vec2>> {
    let mut contours = Vec::new();
    let mut current_contour = Vec::new();
    let mut last_point = Vec2::ZERO;

    for op in &outline.ops {
        match op {
            PathOp::MoveTo(p) => {
                if !current_contour.is_empty() {
                    contours.push(current_contour);
                }
                current_contour = Vec::new();
                last_point = to_vec2(*p);
                current_contour.push(last_point);
            }
            PathOp::LineTo(p) => {
                let p = to_vec2(*p);
                // Subdivide line if needed? For now just push end point.
                // Actually for physics we might want points along the line.
                // Let's just push the endpoint for now, or maybe subdivide linearly?
                // For simplicity, let's just use the endpoints + midpoints if we want high res.
                // But the user prompt asked for "smooth curve interpolation".
                // I'll stick to simple tessellation first.
                current_contour.push(p);
                last_point = p;
            }
            PathOp::QuadTo(c, p) => {
                let c = to_vec2(*c);
                let p = to_vec2(*p);
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let pt = quadratic_bezier(last_point, c, p, t);
                    current_contour.push(pt);
                }
                last_point = p;
            }
            PathOp::CurveTo(c1, c2, p) => {
                let c1 = to_vec2(*c1);
                let c2 = to_vec2(*c2);
                let p = to_vec2(*p);
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let pt = cubic_bezier(last_point, c1, c2, p, t);
                    current_contour.push(pt);
                }
                last_point = p;
            }
            PathOp::Close => {
                // Ensure closed
                if let Some(first) = current_contour.first().cloned() {
                    // Check distance to avoid duplicate point
                    if last_point.distance(first) > 0.1 {
                        current_contour.push(first);
                    }
                }
            }
        }
    }
    if !current_contour.is_empty() {
        contours.push(current_contour);
    }
    contours
}

fn to_vec2(p: Point<f32>) -> Vec2 {
    Vec2::new(p.x, p.y)
}

fn quadratic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    p0 * (u * u * u) + p1 * (3.0 * u * u * t) + p2 * (3.0 * u * t * t) + p3 * (t * t * t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusttype::{Font, Scale};

    #[test]
    fn test_tessellate() {
        // Load font relative to crate root
        let font_data = include_bytes!("../assets/font.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        let glyph = font.glyph('S').scaled(Scale::uniform(100.0));
        let glyph = glyph.positioned(rusttype::point(0.0, 0.0));

        let mut outline = GlyphOutline::new();
        glyph.build_outline(&mut outline);

        let contours = tessellate(&outline, 10);

        assert!(!contours.is_empty());
        for contour in &contours {
            assert!(contour.len() > 2);
        }
        println!("Generated {} contours", contours.len());
        for (i, c) in contours.iter().enumerate() {
            println!("Contour {}: {} points", i, c.len());
        }
    }
}
