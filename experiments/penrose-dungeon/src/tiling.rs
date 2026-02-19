use macroquad::prelude::*;
use std::f32::consts::PI;

const PHI: f32 = 1.61803398875;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriangleType {
    Acute,  // Blue (36-72-72)
    Obtuse, // Red (108-36-36)
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Vec2, // Apex
    pub b: Vec2, // Side 1 end
    pub c: Vec2, // Side 2 end
    pub kind: TriangleType,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, kind: TriangleType) -> Self {
        Self { a, b, c, kind }
    }
}

pub fn subdivide(triangles: &[Triangle]) -> Vec<Triangle> {
    let mut result = Vec::with_capacity(triangles.len() * 2);

    for t in triangles {
        match t.kind {
            TriangleType::Obtuse => {
                // Red (Obtuse) -> 1 Red + 1 Blue
                // P splits AB
                let p = t.a + (t.b - t.a) / PHI;

                // New Red (Obtuse): P, C, A
                result.push(Triangle::new(p, t.c, t.a, TriangleType::Obtuse));

                // New Blue (Acute): P, C, B
                result.push(Triangle::new(p, t.c, t.b, TriangleType::Acute));
            },
            TriangleType::Acute => {
                // Blue (Acute) -> 2 Blue + 1 Red
                // P splits BA
                let p = t.b + (t.a - t.b) / PHI;
                // Q splits BC
                let q = t.b + (t.c - t.b) / PHI;

                // New Blue (Acute): Q, P, B
                result.push(Triangle::new(q, p, t.b, TriangleType::Acute));

                // New Red (Obtuse): P, Q, A
                result.push(Triangle::new(p, q, t.a, TriangleType::Obtuse));

                // New Blue (Acute): C, Q, A
                result.push(Triangle::new(t.c, q, t.a, TriangleType::Acute));
            }
        }
    }
    result
}

pub fn generate_initial_tiling() -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let center = Vec2::ZERO;
    let radius = 1000.0;
    for i in 0..10 {
        let angle1 = (i as f32) * PI / 5.0;
        let angle2 = ((i + 1) as f32) * PI / 5.0;
        let p1 = vec2(angle1.cos(), angle1.sin()) * radius;
        let p2 = vec2(angle2.cos(), angle2.sin()) * radius;

        // Alternating handedness (B, C swap) to ensure edge matching
        if i % 2 == 0 {
             triangles.push(Triangle::new(center, p2, p1, TriangleType::Acute));
        } else {
             triangles.push(Triangle::new(center, p1, p2, TriangleType::Acute));
        }
    }
    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subdivision_growth() {
        let mut triangles = generate_initial_tiling();
        assert_eq!(triangles.len(), 10);

        // Iteration 1
        triangles = subdivide(&triangles);
        assert_eq!(triangles.len(), 30);

        // Iteration 2
        triangles = subdivide(&triangles);
        assert_eq!(triangles.len(), 80);
    }
}
