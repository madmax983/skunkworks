use num_complex::Complex;
use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct Polygon {
    pub vertices: Vec<Complex<f64>>,
    #[allow(dead_code)]
    pub center: Complex<f64>,
}

pub struct Tiling {
    pub polygons: Vec<Polygon>,
}

impl Tiling {
    pub fn new_5_4(depth: usize) -> Self {
        let mut polygons = Vec::new();

        // 1. Calculate the vertices of the central pentagon for {5, 4}
        // Formula for distance from center to vertex in {p, q}:
        // cosh(r) = cos(pi/q) / sin(pi/p)
        // p=5, q=4
        let p = 5.0;
        let q = 4.0;

        let cos_pi_q = (PI / q).cos();
        let sin_pi_p = (PI / p).sin();
        let cosh_r = cos_pi_q / sin_pi_p;
        let r_hyperbolic = cosh_r.acosh();

        // Euclidean radius in Poincaré disk: R = tanh(r_hyperbolic / 2)
        let r_euclidean = (r_hyperbolic / 2.0).tanh();

        let mut central_vertices = Vec::new();
        for k in 0..5 {
            let angle = 2.0 * PI * k as f64 / 5.0;
            central_vertices.push(Complex::from_polar(r_euclidean, angle));
        }

        let central_polygon = Polygon {
            vertices: central_vertices.clone(),
            center: Complex::new(0.0, 0.0),
        };

        polygons.push(central_polygon);

        // 2. Generate neighbors
        // The generators of the symmetry group map the central polygon to its neighbors.
        // Reflection across an edge?
        // Or simpler: We know the neighbors share an edge.
        // We can find the Mobius transformation that maps the center to the neighbor's center.
        // Or we can just reflect the vertices across the geodesic of the edge.

        // Let's use a queue for BFS generation.
        // We need to avoid duplicates. We can check distance of centers.

        let mut queue = Vec::new();
        queue.push((central_vertices, 0));

        let mut centers = Vec::new();
        centers.push(Complex::new(0.0, 0.0));

        let mut head = 0;
        while head < polygons.len() && head < 1000 {
            // Safety break
            let current_poly = polygons[head].clone();
            let current_depth = if head == 0 { 0 } else { 1 }; // Simplified depth logic for now

            if current_depth >= depth {
                head += 1;
                continue;
            }

            // For each edge, find the reflected polygon
            let n = current_poly.vertices.len();
            for i in 0..n {
                let v1 = current_poly.vertices[i];
                let v2 = current_poly.vertices[(i + 1) % n];

                // Reflect across geodesic passing through v1, v2
                // To reflect point z across geodesic defined by v1, v2:
                // 1. Map geodesic to real axis using Mobius M
                // 2. Conjugate (reflection across real axis)
                // 3. Map back M^-1

                // Construct M that maps v1 -> -1, v2 -> 1?
                // Or map geodesic to real axis.
                // Actually, simpler: circle inversion.
                // If geodesic is a line through origin: reflection is just conjugation (rotated).
                // If geodesic is a circle |z - c|^2 = R^2: inversion is c + R^2 / conj(z - c).

                // We need the circle of the geodesic v1-v2.
                // We can reuse Geodesic::euclidean_circle

                let geo = crate::math::Geodesic::new(v1, v2);
                let reflected_verts: Vec<Complex<f64>> = current_poly
                    .vertices
                    .iter()
                    .map(|&v| {
                        if let Some((c, r)) = geo.euclidean_circle() {
                            // Inversion in circle
                            c + (r * r) / (v - c).conj()
                        } else {
                            // Line through origin (should not happen for {5,4} sides except if centered?)
                            // Actually {5,4} centered at origin has no edges passing through origin.
                            // But for completeness:
                            // The line passes through 0, v1.
                            // Reflection is simple geometry.
                            // Not needed here for {5,4} standard orientation.
                            v
                        }
                    })
                    .collect();

                // Compute new center
                let mut new_center = Complex::new(0.0, 0.0);
                for v in &reflected_verts {
                    new_center += v;
                }
                new_center /= reflected_verts.len() as f64;

                // Check if we already have this polygon (by center proximity)
                let mut exists = false;
                for c in &centers {
                    if (c - new_center).norm() < 1e-4 {
                        exists = true;
                        break;
                    }
                }

                if !exists {
                    centers.push(new_center);
                    polygons.push(Polygon {
                        vertices: reflected_verts.clone(),
                        center: new_center,
                    });

                    // In a real BFS we need proper depth tracking.
                    // Here we just expand.
                }
            }

            head += 1;
        }

        Self { polygons }
    }
}
