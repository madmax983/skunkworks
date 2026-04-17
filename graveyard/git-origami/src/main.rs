use git_associates::GitModel;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

#[macroquad::main("Git Origami: Version Control Morphogenesis")]
async fn main() {
    let cols = 20;
    let rows = 20;

    let repo_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string())
        + "/../../crates/git-associates";

    let git_model = match GitModel::open(repo_path) {
        Ok(model) => Some(model),
        Err(e) => {
            eprintln!("Failed to open git repo: {:?}", e);
            None
        }
    };

    let mut commit_index = 0;
    let mut history = vec![];

    if let Some(model) = git_model {
        if let Ok(hist) = model.history_with_diffs(200) {
            history = hist;
        }
    }

    let mut last_update = get_time();
    let update_interval = 0.5; // seconds per commit visualization
    let mut current_expansion_force = 0.0;

    // Initialize Origami Mesh and Physics
    let params = MiuraParams {
        a: 0.5,
        b: 0.5,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let points = generate_miura_grid(params, (cols, rows), 0.5);

    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    // Add particles
    for pos in &points {
        p_indices.push(system.add_particle(*pos, 1.0));
    }

    // Pin corners
    let w = cols + 1;
    let top_left = 0;
    let top_right = cols;
    let bottom_left = rows * w;
    let bottom_right = rows * w + cols;

    system.add_pin_constraint(p_indices[top_left], points[top_left]);
    system.add_pin_constraint(p_indices[top_right], points[top_right]);
    system.add_pin_constraint(p_indices[bottom_left], points[bottom_left]);
    system.add_pin_constraint(p_indices[bottom_right], points[bottom_right]);

    // Add constraints
    let stiffness = 0.8;
    for y in 0..=rows {
        for x in 0..=cols {
            let i = y * w + x;

            if x < cols {
                let right = y * w + (x + 1);
                let dist = points[i].distance(points[right]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[right],
                    dist * 0.2,
                    dist * 1.5,
                    stiffness,
                );
            }

            if y < rows {
                let down = (y + 1) * w + x;
                let dist = points[i].distance(points[down]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[down],
                    dist * 0.2,
                    dist * 1.5,
                    stiffness,
                );
            }
        }
    }

    let mut camera = Camera3D {
        position: vec3(0.0, -15.0, 15.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let mut rotation = 0.0f32;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Headless check
        if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
            return;
        }

        let time = get_time();

        if time - last_update > update_interval && !history.is_empty() {
            if commit_index < history.len() {
                let commit = &history[commit_index];
                if let Some(stats) = &commit.stats {
                    let total_changes = (stats.insertions + stats.deletions) as f32;
                    current_expansion_force = (total_changes + 1.0).ln() * 0.4;
                } else {
                    current_expansion_force = 0.0;
                }
                commit_index = (commit_index + 1) % history.len();
            }
            last_update = time;
        }

        // Relax force over time
        current_expansion_force *= 0.95;

        let target_factor = 1.0 - (current_expansion_force.clamp(0.0, 1.0) * 0.8);

        // Apply expansion force to all actuators
        for constraint in &mut system.constraints {
            if let Constraint::Actuator { ref mut factor, .. } = constraint {
                *factor = *factor * 0.9 + target_factor * 0.1;
            }
        }

        // Simulate physics
        system.step(0.016, 5);

        // 3D Rendering
        rotation += 0.005;
        camera.position = vec3(rotation.sin() * 20.0, 15.0, rotation.cos() * 20.0);
        set_camera(&camera);

        let max_heat = current_expansion_force.clamp(0.0, 1.0);

        for y in 0..rows {
            for x in 0..cols {
                let i = y * w + x;

                let p00 = system.particles[p_indices[i]].pos;
                let p10 = system.particles[p_indices[i + 1]].pos;
                let p01 = system.particles[p_indices[i + w]].pos;

                let r = max_heat;
                let g = max_heat * 0.2 + 0.1;
                let b = (1.0 - max_heat) * 0.8;

                let color = Color::new(r, g, b, 1.0);

                draw_line_3d(p00, p10, color);
                draw_line_3d(p00, p01, color);
            }
        }

        for x in 0..cols {
            let i = rows * w + x;
            let p0 = system.particles[p_indices[i]].pos;
            let p1 = system.particles[p_indices[i + 1]].pos;
            draw_line_3d(p0, p1, Color::new(0.2, 0.2, 0.4, 1.0));
        }
        for y in 0..rows {
            let i = y * w + cols;
            let p0 = system.particles[p_indices[i]].pos;
            let p1 = system.particles[p_indices[i + w]].pos;
            draw_line_3d(p0, p1, Color::new(0.2, 0.2, 0.4, 1.0));
        }

        set_default_camera();

        // Draw some UI
        if !history.is_empty() {
            let commit = &history[commit_index.saturating_sub(1) % history.len()];
            draw_text(
                &format!("Commit: {}", commit.short_hash),
                10.0,
                30.0,
                30.0,
                WHITE,
            );
            draw_text(
                &format!("Author: {}", commit.author),
                10.0,
                60.0,
                20.0,
                LIGHTGRAY,
            );
            if let Some(stats) = &commit.stats {
                draw_text(
                    &format!("+{} -{}", stats.insertions, stats.deletions),
                    10.0,
                    90.0,
                    20.0,
                    if stats.insertions > stats.deletions {
                        GREEN
                    } else {
                        RED
                    },
                );
            }
        } else {
            draw_text("Loading Git History...", 10.0, 30.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}
