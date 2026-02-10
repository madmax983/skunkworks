use crate::outline::{GlyphOutline, PathOp};
use macroquad::prelude::Vec2;
use rusttype::Point;

#[derive(Debug, Clone)]
pub struct GlyphVertex {
    pub position: Vec2,
    pub normal: Vec2,
    pub uv: f32, // Progress along the strip [0.0, 1.0]
}

#[derive(Debug, Clone)]
pub struct GlyphMesh {
    pub strips: Vec<Vec<GlyphVertex>>,
}

impl GlyphMesh {
    pub fn new() -> Self {
        Self { strips: Vec::new() }
    }
}

pub fn tessellate(outline: &GlyphOutline) -> GlyphMesh {
    let mut mesh = GlyphMesh::new();
    let mut current_strip_points = Vec::new();
    let mut last_point = Vec2::ZERO;

    // 1. Flatten to points
    for op in &outline.ops {
        match op {
            PathOp::MoveTo(p) => {
                if !current_strip_points.is_empty() {
                    mesh.strips.push(create_strip_vertices(current_strip_points));
                }
                current_strip_points = Vec::new();
                last_point = to_vec2(*p);
                current_strip_points.push(last_point);
            }
            PathOp::LineTo(p) => {
                let p = to_vec2(*p);
                current_strip_points.push(p);
                last_point = p;
            }
            PathOp::QuadTo(c, p) => {
                let c = to_vec2(*c);
                let p = to_vec2(*p);
                let steps = 10;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let pt = quadratic_bezier(last_point, c, p, t);
                    current_strip_points.push(pt);
                }
                last_point = p;
            }
            PathOp::CurveTo(c1, c2, p) => {
                let c1 = to_vec2(*c1);
                let c2 = to_vec2(*c2);
                let p = to_vec2(*p);
                let steps = 10;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let pt = cubic_bezier(last_point, c1, c2, p, t);
                    current_strip_points.push(pt);
                }
                last_point = p;
            }
            PathOp::Close => {
                if let Some(first) = current_strip_points.first().cloned() {
                    current_strip_points.push(first);
                }
            }
        }
    }
    if !current_strip_points.is_empty() {
        mesh.strips.push(create_strip_vertices(current_strip_points));
    }

    mesh
}

fn create_strip_vertices(points: Vec<Vec2>) -> Vec<GlyphVertex> {
    let mut vertices = Vec::new();
    let len = points.len();
    if len < 2 { return vertices; }

    for (i, &p) in points.iter().enumerate() {
        let normal = if i == 0 {
            get_normal(points[0], points[1])
        } else if i == len - 1 {
            get_normal(points[i - 1], points[i])
        } else {
            (get_normal(points[i - 1], points[i]) + get_normal(points[i], points[i + 1]))
                .normalize_or_zero()
        };

        vertices.push(GlyphVertex {
            position: p,
            normal,
            uv: i as f32 / (len as f32 - 1.0),
        });
    }
    vertices
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

fn get_normal(a: Vec2, b: Vec2) -> Vec2 {
    let dir = (b - a).normalize_or_zero();
    Vec2::new(-dir.y, dir.x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outline::{GlyphOutline, PathOp};
    use rusttype::Point;

    #[test]
    fn test_tessellate() {
        let mut outline = GlyphOutline::new();
        outline.ops.push(PathOp::MoveTo(Point { x: 0.0, y: 0.0 }));
        outline.ops.push(PathOp::LineTo(Point { x: 10.0, y: 0.0 }));
        outline.ops.push(PathOp::LineTo(Point { x: 5.0, y: 10.0 }));
        outline.ops.push(PathOp::Close);

        let mesh = tessellate(&outline);
        assert_eq!(mesh.strips.len(), 1);
        assert!(mesh.strips[0].len() >= 4);

        // check normals
        let v0 = &mesh.strips[0][0];
        // Normal of (0,0)->(10,0) is (0, 1) or (0, -1) depending on winding.
        // dir = (1, 0). normal = (-0, 1) = (0, 1).
        assert!(v0.normal.length() > 0.9);
    }
}
