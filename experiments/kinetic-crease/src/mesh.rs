use crate::physics::{Solver, Particle, Constraint};
use nalgebra::Vector3;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FoldAssignment {
    Mountain,
    Valley,
    Flat,
}

#[derive(Clone, Debug)]
pub struct HingeData {
    pub constraint_idx: usize,
    pub h1: f32,
    pub h2: f32,
    pub assignment: FoldAssignment,
    pub max_angle: f32, // The target angle when fully folded (e.g., 0.1 for tight fold)
}

pub struct CreaseMesh {
    pub indices: Vec<u16>, // Triangle indices for rendering
    pub hinges: Vec<HingeData>,
}

impl CreaseMesh {
    pub fn update_constraints(&self, solver: &mut Solver, fold_factor: f32) {
        // fold_factor: 0.0 (Flat) -> 1.0 (Fully Folded)
        for hinge in &self.hinges {
            if let Some(Constraint::Hinge { target_len, .. }) = solver.constraints.get_mut(hinge.constraint_idx) {
                // Flat = PI (180 degrees).
                // We map fold_factor to an angle deviation from PI.

                // For distance constraints, we can't distinguish M/V easily without signed distance or normals.
                // However, the target distance is symmetric for deviation +alpha or -alpha.
                // d^2 = h1^2 + h2^2 - 2*h1*h2*cos(theta)

                // We define theta as the internal angle.
                // Flat = PI.
                // Folded = PI * (1.0 - fold_factor * 0.8) ? (So it goes to ~36 deg)
                // We don't need to distinguish M/V for the distance formula,
                // BUT we rely on initial perturbation to ensure it buckles the right way.

                let target_angle = PI * (1.0 - fold_factor * 0.9); // Folds down to ~18 degrees

                let h1 = hinge.h1;
                let h2 = hinge.h2;

                // Law of cosines
                let d_sq = h1*h1 + h2*h2 - 2.0*h1*h2 * target_angle.cos();
                *target_len = d_sq.sqrt();
            }
        }
    }
}

