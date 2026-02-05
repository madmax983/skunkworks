use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};

pub struct Quasicrystal {
    pub atoms: Vec<Point3<f32>>,
}

// Golden Ratio
const TAU: f32 = 1.61803398875;

// Basis vectors for 6D -> 3D projection (Icosahedral)
// Derived from the 12 vertices of an icosahedron.
// We select 6 linearly independent ones (half of them).
fn get_basis_vectors() -> (Vec<Vector3<f32>>, Vec<Vector3<f32>>) {
    // Parallel space basis (Physical)
    // Vertices of icosahedron: (±1, ±τ, 0) cyclic
    let norm = (1.0 + TAU * TAU).sqrt();
    let c = 1.0 / norm;
    let t = TAU / norm;

    // 6 basis vectors (upper hemisphere)
    let par = vec![
        Vector3::new(c, t, 0.0),
        Vector3::new(-c, t, 0.0),
        Vector3::new(0.0, c, t),
        Vector3::new(0.0, -c, t),
        Vector3::new(t, 0.0, c),
        Vector3::new(t, 0.0, -c),
    ];

    // Perpendicular space basis
    // For icosahedral symmetry, we conjugate τ -> 1-τ = -1/τ
    // The algebraic conjugate of (1, τ) is (1, 1-τ)
    let tau_prime = 1.0 - TAU; // -0.618
    let norm_prime = (1.0 + tau_prime * tau_prime).sqrt();
    let c_p = 1.0 / norm_prime;
    let t_p = tau_prime / norm_prime;

    let perp = vec![
        Vector3::new(c_p, t_p, 0.0),
        Vector3::new(-c_p, t_p, 0.0),
        Vector3::new(0.0, c_p, t_p),
        Vector3::new(0.0, -c_p, t_p),
        Vector3::new(t_p, 0.0, c_p),
        Vector3::new(t_p, 0.0, -c_p),
    ];

    (par, perp)
}

pub fn generate_icosahedral_lattice(grid_radius: i32) -> Quasicrystal {
    let (basis_par, basis_perp) = get_basis_vectors();
    let mut atoms = Vec::new();

    // Iterate over 6D integer grid
    let range = -grid_radius..=grid_radius;

    // Heuristic threshold for "window" in perp space.
    // A perfect triacontahedron window gives the canonical tiling.
    // A spherical window gives a slightly disordered but valid quasicrystal structure.
    // Threshold ~ 1.5 covers the center well.
    let perp_threshold = 1.8;

    // We can flatten the loop slightly or just nest deep.
    // 6^N is manageable for small N.
    for n1 in range.clone() {
        for n2 in range.clone() {
            for n3 in range.clone() {
                for n4 in range.clone() {
                    let r_perp_partial = basis_perp[0] * (n1 as f32)
                         + basis_perp[1] * (n2 as f32)
                         + basis_perp[2] * (n3 as f32)
                         + basis_perp[3] * (n4 as f32);

                    // Optimization: if partial sum is already too huge, maybe prune?
                    // No, negative contributions can cancel it out.

                    for n5 in range.clone() {
                        for n6 in range.clone() {
                            let n5f = n5 as f32;
                            let n6f = n6 as f32;

                            let r_perp = r_perp_partial
                                + basis_perp[4] * n5f
                                + basis_perp[5] * n6f;

                            if r_perp.magnitude() < perp_threshold {
                                // Accept
                                let r_par = basis_par[0] * (n1 as f32)
                                     + basis_par[1] * (n2 as f32)
                                     + basis_par[2] * (n3 as f32)
                                     + basis_par[3] * (n4 as f32)
                                     + basis_par[4] * n5f
                                     + basis_par[5] * n6f;

                                atoms.push(Point3::from_vec(r_par));
                            }
                        }
                    }
                }
            }
        }
    }

    Quasicrystal { atoms }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_non_empty() {
        let qc = generate_icosahedral_lattice(2);
        assert!(!qc.atoms.is_empty());
        println!("Generated {} atoms with radius 2", qc.atoms.len());
    }
}
