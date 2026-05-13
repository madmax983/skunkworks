use crate::pbd::PbdSystem;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};

pub struct MeshData {
    pub system: PbdSystem,
    pub indices: Vec<u16>,     // For rendering triangles
    pub actuators: Vec<usize>, // Indices into constraints
}

pub fn generate_miura_ori(rows: usize, cols: usize) -> MeshData {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators = Vec::new();

    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Vertical,
    };

    // Generate initial flat positions (expansion = 1.0)
    // Note: generate_miura_grid returns vertices in row-major order (j outer, i inner)
    let vertices = generate_miura_grid(params, (cols, rows), 1.0);

    for v in vertices {
        system.add_particle(v, 1.0);
    }

    let stiffness = 1.0;
    let width = cols + 1; // Vertices per row

    // Structural Constraints & Triangles
    for j in 0..rows {
        for i in 0..cols {
            // Row-major indexing: j * width + i
            let p00 = j * width + i; // (i, j)
            let p10 = j * width + (i + 1); // (i+1, j)
            let p01 = (j + 1) * width + i; // (i, j+1)
            let p11 = (j + 1) * width + (i + 1); // (i+1, j+1)

            // Triangles: Split along p01-p10 diagonal (matching original connectivity)
            // Original used p01-p10 diagonal split.
            // p00(i,j), p01(i, j+1), p10(i+1, j), p11(i+1, j+1)

            // Triangle 1: p00, p01, p10
            indices.push(p00 as u16);
            indices.push(p01 as u16);
            indices.push(p10 as u16);

            // Triangle 2: p10, p01, p11
            indices.push(p10 as u16);
            indices.push(p01 as u16);
            indices.push(p11 as u16);

            // Edges (Structural)
            system.add_distance_constraint(p00, p01, stiffness); // Vertical Left
            system.add_distance_constraint(p00, p10, stiffness); // Horizontal Top
            system.add_distance_constraint(p10, p11, stiffness); // Vertical Right
            system.add_distance_constraint(p01, p11, stiffness); // Horizontal Bottom
            system.add_distance_constraint(p01, p10, stiffness); // Diagonal
        }
    }

    // Actuators (Creases)

    // Vertical Creases (spanning i-1 to i+1)
    // For each row j, and each internal column i (1..cols)
    for j in 0..=rows {
        for i in 1..cols {
            let p_left = j * width + (i - 1);
            let p_right = j * width + (i + 1);

            let dist = system.particles[p_left]
                .pos
                .distance(system.particles[p_right].pos);
            let folded_dist = dist * 0.2;

            system.add_actuator_constraint(p_left, p_right, folded_dist, dist, 0.5);
            actuators.push(system.constraints.len() - 1);
        }
    }

    // Horizontal Creases (spanning j-1 to j+1)
    // For each column i, and each internal row j (1..rows)
    for j in 1..rows {
        for i in 0..=cols {
            let p_top = (j - 1) * width + i;
            let p_bottom = (j + 1) * width + i;

            let dist = system.particles[p_top]
                .pos
                .distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.2;

            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.5);
            actuators.push(system.constraints.len() - 1);
        }
    }

    // Pin center
    let center_idx = (rows / 2) * width + (cols / 2);
    let center_pos = system.particles[center_idx].pos;
    system.add_pin_constraint(center_idx, center_pos);

    // Perturb vertices slightly in Z to break symmetry and allow folding
    for j in 0..=rows {
        for i in 0..=cols {
            let idx = j * width + i;
            let mut z_offset = 0.0;

            // Simple zigzag perturbation
            if i % 2 == 0 {
                z_offset += 0.1;
            } else {
                z_offset -= 0.1;
            }

            if j % 2 == 0 {
                z_offset += 0.05;
            } else {
                z_offset -= 0.05;
            }

            system.particles[idx].pos.z += z_offset;
            system.particles[idx].prev_pos.z += z_offset;
        }
    }

    MeshData {
        system,
        indices,
        actuators,
    }
}
