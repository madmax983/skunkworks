mod mesh_gen;
mod pbd;

use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use miller_lattice::Crystal;
use pbd::Constraint;

#[macroquad::main("Origami Lattice: Codebase Morphogenesis")]
async fn main() {
    if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        return;
    }

    let rows = 15;
    let cols = 15;
    let mesh_data = mesh_gen::generate_miura_ori(rows, cols);
    let mut system = mesh_data.system;
    let indices = mesh_data.indices;

    // Load the codebase hierarchy as a 3D crystal structure
    // We load the current directory (the workspace root)
    let crystal = Crystal::build_from_path(std::path::Path::new(".")).unwrap_or_default();

    // The crystal spans a large coordinate space. We need to normalize/scale it to fit on our paper.
    // Find min/max bounds
    let mut min_pos = vec3(f32::MAX, f32::MAX, f32::MAX);
    let mut max_pos = vec3(f32::MIN, f32::MIN, f32::MIN);

    for atom in &crystal.atoms {
        let p = vec3(
            atom.position.x as f32,
            atom.position.y as f32,
            atom.position.z as f32,
        );
        min_pos = min_pos.min(p);
        max_pos = max_pos.max(p);
    }

    let bounds_size = max_pos - min_pos;
    let max_bound = bounds_size.x.max(bounds_size.y).max(bounds_size.z).max(1.0);
    // Scale down to a size roughly matching the paper (e.g. 20.0 units wide)
    let scale_factor = 20.0 / max_bound;

    // Center offset
    let offset = (max_pos + min_pos) * 0.5;

    // Precompute scaled crystal positions
    let scaled_crystal_positions: Vec<Vec3> = crystal
        .atoms
        .iter()
        .map(|atom| {
            let p = vec3(
                atom.position.x as f32,
                atom.position.y as f32,
                atom.position.z as f32,
            );
            (p - offset) * scale_factor
        })
        .collect();

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 30.0;

    let mut last_mouse_pos = mouse_position();

    loop {
        let dt = 0.016;

        // Input
        let mouse_pos = mouse_position();
        let delta = vec2(
            mouse_pos.0 - last_mouse_pos.0,
            mouse_pos.1 - last_mouse_pos.1,
        );
        last_mouse_pos = mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        // Apply Codebase Structure to Mesh Constraints
        // We calculate spatial density of crystal atoms projected onto the X-Z plane of the paper.
        let mut density = vec![0.0f32; system.particles.len()];

        for c_pos in &scaled_crystal_positions {
            // Project to 2D
            let bpos = vec3(c_pos.x, 0.0, c_pos.y); // Mapping crystal X-Y to paper X-Z
            for (idx, particle) in system.particles.iter().enumerate() {
                // Check distance
                let dist_sq = (particle.pos.x - bpos.x).powi(2) + (particle.pos.z - bpos.z).powi(2);
                if dist_sq < 9.0 {
                    density[idx] += 1.0 / (1.0 + dist_sq);
                }
            }
        }

        for constraint in &mut system.constraints {
            match constraint {
                Constraint::Distance {
                    p1,
                    p2,
                    ref mut stiffness,
                    ..
                } => {
                    let d1 = density[*p1];
                    let d2 = density[*p2];
                    if d1 > 0.0 || d2 > 0.0 {
                        // Flocking density stiffens the paper locally
                        *stiffness = (0.5 + (d1 + d2) * 0.1).min(1.0);
                    } else {
                        *stiffness = 0.5; // default
                    }
                }
                Constraint::Actuator {
                    p1,
                    p2,
                    ref mut factor,
                    ..
                } => {
                    let d1 = density[*p1];
                    let d2 = density[*p2];
                    if d1 > 0.0 || d2 > 0.0 {
                        // Force fold: Close the crease (factor -> 0.0) based on lattice density
                        let target_fold = 0.0;
                        let strength = ((d1 + d2) * 0.1).min(1.0);
                        *factor = *factor * (1.0 - strength) + target_fold * strength;
                    } else {
                        // Relax towards open
                        *factor = *factor * 0.95 + 1.0 * 0.05;
                    }
                }
                _ => {}
            }
        }

        // Physics Step
        system.step(dt, 10);

        // Rendering
        clear_background(BLACK);

        let cam_pos = vec3(
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Crystal Lattice hovering above the paper
        let y_offset = 5.0;
        for c_pos in &scaled_crystal_positions {
            // Map lattice X-Y to 3D X-Z space above paper
            let pos3d = vec3(c_pos.x, c_pos.z + y_offset, c_pos.y);
            draw_sphere(pos3d, 0.2, None, Color::new(0.0, 1.0, 0.5, 0.5));
        }

        for &(p_idx, c_idx) in &crystal.bonds {
            let p1 = scaled_crystal_positions[p_idx];
            let p2 = scaled_crystal_positions[c_idx];
            let pos1 = vec3(p1.x, p1.z + y_offset, p1.y);
            let pos2 = vec3(p2.x, p2.z + y_offset, p2.y);
            draw_line_3d(pos1, pos2, Color::new(0.0, 1.0, 0.5, 0.2));
        }

        // Draw Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in (0..indices.len()).step_by(3) {
            let idx0 = indices[i] as usize;
            let idx1 = indices[i + 1] as usize;
            let idx2 = indices[i + 2] as usize;

            let v0 = system.particles[idx0].pos;
            let v1 = system.particles[idx1].pos;
            let v2 = system.particles[idx2].pos;

            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let light_dir = vec3(0.5, 1.0, 0.5).normalize();
            let intensity = normal.dot(light_dir).abs() * 0.7 + 0.3;

            let avg_d = (density[idx0] + density[idx1] + density[idx2]) / 3.0;

            let r = (0.2 + avg_d * 0.5).min(1.0);
            let g = (0.2 + avg_d * 0.5).min(1.0);
            let b = (0.8 - avg_d * 0.2).max(0.0);

            let color = Color::new(r * intensity, g * intensity, b * intensity, 1.0);

            let start_idx = mesh.vertices.len() as u16;
            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);

            let color_bytes: [u8; 4] = color.into();

            mesh.vertices.push(Vertex {
                position: v0,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v1,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v2,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);
        }

        draw_mesh(&mesh);

        set_default_camera();

        draw_text("Origami Lattice", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Codebase Nodes: {}", scaled_crystal_positions.len()),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Codebase density actively crumples the mesh",
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
