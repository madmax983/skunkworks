//! # Git Origami
//!
//! A hybrid visualization crossing `git-associates` with `origami`.
//!
//! **Lineage:**
//! - `git-associates` provides the structured repository commit metadata.
//! - `origami` provides the procedural Miura-ori mesh structure.
//!
//! The discrete timeline of repository commits is mapped onto a continuous 2D plane
//! underlying a 3D soft-body mesh. When a commit occurs, it acts as a physical "tug"
//! on the paper constraints corresponding to the file's hash, causing the codebase's
//! history to literally crumple and fold the architecture over time.

use ::rand::{thread_rng, Rng};
use git_associates::GitModel;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::PbdSystem;

#[macroquad::main("Git Origami")]
async fn main() {
    let root_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    // Parse Git History (Inherited from git-associates)
    let git_model = match GitModel::open(&root_path) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to open git repository: {}", e);
            return;
        }
    };

    let history = match git_model.history_with_diffs(200) {
        Ok(h) => {
            let mut h = h;
            h.reverse(); // Process oldest to newest
            h
        }
        Err(e) => {
            println!("Failed to fetch git history: {}", e);
            return;
        }
    };

    println!("Fetched {} commits.", history.len());

    let cols = 15;
    let rows = 15;
    let w = cols + 1;

    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    // Generate mesh points (Inherited from origami)
    let points = generate_miura_grid(params, (cols, rows), 0.5);
    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    for pos in &points {
        let p_idx = system.add_particle(*pos, 1.0);
        p_indices.push(p_idx);

        let is_corner = p_indices.len() - 1 == 0
            || p_indices.len() - 1 == cols
            || p_indices.len() - 1 == rows * w
            || p_indices.len() - 1 == rows * w + cols;

        if is_corner {
            system.add_pin_constraint(p_idx, *pos);
        }
    }

    // Horizontal constraints
    for j in 0..=rows {
        for i in 0..cols {
            let p1 = p_indices[j * w + i];
            let p2 = p_indices[j * w + i + 1];
            let dist = points[j * w + i].distance(points[j * w + i + 1]);
            system.add_distance_constraint(p1, p2, dist);
        }
    }

    // Vertical constraints
    for j in 0..rows {
        for i in 0..=cols {
            let p1 = p_indices[j * w + i];
            let p2 = p_indices[(j + 1) * w + i];
            let dist = points[j * w + i].distance(points[(j + 1) * w + i]);
            system.add_distance_constraint(p1, p2, dist);
        }
    }

    // Structural/Bend constraints (diagonals)
    for j in 0..rows {
        for i in 0..cols {
            let p1 = p_indices[j * w + i];
            let p2 = p_indices[(j + 1) * w + i + 1];
            let dist = points[j * w + i].distance(points[(j + 1) * w + i + 1]);
            system.add_distance_constraint(p1, p2, dist);

            let p3 = p_indices[j * w + i + 1];
            let p4 = p_indices[(j + 1) * w + i];
            let dist = points[j * w + i + 1].distance(points[(j + 1) * w + i]);
            system.add_distance_constraint(p3, p4, dist);
        }
    }

    let mut commit_idx = 0;
    let mut frames_since_commit = 0;
    let frames_per_commit = 30; // Speed of history playback

    let mut rng = thread_rng();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let time = get_time() as f32;
        let cam_x = time.sin() * 15.0;
        let cam_z = time.cos() * 15.0;
        let cam_pos = vec3(cam_x, 15.0, cam_z);
        let center = vec3(0.0, 0.0, 0.0);

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: center,
            ..Default::default()
        });

        // Apply Git History Logic (Novel Trait)
        frames_since_commit += 1;
        if frames_since_commit >= frames_per_commit {
            frames_since_commit = 0;
            if commit_idx < history.len() {
                let commit = &history[commit_idx];

                // Actuate the mesh based on the commit
                // Let's create an impact force for every file changed
                for file in &commit.files {
                    let mut hash: usize = 0;
                    for byte in file.path.bytes() {
                        hash = hash.wrapping_add(byte as usize);
                    }

                    let target_idx = hash % p_indices.len();

                    // The force is proportional to insertions and deletions
                    let force = (file.insertions + file.deletions) as f32 * 0.1;

                    // Cap the force to prevent exploding the simulation
                    let force = force.min(5.0);

                    let direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

                    let p_id = p_indices[target_idx];
                    system.particles[p_id].pos += vec3(0.0, direction * force, 0.0);
                }

                commit_idx += 1;
            }
        }

        system.step(0.016, 5);

        // Draw the origami mesh
        let current_positions: Vec<Vec3> = p_indices
            .iter()
            .map(|id| system.particles[*id].pos)
            .collect();

        // Draw horizontal edges
        for j in 0..=rows {
            for i in 0..cols {
                let p1 = current_positions[j * w + i];
                let p2 = current_positions[j * w + i + 1];
                let color = if (i + j) % 2 == 0 { LIGHTGRAY } else { GRAY };
                draw_line_3d(p1, p2, color);
            }
        }

        // Draw vertical edges
        for j in 0..rows {
            for i in 0..=cols {
                let p1 = current_positions[j * w + i];
                let p2 = current_positions[(j + 1) * w + i];
                let color = if (i + j) % 2 == 0 { LIGHTGRAY } else { GRAY };
                draw_line_3d(p1, p2, color);
            }
        }

        // Draw spheres at vertices
        for pos in &current_positions {
            draw_sphere(*pos, 0.05, None, WHITE);
        }

        set_default_camera();

        // UI
        let progress = if history.is_empty() {
            0.0
        } else {
            commit_idx as f32 / history.len() as f32
        };
        draw_rectangle(10.0, 10.0, 300.0, 60.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text("Git Origami: Codebase Deformation", 20.0, 30.0, 20.0, WHITE);

        if commit_idx > 0 && commit_idx <= history.len() {
            let current_commit = &history[commit_idx - 1];
            draw_text(
                &format!("Commit: {}", &current_commit.short_hash),
                20.0,
                50.0,
                20.0,
                YELLOW,
            );
        }

        draw_rectangle(10.0, 75.0, 300.0 * progress, 5.0, GREEN);

        next_frame().await;
    }
}
