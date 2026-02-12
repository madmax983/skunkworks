use crate::pbd::PbdSystem;
use macroquad::prelude::*;

pub struct MeshData {
    pub system: PbdSystem,
    pub indices: Vec<u16>,     // For rendering triangles
    pub actuators: Vec<usize>, // Indices into constraints
}

pub fn generate_miura_ori(rows: usize, cols: usize) -> MeshData {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators = Vec::new();

    append_miura_ori(
        &mut system,
        &mut indices,
        &mut actuators,
        Vec3::ZERO,
        rows,
        cols,
    );

    // Pin center
    let center_idx = (cols / 2) * (rows + 1) + (rows / 2);
    let center_pos = system.particles[center_idx].pos;
    system.add_pin_constraint(center_idx, center_pos);

    MeshData {
        system,
        indices,
        actuators,
    }
}

pub fn append_miura_ori(
    system: &mut PbdSystem,
    indices: &mut Vec<u16>,
    actuators: &mut Vec<usize>,
    origin: Vec3,
    rows: usize,
    cols: usize,
) {
    let a = 1.0; // Edge length X
    let b = 1.0; // Edge length Y
    let angle = 80.0f32.to_radians(); // Parallelogram angle

    let start_idx = system.particles.len();

    // Create particles
    // Grid: (cols+1) x (rows+1)
    for i in 0..=cols {
        for j in 0..=rows {
            let x = (i as f32) * a * angle.sin();
            let offset = if i % 2 == 1 { a * angle.cos() } else { 0.0 };
            let y = (j as f32) * b + offset;

            // Center the mesh locally relative to origin
            let cx = (cols as f32 * a * angle.sin()) / 2.0;
            let cy = (rows as f32 * b) / 2.0;

            let mut pos = origin + vec3(x - cx, y - cy, 0.0);

             // Perturb vertices slightly in Z to break symmetry
            let mut z_offset = 0.0;
            if i % 2 == 0 { z_offset += 0.1; } else { z_offset -= 0.1; }
            if j % 2 == 0 { z_offset += 0.05; } else { z_offset -= 0.05; }
            pos.z += z_offset;

            system.add_particle(pos, 1.0);
        }
    }

    let stiffness = 1.0;

    // Structural Constraints & Triangles
    for i in 0..cols {
        for j in 0..rows {
            let p00 = start_idx + i * (rows + 1) + j;
            let p01 = start_idx + i * (rows + 1) + (j + 1);
            let p10 = start_idx + (i + 1) * (rows + 1) + j;
            let p11 = start_idx + (i + 1) * (rows + 1) + (j + 1);

            // Triangles: Split along p01-p10 diagonal
            indices.push(p00 as u16);
            indices.push(p01 as u16);
            indices.push(p10 as u16);
            indices.push(p10 as u16);
            indices.push(p01 as u16);
            indices.push(p11 as u16);

            system.add_distance_constraint(p00, p01, stiffness); // Left
            system.add_distance_constraint(p00, p10, stiffness); // Top
            system.add_distance_constraint(p10, p11, stiffness); // Right
            system.add_distance_constraint(p01, p11, stiffness); // Bottom
            system.add_distance_constraint(p01, p10, stiffness); // Diagonal
        }
    }

    // Actuators
    // Vertical Creases
    for i in 1..cols {
        for j in 0..=rows {
            let p_left = start_idx + (i - 1) * (rows + 1) + j;
            let p_right = start_idx + (i + 1) * (rows + 1) + j;
            let dist = system.particles[p_left].pos.distance(system.particles[p_right].pos);
            let folded_dist = dist * 0.2;
            system.add_actuator_constraint(p_left, p_right, folded_dist, dist, 0.5);
            actuators.push(system.constraints.len() - 1);
        }
    }

    // Horizontal Creases
    for i in 0..=cols {
        for j in 1..rows {
            let p_top = start_idx + i * (rows + 1) + (j - 1);
            let p_bottom = start_idx + i * (rows + 1) + (j + 1);
            let dist = system.particles[p_top].pos.distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.2;
            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.5);
            actuators.push(system.constraints.len() - 1);
        }
    }
}

pub fn generate_yoshimura(segments: usize, radius: f32) -> MeshData {
     let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators = Vec::new();

    append_yoshimura(&mut system, &mut indices, &mut actuators, Vec3::ZERO, segments, radius);

    MeshData {
        system, indices, actuators
    }
}

