// LINEAGE:
// - Parent A: crates/git-associates (Discrete codebase metadata and repository history)
// - Parent B: crates/physics-pbd (Continuous Position-Based Dynamics rigid body simulation)
// - Novel Trait: Git commits spawn as physical particles dropping into a PBD simulation, physically colliding and settling to represent codebase evolution weight.

use anyhow::Result;
use git_associates::GitModel;
use macroquad::prelude::*;
use physics_pbd::PbdSystem;

const GRAVITY: f32 = -9.81;

// Wrapper struct to hold rendering data alongside our physics particle index.
struct CommitBody {
    particle_index: usize,
    commit_hash: String,
    size: f32,
    color: Color,
}

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Git Physics".to_owned(),
        ..Default::default()
    }
}

fn main() -> Result<()> {
    if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        println!("Headless environment detected. Exiting gracefully to avoid XOpenDisplay panic.");
        return Ok(());
    }

    macroquad::Window::from_config(window_conf(), async_main());
    Ok(())
}

async fn async_main() {
    if let Err(e) = run_sim().await {
        println!("Error: {}", e);
    }
}

async fn run_sim() -> Result<()> {
    // Load Git history
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_path = std::path::Path::new(manifest_dir).join("../../");
    let mut commits = Vec::new();

    if let Ok(model) = GitModel::open(&repo_path) {
        if let Ok(history) = model.history_with_diffs(50) {
            // Limit to 50 for performance
            commits = history;
        }
    }

    if commits.is_empty() {
        println!("No commits found or unable to open repo.");
        return Ok(());
    }

    // Reverse so oldest commits drop first
    commits.reverse();

    let mut system = PbdSystem::new();
    let mut commit_bodies: Vec<CommitBody> = Vec::new();
    let mut commit_index = 0;

    // Timer to drop one commit per interval
    let mut drop_timer = 0.0;
    let drop_interval = 0.2; // seconds

    // Camera setup
    let mut cam_yaw: f32 = std::f32::consts::PI / 4.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 40.0;
    let mut last_mouse_pos = mouse_position();

    loop {
        let dt = get_frame_time().min(0.05);

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
        cam_dist = cam_dist.clamp(10.0, 150.0);

        // Spawning commits as particles
        drop_timer += dt;
        if drop_timer >= drop_interval && commit_index < commits.len() {
            drop_timer = 0.0;
            let commit = &commits[commit_index];

            // Calculate size based on insertions/deletions
            let mut total_changes = 1;
            if let Some(stats) = &commit.stats {
                total_changes = stats.insertions + stats.deletions;
            }
            let size = 0.5 + (total_changes as f32).log2().max(0.0) * 0.2;
            let mass = size * size;

            let spawn_x = rand::gen_range(-5.0, 5.0);
            let spawn_z = rand::gen_range(-5.0, 5.0);

            // Use component-wise creation to avoid glam version mismatches
            let p_idx = system.add_particle(
                physics_pbd::glam::Vec3::new(spawn_x, 20.0, spawn_z),
                1.0 / mass,
            );

            commit_bodies.push(CommitBody {
                particle_index: p_idx,
                commit_hash: commit.short_hash.clone(),
                size,
                color: Color::new(
                    rand::gen_range(0.3, 1.0),
                    rand::gen_range(0.3, 1.0),
                    rand::gen_range(0.3, 1.0),
                    1.0,
                ),
            });

            commit_index += 1;
        }

        // Apply external forces (Gravity)
        for p in &mut system.particles {
            if p.inv_mass > 0.0 {
                p.vel.y += GRAVITY * dt;
            }
        }

        // Collision detection and response (simple spheres)
        let num_bodies = commit_bodies.len();
        for _ in 0..3 {
            // Solve iterations
            // Ground collision
            for body in &commit_bodies {
                let p = &mut system.particles[body.particle_index];
                if p.pos.y < body.size {
                    p.pos.y = body.size;
                }
            }

            // Particle-Particle collision
            for i in 0..num_bodies {
                for j in (i + 1)..num_bodies {
                    let idx_a = commit_bodies[i].particle_index;
                    let idx_b = commit_bodies[j].particle_index;

                    let p_a = system.particles[idx_a].pos;
                    let p_b = system.particles[idx_b].pos;

                    let r_a = commit_bodies[i].size;
                    let r_b = commit_bodies[j].size;

                    let diff = p_a - p_b;
                    let dist_sq = diff.length_squared();
                    let min_dist = r_a + r_b;

                    if dist_sq < min_dist * min_dist && dist_sq > 0.0001 {
                        let dist = dist_sq.sqrt();
                        let normal = diff / dist;
                        let overlap = min_dist - dist;

                        let inv_mass_a = system.particles[idx_a].inv_mass;
                        let inv_mass_b = system.particles[idx_b].inv_mass;
                        let w_sum = inv_mass_a + inv_mass_b;

                        if w_sum > 0.0 {
                            let correction = normal * overlap / w_sum;
                            system.particles[idx_a].pos += correction * inv_mass_a;
                            system.particles[idx_b].pos -= correction * inv_mass_b;
                        }
                    }
                }
            }
        }

        // Physics step
        system.step(dt, 5);

        // Manual bounds check to keep particles contained
        for body in &commit_bodies {
            let p = &mut system.particles[body.particle_index];
            if p.pos.x > 10.0 {
                p.pos.x = 10.0;
            }
            if p.pos.x < -10.0 {
                p.pos.x = -10.0;
            }
            if p.pos.z > 10.0 {
                p.pos.z = 10.0;
            }
            if p.pos.z < -10.0 {
                p.pos.z = -10.0;
            }
        }

        clear_background(BLACK);

        let cam_pos = vec3(
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 5.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw ground grid
        for i in -10..=10 {
            draw_line_3d(
                vec3(i as f32, 0.0, -10.0),
                vec3(i as f32, 0.0, 10.0),
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
            draw_line_3d(
                vec3(-10.0, 0.0, i as f32),
                vec3(10.0, 0.0, i as f32),
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        // Draw commit particles
        for body in &commit_bodies {
            let p = &system.particles[body.particle_index];
            // Extract components manually to cross glam versions
            let draw_pos = vec3(p.pos.x, p.pos.y, p.pos.z);
            draw_sphere(draw_pos, body.size, None, body.color);
        }

        set_default_camera();

        draw_text("Git Physics", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Commits dropped: {}/{}", commit_index, commits.len()),
            10.0,
            60.0,
            20.0,
            WHITE,
        );

        // Draw a small 2D overlay text for the last dropped commit
        if commit_index > 0 {
            let last = &commit_bodies[commit_index - 1];
            draw_text(
                &format!("Latest: {}", last.commit_hash),
                10.0,
                90.0,
                20.0,
                last.color,
            );
        }

        next_frame().await
    }
}
