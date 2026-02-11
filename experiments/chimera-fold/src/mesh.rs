use crate::pbd::PbdSystem;
use macroquad::prelude::*;

// Adapted from origami-constellation/src/mesh_gen.rs and neuro-fold/src/main.rs
pub fn generate_miura_ori(rows: usize, cols: usize) -> (PbdSystem, Vec<u16>, Vec<usize>, Vec<usize>) {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators_v = Vec::new();
    let mut actuators_h = Vec::new();

    let a = 2.0; // Scale up
    let b = 2.0;
    let angle = 80.0f32.to_radians();

    // Create particles
    for i in 0..=cols {
        for j in 0..=rows {
            let x = (i as f32) * a * angle.sin();
            let offset = if i % 2 == 1 { a * angle.cos() } else { 0.0 };
            let y = (j as f32) * b + offset;

            let cx = (cols as f32 * a * angle.sin()) / 2.0;
            let cy = (rows as f32 * b) / 2.0;

            // Add some Z curvature to make it look like a creature
            let z = (i as f32 * 0.5).sin() * 2.0;

            system.add_particle(vec3(x - cx, y - cy, z), 1.0);
        }
    }

    let stiffness = 1.0;

    // Constraints & Triangles
    for i in 0..cols {
        for j in 0..rows {
            let p00 = i * (rows + 1) + j;
            let p01 = i * (rows + 1) + (j + 1);
            let p10 = (i + 1) * (rows + 1) + j;
            let p11 = (i + 1) * (rows + 1) + (j + 1);

            // Triangles
            indices.push(p00 as u16);
            indices.push(p01 as u16);
            indices.push(p10 as u16);
            indices.push(p10 as u16);
            indices.push(p01 as u16);
            indices.push(p11 as u16);

            // Structural Edges
            system.add_distance_constraint(p00, p01, stiffness);
            system.add_distance_constraint(p00, p10, stiffness);
            system.add_distance_constraint(p10, p11, stiffness);
            system.add_distance_constraint(p01, p11, stiffness);
            system.add_distance_constraint(p01, p10, stiffness);
        }
    }

    // Actuators (Creases)

    // Vertical Creases (The "V" folds) - These drive the expansion/contraction
    for i in 1..cols {
        for j in 0..=rows {
            let p_left = (i - 1) * (rows + 1) + j;
            let p_right = (i + 1) * (rows + 1) + j;

            let dist = system.particles[p_left]
                .pos
                .distance(system.particles[p_right].pos);
            let folded_dist = dist * 0.1; // Deep fold

            system.add_actuator_constraint(p_left, p_right, folded_dist, dist, 0.2); // Low stiffness for compliance
            actuators_v.push(system.constraints.len() - 1);
        }
    }

    // Horizontal Creases
    for i in 0..=cols {
        for j in 1..rows {
            let p_top = i * (rows + 1) + (j - 1);
            let p_bottom = i * (rows + 1) + (j + 1);

            let dist = system.particles[p_top]
                .pos
                .distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.1;

            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.2);
            actuators_h.push(system.constraints.len() - 1);
        }
    }

    // Pin the head (left side) so it doesn't float away
    let head_idx = (rows + 1) / 2;
    let head_pos = system.particles[head_idx].pos;
    system.add_pin_constraint(head_idx, head_pos);

    (system, indices, actuators_v, actuators_h)
}
