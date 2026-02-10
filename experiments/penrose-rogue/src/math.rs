use cgmath::Point2;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TriangleType {
    Acute,  // 36-72-72
    Obtuse, // 108-36-36
}

#[derive(Clone, Debug)]
pub struct Triangle {
    // Vertices [A, B, C]
    // For Acute: A is the apex (36 deg). B, C are base (72 deg).
    // For Obtuse: A is the apex (108 deg). B, C are base (36 deg).
    pub vertices: [Point2<f32>; 3],
    pub t_type: TriangleType,
}

#[derive(Clone, Debug)]
pub struct Rhombus {
    pub vertices: [Point2<f32>; 4],
    pub r_type: TriangleType, // Acute -> Thick, Obtuse -> Thin
    pub center: Point2<f32>,
    pub id: usize,
}

// Golden Ratio
pub const PHI: f32 = 1.61803398875;

impl Triangle {
    pub fn new(a: Point2<f32>, b: Point2<f32>, c: Point2<f32>, t_type: TriangleType) -> Self {
        Self {
            vertices: [a, b, c],
            t_type,
        }
    }

    pub fn subdivide(&self) -> Vec<Triangle> {
        let a = self.vertices[0];
        let b = self.vertices[1];
        let c = self.vertices[2];

        match self.t_type {
            TriangleType::Acute => {
                // Rule: Split Leg AB at P.
                // P = A + (B - A) / PHI
                // New Triangles:
                // 1. Acute: Apex C, Legs CB, CP. Base PB.
                // 2. Obtuse: Apex P, Legs PA, PC. Base AC.

                let p = a + (b - a) / PHI;

                vec![
                    Triangle::new(c, p, b, TriangleType::Acute),
                    Triangle::new(p, c, a, TriangleType::Obtuse),
                ]
            }
            TriangleType::Obtuse => {
                // Rule: Split Base BC at P.
                // P = B + (C - B) / PHI
                // New Triangles:
                // 1. Acute: Apex B, Legs BA, BP. Base PA.
                // 2. Obtuse: Apex P, Legs PC, PA. Base CA.

                let p = b + (c - b) / PHI;

                vec![
                    Triangle::new(b, p, a, TriangleType::Acute),
                    Triangle::new(p, c, a, TriangleType::Obtuse),
                ]
            }
        }
    }
}

pub fn generate_sun(radius: f32) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let center = Point2::new(0.0, 0.0);

    // 10 Acute triangles around the center.
    // Alternating handedness (A, B, C) vs (A, C, B) to ensure shared edges match types.

    for i in 0..10 {
        let theta1 = (i as f32) * std::f32::consts::PI / 5.0;
        let theta2 = ((i + 1) as f32) * std::f32::consts::PI / 5.0;

        let p1 = Point2::new(theta1.cos() * radius, theta1.sin() * radius);
        let p2 = Point2::new(theta2.cos() * radius, theta2.sin() * radius);

        // Even: A=Center, B=Right(p2), C=Left(p1)?
        // Note: In standard math angle increases counter-clockwise.
        // p1 is "Right" (smaller angle), p2 is "Left" (larger angle) relative to origin?
        // Let's call p1 "Start" and p2 "End".

        // We want neighbor compatibility.
        // Tri i shares edge "Start" with Tri i-1 "End".
        // Tri i shares edge "End" with Tri i+1 "Start".

        if i % 2 == 0 {
            // Even: Split the "End" edge?
            // Acute splits AB.
            // If we set B=p2 (End), then End edge is split.
            triangles.push(Triangle::new(center, p2, p1, TriangleType::Acute));
        } else {
            // Odd: Split the "End" edge?
            // Neighbor i-1 (Even) split its "End" edge (which is our "Start" edge).
            // So we must split our "Start" edge to match?
            // Start edge is p1.
            // So we set B=p1.
            triangles.push(Triangle::new(center, p1, p2, TriangleType::Acute));
        }
    }

    triangles
}