pub fn append_yoshimura(
    system: &mut PbdSystem,
    indices: &mut Vec<u16>,
    actuators: &mut Vec<usize>,
    origin: Vec3,
    segments: usize,
    radius: f32,
) {
    let n = 6;
    let height_per_segment = 1.5;
    let start_idx = system.particles.len();
    let stiffness = 1.0;

    for s in 0..=segments {
        let z = s as f32 * height_per_segment;
        let offset_angle = if s % 2 == 0 { 0.0 } else { std::f32::consts::PI / n as f32 };

        for i in 0..n {
            let angle = (i as f32 * 2.0 * std::f32::consts::PI / n as f32) + offset_angle;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            // Align cylinder along Y for consistency with world up?
            // Or Z? The previous code used Z for offset and Y for "up" in camera?
            // "draw_line_3d(*star, *star + vec3(0.1, 0.0, 0.0), WHITE);" -> stars are points.
            // Camera position: y is up.
            // So cylinder should be along Y or X?
            // Let's align along X for "deployment" or Z.
            // Let's stick to the generated coordinates: x, z, y.
            // So it's along Y axis (z is height in loop var, but I assigned it to Y in vec3? no)
            // system.add_particle(vec3(x, z, y), 1.0); -> x, y=z, z=y?
            // x = radius*cos, y = height, z = radius*sin.
            // This aligns cylinder along Y.
            system.add_particle(origin + vec3(x, z, y), 1.0);
        }
    }

     // Triangulation
    for s in 0..segments {
        let offset = start_idx + s * n;
        let next_offset = start_idx + (s + 1) * n;

        for i in 0..n {
            let p_curr = offset + i;

            let (p_a, p_b) = if s % 2 == 0 {
                 let p_up = next_offset + i;
                 let p_up_prev = next_offset + (if i == 0 { n - 1 } else { i - 1 });
                 (p_up, p_up_prev)
            } else {
                 let p_up = next_offset + i;
                 let p_up_next = next_offset + (i + 1) % n;
                 (p_up, p_up_next)
            };

            indices.push(p_curr as u16);
            indices.push(p_b as u16);
            indices.push(p_a as u16);

            system.add_distance_constraint(p_curr, p_a, stiffness);
            system.add_distance_constraint(p_curr, p_b, stiffness);
            system.add_distance_constraint(p_a, p_b, stiffness);
        }
    }

    // Bottom ring horizontals
    for i in 0..n {
        let p = start_idx + i;
        let p_next = start_idx + (i + 1) % n;
        system.add_distance_constraint(p, p_next, stiffness);
    }

     // Actuators: Compress along Axis (Y)
    for s in 0..segments {
         if s + 2 <= segments {
             for i in 0..n {
                 let p_start = start_idx + s * n + i;
                 let p_end = start_idx + (s + 2) * n + i;
                 let dist = system.particles[p_start].pos.distance(system.particles[p_end].pos);
                 let folded_dist = dist * 0.1;
                 system.add_actuator_constraint(p_start, p_end, folded_dist, dist, 0.5);
                 actuators.push(system.constraints.len() - 1);
             }
         }
    }

    // Pin bottom ring
    for i in 0..n {
        let p = start_idx + i;
        system.add_pin_constraint(p, system.particles[p].pos);
    }
}

pub fn generate_solar_array() -> MeshData {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators = Vec::new();

    let half_size = 2.0;

    let offsets = [
        vec3(-1., -1., -1.), vec3(1., -1., -1.), vec3(1., 1., -1.), vec3(-1., 1., -1.),
        vec3(-1., -1., 1.), vec3(1., -1., 1.), vec3(1., 1., 1.), vec3(-1., 1., 1.),
    ];

    // Hub Particles
    let start_hub = system.particles.len();
    for off in offsets {
        system.add_particle(off * half_size, 1.0);
    }
    // Pin Hub
    for i in 0..8 {
        let idx = start_hub + i;
        system.add_pin_constraint(idx, system.particles[idx].pos);
    }

    // Add Hub triangles (Cube)
    // Front: 0,1,2,3 (indices into offsets)
    // 0 (-,-,-), 1 (+,-,-), 2 (+,+,-), 3 (-,+,-) -> Z=-1 face (Back actually in OpenGL, but whatever)
    // Let's just add indices.
    let cube_indices = [
        0, 1, 2, 0, 2, 3, // Back
        4, 5, 6, 4, 6, 7, // Front
        0, 4, 7, 0, 7, 3, // Left
        1, 5, 6, 1, 6, 2, // Right
        3, 2, 6, 3, 6, 7, // Top
        0, 1, 5, 0, 5, 4, // Bottom
    ];
    for &idx in &cube_indices {
        indices.push((start_hub + idx) as u16);
    }

    // Left Wing (-X)
    append_miura_ori(
        &mut system,
        &mut indices,
        &mut actuators,
        vec3(-7.0, 0.0, 0.0),
        5, 5
    );

    // Right Wing (+X)
    append_miura_ori(
        &mut system,
        &mut indices,
        &mut actuators,
        vec3(7.0, 0.0, 0.0),
        5, 5
    );

    MeshData {
        system, indices, actuators
    }
}