pub fn generate_miura_ori(solver: &mut Solver, rows: usize, cols: usize, cell_w: f32, cell_h: f32, angle_deg: f32) -> CreaseMesh {
    let mut indices = Vec::new();
    let mut hinges = Vec::new();

    let mut grid_ids = vec![vec![0; cols + 1]; rows + 1];

    // 1. Create Particles
    let alpha = angle_deg.to_radians();
    let s = cell_w * alpha.tan(); // vertical shift for zig-zag

    for r in 0..=rows {
        for c in 0..=cols {
            // Zig-Zag logic
            let x = c as f32 * cell_w;
            let mut y = r as f32 * cell_h;

            // Shift odd columns
            if c % 2 == 1 {
                y += s;
            }

            // Perturb Z based on M/V pattern to guide folding
            // Miura pattern:
            // Horizontals: M, V, M, V...
            // Verticals: M, M, M... (or V, V, V)
            // We use a checkerboard perturbation
            let z_perturb = if (r + c) % 2 == 0 { 0.1 } else { -0.1 };

            let p = Particle::new(x - (cols as f32 * cell_w)/2.0, y - (rows as f32 * cell_h)/2.0, z_perturb, 1.0);
            grid_ids[r][c] = solver.add_particle(p);
        }
    }

    // 2. Build Connectivity & Constraints
    let stiffness_structural = 1.0;
    let stiffness_hinge = 0.5;

    // Helper to add structural edge
    let mut add_edge = |solver: &mut Solver, p1: usize, p2: usize| {
        solver.add_distance_constraint(p1, p2, stiffness_structural);
    };

    for r in 0..rows {
        for c in 0..cols {
            let p00 = grid_ids[r][c];
            let p10 = grid_ids[r][c+1];
            let p01 = grid_ids[r+1][c];
            let p11 = grid_ids[r+1][c+1];

            // Quad Faces: (p00, p10, p11, p01)
            // Edges
            add_edge(solver, p00, p10); // Top
            add_edge(solver, p10, p11); // Right
            add_edge(solver, p11, p01); // Bottom
            add_edge(solver, p01, p00); // Left

            // Triangulate to make rigid: Split along diagonal p00-p11
            add_edge(solver, p00, p11); // Diagonal

            // Add render indices (counter-clockwise)
            // Tri 1: 00, 10, 11
            indices.push(p00 as u16);
            indices.push(p10 as u16);
            indices.push(p11 as u16);

            // Tri 2: 00, 11, 01
            indices.push(p00 as u16);
            indices.push(p11 as u16);
            indices.push(p01 as u16);
        }
    }

    // 3. Add Hinges
    // A hinge is between two triangles sharing an edge.
    // We have horizontal edges, vertical edges, and diagonal edges (internal to quads).
    // The internal diagonal edges are "Flat" assignment (fixed at 180 or 0? No, they are flat 180).
    // Actually, we don't need hinge constraints for the internal diagonals because the distance constraint on the cross-diagonal (01-10) would enforce planarity if we added it.
    // But we didn't add the cross-diagonal. We added p00-p11.
    // So the quad can fold along p00-p11.
    // To make the face rigid, we need to enforce the other diagonal too?
    // OR, we just add a "Flat" hinge constraint on p00-p11 with target angle 180.
    // Let's add the other diagonal distance constraint to be safe. It makes the quad a rigid body.

    // Refined Structural:
    for r in 0..rows {
        for c in 0..cols {
            let p00 = grid_ids[r][c];
            let p10 = grid_ids[r][c+1];
            let p01 = grid_ids[r+1][c];
            let p11 = grid_ids[r+1][c+1];
            // Cross-brace
            add_edge(solver, p10, p01);
        }
    }

    // Now faces are rigid. We only need hinges on the grid lines.

    // Horizontal Hinges (between (r, c)-(r, c+1) and (r+1, c)-(r+1, c+1))
    // Wait, horizontal grid lines are edges between row r and row r-1.
    // Let's iterate internal horizontal edges.
    for r in 1..rows {
        for c in 0..cols {
            // Edge is (r, c) -> (r, c+1)
            // Neighbors are (r-1, c), (r-1, c+1) [Top Quad]
            // And (r+1, c), (r+1, c+1) [Bottom Quad]

            // The edge (r,c)-(r,c+1) is shared by two triangles?
            // Depends on triangulation.
            // My triangulation: split p00-p11 (Top-Left to Bottom-Right).

            // In row r-1: Quads are (r-1, c). Diag is (r-1, c) -> (r, c+1).
            // So edge (r, c) -> (r, c+1) belongs to triangle (r-1, c), (r, c), (r, c+1)? No.
            // Tri 2 of row r-1 is (r-1, c), (r, c+1), (r, c).
            // Yes. Shared edge is (r,c)-(r,c+1).
            // Wing in top is (r-1, c)? No, (r-1, c) is p00. p01 is (r,c). p11 is (r,c+1).
            // Tri 2 is (00, 11, 01) => ((r-1,c), (r,c+1), (r,c)).
            // So wing is (r-1,c).

            // In row r: Quad (r, c). Diag is (r, c) -> (r+1, c+1).
            // Tri 1 is (r, c), (r, c+1), (r+1, c+1).
            // Shared edge is (r,c)-(r,c+1).
            // Wing is (r+1, c+1).

            let p_shared1 = grid_ids[r][c];
            let p_shared2 = grid_ids[r][c+1];
            let p_wing1 = grid_ids[r-1][c];
            let p_wing2 = grid_ids[r+1][c+1];

            // Compute altitudes for hinge constraint
            // Need perpendicular distance from wing to axis.
            // Since faces are rigid and we initialized them, we can compute this from initial pos.
            let h1 = point_line_distance(solver.particles[p_wing1].pos, solver.particles[p_shared1].pos, solver.particles[p_shared2].pos);
            let h2 = point_line_distance(solver.particles[p_wing2].pos, solver.particles[p_shared1].pos, solver.particles[p_shared2].pos);

            solver.add_hinge_constraint(p_wing1, p_wing2, stiffness_hinge);
            let c_idx = solver.constraints.len() - 1;

            hinges.push(HingeData {
                constraint_idx: c_idx,
                h1,
                h2,
                assignment: if r % 2 == 0 { FoldAssignment::Mountain } else { FoldAssignment::Valley },
                max_angle: 0.1,
            });
        }
    }

    // Vertical Hinges (between (r, c)-(r+1, c))
    for r in 0..rows {
        for c in 1..cols {
            // Edge (r,c)-(r+1,c).
            // Neighbors: col c-1 (Left), col c (Right).

            // Col c-1: Quad (r, c-1). Diag (r, c-1)->(r+1, c).
            // Tri 1: (r, c-1), (r, c), (r+1, c).
            // Shared edge: (r, c)-(r+1, c).
            // Wing: (r, c-1).

            // Col c: Quad (r, c). Diag (r, c)->(r+1, c+1).
            // Tri 2: (r, c), (r+1, c+1), (r+1, c).
            // Shared edge: (r, c)-(r+1, c).
            // Wing: (r+1, c+1).

            let p_shared1 = grid_ids[r][c];
            let p_shared2 = grid_ids[r+1][c];
            let p_wing1 = grid_ids[r][c-1];
            let p_wing2 = grid_ids[r+1][c+1];

            let h1 = point_line_distance(solver.particles[p_wing1].pos, solver.particles[p_shared1].pos, solver.particles[p_shared2].pos);
            let h2 = point_line_distance(solver.particles[p_wing2].pos, solver.particles[p_shared1].pos, solver.particles[p_shared2].pos);

            solver.add_hinge_constraint(p_wing1, p_wing2, stiffness_hinge);
            let c_idx = solver.constraints.len() - 1;

            hinges.push(HingeData {
                constraint_idx: c_idx,
                h1,
                h2,
                assignment: FoldAssignment::Mountain, // Verticals are all Mountain in Miura
                max_angle: 0.1,
            });
        }
    }

    CreaseMesh { indices, hinges }
}

fn point_line_distance(p: Vector3<f32>, a: Vector3<f32>, b: Vector3<f32>) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let proj = ap.dot(&ab) / ab.dot(&ab);
    let closest = a + ab * proj;
    (p - closest).magnitude()
}
