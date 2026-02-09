use crate::pbd::PbdSystem;
use macroquad::prelude::*;

pub struct MeshData {
    pub system: PbdSystem,
    pub indices: Vec<u16>, // For rendering triangles
    pub actuators: Vec<usize>, // Indices into constraints
}

pub fn generate_miura_ori(rows: usize, cols: usize) -> MeshData {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators = Vec::new();

    let a = 1.0; // Edge length X
    let b = 1.0; // Edge length Y
    let angle = 80.0f32.to_radians(); // Parallelogram angle

    // Create particles
    // Grid: (cols+1) x (rows+1)
    for i in 0..=cols {
        for j in 0..=rows {
            let x = (i as f32) * a * angle.sin();
            let offset = if i % 2 == 1 { a * angle.cos() } else { 0.0 };
            let y = (j as f32) * b + offset;

            // Center the mesh
            let cx = (cols as f32 * a * angle.sin()) / 2.0;
            let cy = (rows as f32 * b) / 2.0;

            system.add_particle(vec3(x - cx, y - cy, 0.0), 1.0);
        }
    }

    let stiffness = 1.0;

    // Structural Constraints & Triangles
    for i in 0..cols {
        for j in 0..rows {
            let p00 = i * (rows + 1) + j;
            let p01 = i * (rows + 1) + (j + 1);
            let p10 = (i + 1) * (rows + 1) + j;
            let p11 = (i + 1) * (rows + 1) + (j + 1);

            // Triangles: Split along p01-p10 diagonal
            indices.push(p00 as u16); indices.push(p01 as u16); indices.push(p10 as u16);
            indices.push(p10 as u16); indices.push(p01 as u16); indices.push(p11 as u16);

            // Edges (Structural)
            // We add all edges of the quad + diagonal.
            // PBD allows duplicate constraints, it just converges faster/stiffer.
            // For efficiency, we could check if edge exists, but for this size it's fine.

            system.add_distance_constraint(p00, p01, stiffness); // Left
            system.add_distance_constraint(p00, p10, stiffness); // Top
            system.add_distance_constraint(p10, p11, stiffness); // Right
            system.add_distance_constraint(p01, p11, stiffness); // Bottom
            system.add_distance_constraint(p01, p10, stiffness); // Diagonal
        }
    }

    // Actuators (Creases)

    // Vertical Creases (spanning i-1 to i+1)
    for i in 1..cols {
        for j in 0..=rows {
            let p_left = (i - 1) * (rows + 1) + j;
            let p_right = (i + 1) * (rows + 1) + j;

            let dist = system.particles[p_left].pos.distance(system.particles[p_right].pos);
            let folded_dist = dist * 0.2;

            // Mountain vs Valley assignment?
            // In this simulation, we just control the fold amount (0% to 100%).
            // The "direction" (M vs V) is determined by the initial perturbation or explicit bias.
            // To ensure it folds correctly, we might need to nudge vertices z-wise.
            // But let's see if just contracting the span works.

            system.add_actuator_constraint(p_left, p_right, folded_dist, dist, 0.5); // Lower stiffness for actuators
            actuators.push(system.constraints.len() - 1);
        }
    }

    // Horizontal Creases (spanning j-1 to j+1)
    for i in 0..=cols {
        for j in 1..rows {
            let p_top = i * (rows + 1) + (j - 1);
            let p_bottom = i * (rows + 1) + (j + 1);

            let dist = system.particles[p_top].pos.distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.2;

            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.5);
            actuators.push(system.constraints.len() - 1);
        }
    }

    // Pin center
    let center_idx = (cols / 2) * (rows + 1) + (rows / 2);
    let center_pos = system.particles[center_idx].pos;
    system.add_pin_constraint(center_idx, center_pos);

    // Perturb vertices slightly in Z to break symmetry and allow folding
    // M/V pattern:
    // Vertical lines i: M if i%2==0, V if i%2==1 (or vice versa)
    // Horizontal lines j: M

    // We can pre-bend the mesh slightly by modifying z positions.
    for i in 0..=cols {
        for j in 0..=rows {
            let idx = i * (rows + 1) + j;
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