pub fn generate_tiling(iterations: usize) -> (Vec<Rhombus>, Vec<Vec<usize>>) {
    let mut triangles = generate_sun(100.0); // Large starting radius

    for _ in 0..iterations {
        let mut next = Vec::new();
        for tri in triangles {
            next.extend(tri.subdivide());
        }
        triangles = next;
    }

    // Convert to Rhombuses
    // Join triangles sharing the base edge (BC).

    let mut edge_map: HashMap<(i32, i32, i32, i32), Vec<usize>> = HashMap::new();

    // Helper to quantize points for keying
    // We use a relatively coarse epsilon because floating point error accumulates after substitutions.
    let key = |p: Point2<f32>| {
        ((p.x * 1000.0).round() as i32, (p.y * 1000.0).round() as i32)
    };

    let edge_key = |p1: Point2<f32>, p2: Point2<f32>| {
        let k1 = key(p1);
        let k2 = key(p2);
        if k1 < k2 { (k1.0, k1.1, k2.0, k2.1) } else { (k2.0, k2.1, k1.0, k1.1) }
    };

    for (i, tri) in triangles.iter().enumerate() {
        // Base edge is vertices[1] -> vertices[2]
        let edge = edge_key(tri.vertices[1], tri.vertices[2]);
        edge_map.entry(edge).or_default().push(i);
    }

    let mut rhombuses = Vec::new();

    for (_, indices) in edge_map {
        if indices.len() == 2 {
            let t1 = &triangles[indices[0]];
            let t2 = &triangles[indices[1]];

            // Verify types match
            if t1.t_type == t2.t_type {
                 // Form Rhombus
                 // Vertices: t1.A, t1.B, t2.A, t1.C?
                 // Shared edge is B-C.
                 // So vertices are t1.A, t1.B, t2.A, t1.C.
                 // Note: t1.B is t2.C or t2.B?
                 // Shared edge (p1, p2).
                 // For t1: B, C are endpoints.
                 // For t2: B, C are endpoints.
                 // So vertices are [t1.A, t1.B, t2.A, t1.C] in cyclic order?
                 // t1.B -> t2.A -> t1.C -> t1.A ?
                 // Let's trace.
                 // t1: A -> B -> C.
                 // t2: A' -> B' -> C'.
                 // Edge BC is shared.
                 // Rhombus vertices: A, B, A', C.
                 // This forms a quad.

                 let r = Rhombus {
                     vertices: [t1.vertices[0], t1.vertices[1], t2.vertices[0], t1.vertices[2]],
                     r_type: t1.t_type,
                     center: Point2::new(
                         (t1.vertices[0].x + t2.vertices[0].x)/2.0,
                         (t1.vertices[0].y + t2.vertices[0].y)/2.0
                     ),
                     id: rhombuses.len(),
                 };
                 rhombuses.push(r);
            }
        }
    }

    // Adjacency
    let mut rhombus_edge_map: HashMap<(i32, i32, i32, i32), Vec<usize>> = HashMap::new();
    for (rid, r) in rhombuses.iter().enumerate() {
        for i in 0..4 {
            let p1 = r.vertices[i];
            let p2 = r.vertices[(i+1)%4];
            rhombus_edge_map.entry(edge_key(p1, p2)).or_default().push(rid);
        }
    }

    let mut adjacency = vec![Vec::new(); rhombuses.len()];
    for (_, rids) in rhombus_edge_map {
        if rids.len() == 2 {
            adjacency[rids[0]].push(rids[1]);
            adjacency[rids[1]].push(rids[0]);
        }
    }

    (rhombuses, adjacency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let (rhombuses, adjacency) = generate_tiling(2);
        println!("Generated {} rhombuses", rhombuses.len());
        assert!(!rhombuses.is_empty());
        assert!(!adjacency.is_empty());

        // Verify adjacency
        let connected_count = adjacency.iter().filter(|a| !a.is_empty()).count();
        println!("Connected rhombuses: {}", connected_count);
        assert!(connected_count > 0);
    }
}
