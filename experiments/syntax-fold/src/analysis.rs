use crate::origami::Mesh;
use std::collections::HashMap;

pub fn check_maekawa(mesh: &Mesh) -> Vec<usize> {
    let mut vertex_creases: HashMap<usize, Vec<usize>> = HashMap::new();

    // Map vertices to connected creases
    for (i, crease) in mesh.creases.iter().enumerate() {
        let edge = &mesh.edges[crease.edge_index];
        vertex_creases.entry(edge.a).or_default().push(i);
        vertex_creases.entry(edge.b).or_default().push(i);
    }

    let mut invalid_vertices = Vec::new();

    for (&v_idx, creases) in vertex_creases.iter() {
        // Filter out "Flat" creases
        let active_creases: Vec<&crate::origami::Crease> = creases.iter()
            .map(|&i| &mesh.creases[i])
            .filter(|c| (c.design_angle - std::f32::consts::PI).abs() > 0.1)
            .collect();

        if active_creases.is_empty() { continue; }

        // If fold line ends (degree 1), it's invalid (singularity)
        if active_creases.len() == 1 {
            invalid_vertices.push(v_idx);
            continue;
        }

        // If fold line passes through (degree 2), it must be straight (180 deg).
        // My grid is straight, so M-M is valid? No, M-V is valid?
        // Actually, M-M is valid if angles are 180. (Fold continues).
        // If angle is 90 (corner), M-M is valid?
        // Let's stick to Degree 4 check.

        if active_creases.len() == 4 {
            let mut m = 0;
            let mut v = 0;
            for c in active_creases {
                if c.design_angle < std::f32::consts::PI { m += 1; }
                else { v += 1; }
            }

            // Maekawa: |M - V| = 2
            if (m as i32 - v as i32).abs() != 2 {
                invalid_vertices.push(v_idx);
            }
        } else if active_creases.len() == 3 {
            // T-junction of folds. Usually invalid for flat folding unless special geometry.
            invalid_vertices.push(v_idx);
        }
    }

    invalid_vertices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::origami::{Mesh};
    use macroquad::prelude::*;

    #[test]
    fn test_maekawa_invalid() {
        // Create a mesh with 1 vertex and 4 creases
        let mut mesh = Mesh::new();
        let center = mesh.add_vertex(vec3(0.0,0.0,0.0), vec2(0.,0.), false);
        let n = mesh.add_vertex(vec3(0.0,1.0,0.0), vec2(0.,0.), false);
        let e = mesh.add_vertex(vec3(1.0,0.0,0.0), vec2(0.,0.), false);
        let s = mesh.add_vertex(vec3(0.0,-1.0,0.0), vec2(0.,0.), false);
        let w = mesh.add_vertex(vec3(-1.0,0.0,0.0), vec2(0.,0.), false);

        mesh.add_triangle(center, n, e, RED);
        mesh.add_triangle(center, e, s, RED);
        mesh.add_triangle(center, s, w, RED);
        mesh.add_triangle(center, w, n, RED);

        let edge_n = mesh.get_edge_index(center, n).unwrap();
        let edge_e = mesh.get_edge_index(center, e).unwrap();
        let edge_s = mesh.get_edge_index(center, s).unwrap();
        let edge_w = mesh.get_edge_index(center, w).unwrap();

        // Assign M, M, M, M (Invalid)
        let pi = std::f32::consts::PI;
        mesh.add_crease(edge_n, pi/2.0, 1.0, true);
        mesh.add_crease(edge_e, pi/2.0, 1.0, true);
        mesh.add_crease(edge_s, pi/2.0, 1.0, true);
        mesh.add_crease(edge_w, pi/2.0, 1.0, true);

        let invalid = check_maekawa(&mesh);
        assert!(!invalid.is_empty(), "Should be invalid (4M)");
    }
}
