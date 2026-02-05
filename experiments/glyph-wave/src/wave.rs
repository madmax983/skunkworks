use crate::outline::{GlyphOutline, PathOp};
use macroquad::prelude::Vec2;
use rusttype::Point;

#[derive(Debug, Clone)]
pub struct WaveParams {
    pub time: f32,
    pub frequency: f32,
    pub amplitude: f32,
    pub phase_speed: f32,
}

pub fn tessellate_and_distort(outline: &GlyphOutline, params: &WaveParams) -> Vec<Vec<Vec2>> {
    let mut result = Vec::new();
    let mut current_strip = Vec::new();
    let mut last_point = Vec2::ZERO;
    // let start_point = Vec2::ZERO; // Unused for now

    // 1. Flatten
    for op in &outline.ops {
        match op {
            PathOp::MoveTo(p) => {
                if !current_strip.is_empty() {
                    result.push(current_strip);
                }
                current_strip = Vec::new();
                last_point = to_vec2(*p);
                current_strip.push(last_point);
            }
            PathOp::LineTo(p) => {
                let p = to_vec2(*p);
                current_strip.push(p);
                last_point = p;
            }
            PathOp::QuadTo(c, p) => {
                let c = to_vec2(*c);
                let p = to_vec2(*p);
                let steps = 5; // Reduced for performance
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let pt = quadratic_bezier(last_point, c, p, t);
                    current_strip.push(pt);
                }
                last_point = p;
            }
            PathOp::CurveTo(c1, c2, p) => {
                 let c1 = to_vec2(*c1);
                 let c2 = to_vec2(*c2);
                 let p = to_vec2(*p);
                 let steps = 5;
                 for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let pt = cubic_bezier(last_point, c1, c2, p, t);
                    current_strip.push(pt);
                 }
                 last_point = p;
            }
            PathOp::Close => {
                if let Some(first) = current_strip.first().cloned() {
                    current_strip.push(first);
                }
            }
        }
    }
    if !current_strip.is_empty() {
        result.push(current_strip);
    }

    // 2. Distort
    let mut distorted_result = Vec::new();
    for strip in result {
        let mut distorted_strip = Vec::new();
        // If strip is too short, just copy
        if strip.len() < 2 {
            distorted_strip = strip.clone();
        } else {
             for (i, p) in strip.iter().enumerate() {
                let normal = if i == 0 {
                    get_normal(strip[0], strip[1])
                } else if i == strip.len() - 1 {
                    get_normal(strip[i-1], strip[i])
                } else {
                    // Average of normals
                    (get_normal(strip[i-1], strip[i]) + get_normal(strip[i], strip[i+1])).normalize_or_zero()
                };

                let phase = i as f32 * 0.2;
                let wave = (params.time * params.phase_speed + phase * params.frequency).sin();
                let offset = normal * wave * params.amplitude;

                distorted_strip.push(*p + offset);
            }
        }
        distorted_result.push(distorted_strip);
    }

    distorted_result
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
    use crate::outline::GlyphOutline;
    use rusttype::Point;
    use crate::outline::PathOp;

    #[test]
    fn test_tessellate_and_distort() {
        // Create a simple outline (a triangle)
        let mut outline = GlyphOutline::new();
        outline.ops.push(PathOp::MoveTo(Point{x:0.0, y:0.0}));
        outline.ops.push(PathOp::LineTo(Point{x:10.0, y:0.0}));
        outline.ops.push(PathOp::LineTo(Point{x:5.0, y:10.0}));
        outline.ops.push(PathOp::Close);

        let params1 = WaveParams {
            time: 0.0,
            frequency: 1.0,
            amplitude: 0.0, // No distortion
            phase_speed: 1.0,
        };

        let result1 = tessellate_and_distort(&outline, &params1);
        assert!(!result1.is_empty());
        let strip1 = &result1[0];
        assert!(strip1.len() >= 4); // MoveTo, LineTo, LineTo, Close

        // Distort
        let params2 = WaveParams {
            time: 1.0,
            frequency: 1.0,
            amplitude: 10.0,
            phase_speed: 1.0,
        };
        let result2 = tessellate_and_distort(&outline, &params2);

        // Compare points
        // Point 0 (0,0) might stay 0,0 if phase is 0 and wave is 0, but let's check a middle point
        // With amplitude 10, points should move.
        // Let's just check equality.
        let p1 = result1[0][1];
        let p2 = result2[0][1];

        assert!(p1 != p2, "Points should be distorted. P1: {:?}, P2: {:?}", p1, p2);
    }
}
