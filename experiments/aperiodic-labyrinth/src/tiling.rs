use nalgebra::Point2;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriangleType {
    Acute,
    Obtuse,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub vertices: [Point2<f64>; 3],
    pub ttype: TriangleType,
}

impl Triangle {
    pub fn new(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>, ttype: TriangleType) -> Self {
        Self {
            vertices: [a, b, c],
            ttype,
        }
    }

    pub fn center(&self) -> Point2<f64> {
        let [a, b, c] = self.vertices;
        Point2::new(
            (a.x + b.x + c.x) / 3.0,
            (a.y + b.y + c.y) / 3.0,
        )
    }

    pub fn contains(&self, p: Point2<f64>) -> bool {
        let [a, b, c] = self.vertices;
        let v0 = c - a;
        let v1 = b - a;
        let v2 = p - a;

        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }
}

pub fn subdivide(triangles: &[Triangle]) -> Vec<Triangle> {
    let mut result = Vec::with_capacity(triangles.len() * 2);
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0;

    for tri in triangles {
        let [a, b, c] = tri.vertices;
        match tri.ttype {
            TriangleType::Acute => {
                // P = A + (B - A) / phi
                let p = a + (b - a) / phi;
                result.push(Triangle::new(c, p, b, TriangleType::Acute));
                result.push(Triangle::new(p, c, a, TriangleType::Obtuse));
            }
            TriangleType::Obtuse => {
                // P = B + (A - B) / phi
                let p = b + (a - b) / phi;
                result.push(Triangle::new(b, c, p, TriangleType::Acute));
                result.push(Triangle::new(p, a, c, TriangleType::Obtuse));
            }
        }
    }
    result
}

pub fn generate_initial_sun() -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let center = Point2::new(0.0, 0.0);
    let r = 100.0;

    // 10 Triangles to fill 360 degrees (36 degrees each)
    for i in 0..10 {
        let angle1 = (i as f64 - 0.5) * PI / 5.0;
        let angle2 = (i as f64 + 0.5) * PI / 5.0;

        // Ensure alternating colors or just all Acute?
        // Standard Sun is 5 Kites? Or 10 Robinson Triangles.
        // If we use 10 Acute triangles joined at the center (A).
        // A is the apex (36 deg).
        // So we need 10 triangles.
        // Vertices: Center, P1, P2.

        let p1 = Point2::new(r * angle1.cos(), r * angle1.sin());
        let p2 = Point2::new(r * angle2.cos(), r * angle2.sin());

        // Note: For Penrose P3, we usually alternate mirror images.
        // But for our fractal, let's just use Acute.
        triangles.push(Triangle::new(center, p1, p2, TriangleType::Acute));
    }
    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deflation_growth() {
        let sun = generate_initial_sun();
        assert_eq!(sun.len(), 10);

        // Gen 1
        let gen1 = subdivide(&sun);
        assert_eq!(gen1.len(), 20); // 10 * 2

        // Gen 2
        let gen2 = subdivide(&gen1);
        assert_eq!(gen2.len(), 40); // 20 * 2
    }
}
