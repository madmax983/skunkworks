use crate::geometry::{Mobius, Point};
use std::collections::{HashSet, VecDeque};
use std::f64::consts::PI;

const P: f64 = 7.0;
const Q: f64 = 3.0;

#[derive(Clone, Debug)]
pub struct Polygon {
    pub vertices: Vec<Point>,
    pub transform: Mobius, // The transform from the central polygon to this one
    pub id: u64,
}

pub fn generate_7_3_tiling(max_depth: usize) -> Vec<Polygon> {
    let mut polygons = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // 1. Calculate radius of vertices for {7,3}
    // r = sqrt( (cos(pi/p + pi/q)) / (cos(pi/p - pi/q)) )
    let num = (PI / P + PI / Q).cos();
    let den = (PI / P - PI / Q).cos();
    let r = (num / den).sqrt();

    // 2. Base Polygon Vertices (centered at 0)
    let mut base_vertices = Vec::new();
    for i in 0..P as i32 {
        let theta = 2.0 * PI * (i as f64) / P;
        base_vertices.push(Point::from_polar(r, theta));
    }

    // 3. Generators (Rotations around edge midpoints)
    // For each edge of the base polygon, calculate the midpoint M.
    // The transformation to the neighbor is Rotation(pi) around M.
    let mut generators = Vec::new();
    for i in 0..P as usize {
        let v1 = base_vertices[i];
        let v2 = base_vertices[(i + 1) % P as usize];

        // Midpoint in Poincaré disk model
        // M = (v1 + v2) / (1 + v1*conj(v2)) ? No, that's for adding velocities.
        // Midpoint of geodesic between u and v:
        // T_u maps u to 0. v maps to v'. Midpoint is v'/2? No.
        // Let's use the fact that the polygon is centered at 0.
        // The edge midpoint is simply on the ray bisecting the angle, at some distance `a`.
        // The angle of the midpoint is (theta1 + theta2)/2.
        let angle = (2.0 * PI * (i as f64) + PI) / P; // (i + 0.5) * 2pi/p
        // Wait, angle of v1 is 2pi*i/P. Angle of v2 is 2pi*(i+1)/P.
        // Midpoint angle is 2pi*(i+0.5)/P.

        // Distance `a` to midpoint:
        // Right triangle center(0) - midpoint(M) - vertex(V).
        // Angle at center = pi/p.
        // Hypotenuse (0-V) = r_dist (hyperbolic distance).
        // tanh(r_dist) = r (Euclidean distance in disk).
        // cos(angle_center) = tanh(dist_center_midpoint) / tanh(dist_center_vertex)
        // tanh(a_dist) = cos(pi/p) * tanh(r_dist)
        // Euclidean midpoint distance `a_eucl` = tanh(a_dist) ? No, `a_eucl` = tanh(a_dist/2)?
        // Wait. r is Euclidean distance. r = tanh(R/2) where R is hyperbolic distance from center.
        // tanh(R) = 2r / (1+r^2).
        // Let's use hyperbolic formulas.
        // cos(pi/p) = tanh(a_hyp) / tanh(R_hyp)
        // tanh(a_hyp) = cos(pi/p) * tanh(R_hyp)
        // But we have r = tanh(R_hyp/2)? No, standard metric ds = 2|dz|/(1-|z|^2).
        // Distance from 0 to r is ln((1+r)/(1-r)).
        // R_hyp = ln((1+r)/(1-r)).
        // tanh(R_hyp) = ( (1+r)/(1-r) - 1 ) / ( ... + 1 ) = 2r / 2 = r. Wait.
        // tanh(x) = (e^x - e^-x)/(e^x + e^-x).
        // If x = ln((1+r)/(1-r)), then e^x = (1+r)/(1-r).
        // tanh(x) = ( (1+r)/(1-r) - (1-r)/(1+r) ) / ( ... )
        // = ( (1+r)^2 - (1-r)^2 ) / ( (1+r)^2 + (1-r)^2 )
        // = ( 4r ) / ( 2(1+r^2) ) = 2r / (1+r^2).
        // So tanh(R_hyp) = 2r / (1+r^2).
        // Let T_R = tanh(R_hyp).
        // T_a = cos(pi/p) * T_R.
        // We need `a_eucl` such that 2*a_eucl / (1+a_eucl^2) = T_a.
        // Let T = T_a. 2x/(1+x^2) = T => 2x = T + T x^2 => T x^2 - 2x + T = 0.
        // x = (2 - sqrt(4 - 4 T^2)) / 2T = (1 - sqrt(1-T^2)) / T.

        let t_r = 2.0 * r / (1.0 + r * r);
        let t_a = (PI / P).cos() * t_r;
        let a_eucl = (1.0 - (1.0 - t_a * t_a).sqrt()) / t_a;

        let midpoint_angle = 2.0 * PI * (i as f64 + 0.5) / P;
        let midpoint = Point::from_polar(a_eucl, midpoint_angle);

        // Transform is Rotation(pi) around midpoint.
        // Rotation by pi around M: M + (z-M)/(...) ? No.
        // T_M(z) = (z - M) / (1 - conj(M)z). Maps M to 0.
        // Rot_pi(w) = -w.
        // Res = T_M^-1 ( - T_M(z) ).
        // T_M^-1(w) = (w + M) / (1 + conj(M)w).

        // Let's build the Mobius transform for this.
        // Step 1: Translate M to 0.
        // T1 = Mobius::translation(-midpoint). NO.
        // translation(p) maps 0 to p.
        // We want 0 to be mapped to -M? No.
        // We want M to be mapped to 0.
        // The inverse of translation(M) maps M to 0.
        let t_m = Mobius::translation(midpoint);
        let t_m_inv = t_m.inverse();

        // Step 2: Rotate by pi (180 deg) at origin.
        let rot_pi = Mobius::rotation(PI);

        // Step 3: Translate back (0 to M).
        let generator = t_m.compose(rot_pi.compose(t_m_inv)); // T_M * Rot * T_M^-1 ?
        // Wait. f(z) = T_M ( Rot ( T_M_inv (z) ) ).
        // Apply T_M_inv first (z -> local), then Rot, then T_M (local -> global).
        // compose(other) does self * other.
        // So we want T_M * (Rot * T_M_inv).

        generators.push(generator);
    }

    // 4. BFS to generate tiling
    let identity = Mobius::identity();

    // Hash based on approximate center to avoid duplicates
    // Using integer coords or string repr
    let center_key = (0, 0); // (0,0) is center
    // Better key: string of path? Or just spatial hashing?
    // Spatial hashing of the center point is tricky in hyperbolic.
    // Let's use a "string" of generator indices, reduced?
    // Or simpler: just use the center point coordinates with some epsilon.
    // (re * 1000) as i64, (im * 1000) as i64.

    // Initial polygon
    queue.push_back((identity, 0)); // (transform, depth)

    // We need a robust "visited" check.
    // Transform center (0,0) by the transform.
    let origin = Point::new(0.0, 0.0);

    while let Some((transform, depth)) = queue.pop_front() {
        let center = transform.apply(origin);
        let key = (
            (center.re * 10000.0).round() as i64,
            (center.im * 10000.0).round() as i64,
        );

        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);

        // Create polygon
        let mut poly_verts = Vec::new();
        for &v in &base_vertices {
            poly_verts.push(transform.apply(v));
        }

        let poly = Polygon {
            vertices: poly_verts,
            transform: transform.clone(),
            id: (key.0 ^ key.1.rotate_left(32)) as u64, // simple hash
        };
        polygons.push(poly);

        if depth < max_depth {
            for generator in &generators {
                // Next transform = transform * gen
                // Because we want to attach to the *current* polygon's edge.
                // The current polygon is T(P0).
                // Its edges are T(e_i).
                // We want to reflect across T(e_i).
                // Reflection across e_i is G_i.
                // Reflection across T(e_i) is T * G_i * T^-1 ?
                // No.
                // If we are at P_current, and we want to go to neighbor i (relative to P_current):
                // We apply G_i in the *local frame* of P_current.
                // So new_transform = current_transform * G_i.
                // Because G_i moves from P0 to neighbor i of P0.
                // T maps P0 to P_current.
                // T * G_i maps P0 -> Neighbor i of P0 -> Neighbor i of P_current?
                // Let's verify.
                // Point p in P0 neighbor. G_i(p) is in P0? No G_i maps P0 to neighbor.
                // So point x in P0. G_i(x) is in neighbor.
                // T(G_i(x)) is in T(neighbor).
                // Is T(neighbor) the neighbor of T(P0)? Yes, because T is conformal (isometry).
                // So yes, new_transform = transform.compose(*gen).

                let next_transform = transform.compose(*generator);
                queue.push_back((next_transform, depth + 1));
            }
        }
    }

    polygons
}
