use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Quasicrystal {
    pub atoms: Vec<Point3<f32>>,
    pub edges: Vec<(Point3<f32>, Point3<f32>)>,
    pub adj: Vec<Vec<usize>>,
}

// Golden Ratio
const TAU: f32 = 1.61803398875;

// Basis vectors for 6D -> 3D projection (Icosahedral)
fn get_basis_vectors() -> (Vec<Vector3<f32>>, Vec<Vector3<f32>>) {
    // Parallel space basis (Physical)
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
    let mut atom_map: HashMap<[i32; 6], usize> = HashMap::new();
    let mut atoms = Vec::new();

    // Iterate over 6D integer grid
    let range = -grid_radius..=grid_radius;
    let perp_threshold = 1.8;

    for n1 in range.clone() {
        for n2 in range.clone() {
            for n3 in range.clone() {
                for n4 in range.clone() {
                    let r_perp_partial = basis_perp[0] * (n1 as f32)
                        + basis_perp[1] * (n2 as f32)
                        + basis_perp[2] * (n3 as f32)
                        + basis_perp[3] * (n4 as f32);

                    for n5 in range.clone() {
                        for n6 in range.clone() {
                            let n5f = n5 as f32;
                            let n6f = n6 as f32;

                            let r_perp = r_perp_partial + basis_perp[4] * n5f + basis_perp[5] * n6f;

                            if r_perp.magnitude() < perp_threshold {
                                // Accept
                                let r_par = basis_par[0] * (n1 as f32)
                                    + basis_par[1] * (n2 as f32)
                                    + basis_par[2] * (n3 as f32)
                                    + basis_par[3] * (n4 as f32)
                                    + basis_par[4] * n5f
                                    + basis_par[5] * n6f;

                                let p = Point3::from_vec(r_par);
                                let idx = [n1, n2, n3, n4, n5, n6];

                                let current_idx = atoms.len();
                                atom_map.insert(idx, current_idx);
                                atoms.push(p);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut edges = Vec::new();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); atoms.len()];

    // To avoid duplicating edges in visual list, we can use a set of sorted tuples
    // Or just iterate and add if neighbor_idx > current_idx (for undirected graph visual)
    // But adj list needs both directions.

    // Reconstruct the loop over atoms using the map keys is slow? No, map iteration is fine.
    // Wait, HashMap iteration order is arbitrary. But we have `atoms` vector which is ordered by insertion (roughly).
    // We need to iterate `atom_map` to access the `[i32; 6]` coordinate.
    // Or store coord in struct?
    // Let's iterate `atom_map`.

    for (coord, &current_idx) in &atom_map {
        let current_pos = atoms[current_idx];

        // Check 12 neighbors in 6D grid (+/- 1 along each axis)
        // Actually, only +1 and -1 for each of 6 dimensions.
        for k in 0..6 {
            for sign in [-1, 1] {
                let mut neighbor_coord = *coord;
                neighbor_coord[k] += sign;

                if let Some(&neighbor_idx) = atom_map.get(&neighbor_coord) {
                    // Add to Adjacency List
                    adj[current_idx].push(neighbor_idx);

                    // Add to Visual Edges (only once per pair)
                    // Convention: from smaller index to larger index
                    if current_idx < neighbor_idx {
                        let neighbor_pos = atoms[neighbor_idx];
                        edges.push((current_pos, neighbor_pos));
                    }
                }
            }
        }
    }

    // Sort adjacency lists for determinism (optional but good for debugging)
    for neighbors in &mut adj {
        neighbors.sort();
        neighbors.dedup(); // Should not have dups logic above, but safe.
    }

    Quasicrystal { atoms, edges, adj }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_connectivity() {
        let qc = generate_icosahedral_lattice(2);
        assert!(!qc.atoms.is_empty());
        assert!(!qc.edges.is_empty());
        assert_eq!(qc.adj.len(), qc.atoms.len());

        // Check symmetry
        for (i, neighbors) in qc.adj.iter().enumerate() {
            for &n in neighbors {
                assert!(qc.adj[n].contains(&i), "Graph must be undirected");
            }
        }

        println!("Generated {} atoms with radius 2", qc.atoms.len());
        println!("Generated {} edges with radius 2", qc.edges.len());
    }
}
