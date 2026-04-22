//! # 🧬 Splice: `git-origami`
//!
//! **Lineage:** `crates/git-associates` × `crates/origami`
//!
//! **Concept:** Structural Codebase Deformation. A 3D continuous procedural Miura-ori soft-body mesh
//! dynamically represents the codebase file hierarchy and history.
//!
//! **Novel Trait:** The rigid, discrete lattice of the codebase is projected down onto a continuous
//! soft-body mesh. As the repository undergoes changes, the topological space buckles and crumples.

use git_associates::GitModel;
use macroquad::prelude::*;
use origami::{generate_miura_mesh, MiuraParams, Orientation};
use physics_pbd::PbdSystem;
use std::env;

const GRID_SIZE: usize = 20;

#[macroquad::main("Git Origami")]
async fn main() -> anyhow::Result<()> {
    let current_dir = env::current_dir().unwrap_or_else(|_| ".".into());
    let repo_path = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or(current_dir);

    // Parse Git History
    let git_model = GitModel::open(&repo_path)?;
    let mut history = git_model.history_with_diffs(50)?;
    history.reverse(); // Play chronologically

    // Setup Origami mesh
    let params = MiuraParams {
        a: 1.5,
        b: 1.5,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let base_mesh = generate_miura_mesh(params, (GRID_SIZE, GRID_SIZE), 0.8);

    let mut pbd = PbdSystem::new();
    let mut particle_ids = Vec::new();

    // Add particles for each vertex
    for vertex in &base_mesh.vertices {
        let mass = 1.0;
        let id = pbd.add_particle(vertex.pos, mass);
        particle_ids.push(id);
    }

    // Create soft body structural constraints based on the mesh topology
    for chunk in base_mesh.indices.chunks_exact(3) {
        for i in 0..3 {
            let p1 = particle_ids[chunk[i] as usize];
            let p2 = particle_ids[chunk[(i + 1) % 3] as usize];
            let v1 = pbd.particles[p1].pos;
            let v2 = pbd.particles[p2].pos;
            let dist = v1.distance(v2);
            pbd.add_distance_constraint(p1, p2, dist);
        }
    }

    // Anchor the corners
    let top_left = 0;
    let top_right = GRID_SIZE;
    let bottom_left = (GRID_SIZE + 1) * GRID_SIZE;
    let bottom_right = (GRID_SIZE + 1) * (GRID_SIZE + 1) - 1;

    pbd.particles[particle_ids[top_left]].inv_mass = 0.0;
    pbd.particles[particle_ids[top_right]].inv_mass = 0.0;
    pbd.particles[particle_ids[bottom_left]].inv_mass = 0.0;
    pbd.particles[particle_ids[bottom_right]].inv_mass = 0.0;

    let mut commit_idx = 0;
    let mut last_commit_time = get_time();
    let commit_interval = 0.5;

    let camera = Camera3D {
        position: vec3(GRID_SIZE as f32, GRID_SIZE as f32, GRID_SIZE as f32 * 1.5),
        target: vec3(GRID_SIZE as f32 / 2.0, GRID_SIZE as f32 / 2.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    loop {
        let current_time = get_time();

        // Step physics
        pbd.step(0.016, 5);

        clear_background(BLACK);

        set_camera(&camera);

        // Draw the soft body mesh
        for chunk in base_mesh.indices.chunks_exact(3) {
            let p1 = particle_ids[chunk[0] as usize];
            let p2 = particle_ids[chunk[1] as usize];
            let p3 = particle_ids[chunk[2] as usize];

            let v1 = pbd.particles[p1].pos;
            let v2 = pbd.particles[p2].pos;
            let v3 = pbd.particles[p3].pos;

            // Draw lines for the mesh
            draw_line_3d(v1, v2, GREEN);
            draw_line_3d(v2, v3, GREEN);
            draw_line_3d(v3, v1, GREEN);
        }

        set_default_camera();

        // Display current commit info
        if commit_idx < history.len() {
            let commit = &history[commit_idx];
            draw_text(
                &format!("Commit: {} - {}", commit.short_hash, commit.message),
                10.0,
                20.0,
                20.0,
                WHITE,
            );
        } else {
            draw_text("End of history", 10.0, 20.0, 20.0, WHITE);
        }

        // Apply forces based on Git Commits
        if current_time - last_commit_time > commit_interval {
            if commit_idx < history.len() {
                let commit = &history[commit_idx];

                if let Some(stats) = &commit.stats {
                    let total_changes = stats.insertions + stats.deletions;
                    if total_changes > 0 {
                        let hash_bytes = commit.hash.as_bytes();
                        let target_idx = hash_bytes[0] as usize % (GRID_SIZE * GRID_SIZE);

                        // Pick a node and apply force
                        let p_id = particle_ids[target_idx];

                        let force_dir = vec3(
                            0.0,
                            0.0,
                            (stats.insertions as f32 - stats.deletions as f32).signum(),
                        );
                        let force_mag = (total_changes as f32).sqrt().clamp(0.5, 10.0);

                        let mut current_pos = pbd.particles[p_id].pos;
                        current_pos += force_dir * force_mag * 0.1;
                        pbd.particles[p_id].pos = current_pos;
                    }
                }

                commit_idx += 1;
            }
            last_commit_time = current_time;
        }

        next_frame().await;
    }
}
