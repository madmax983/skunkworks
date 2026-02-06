mod erosion;
mod git_history;
mod terrain;

use macroquad::prelude::*;
use terrain::{Terrain, GRID_SIZE};

#[macroquad::main("Code Erosion")]
async fn main() {
    let mut terrain = Terrain::new();
    terrain.init_from_files(".");

    let history = git_history::load_history();
    let mut history_index = 0;

    // Camera params
    let cam_pos = vec3(GRID_SIZE as f32 / 2.0, 100.0, GRID_SIZE as f32 / 2.0);
    let mut cam_target = vec3(GRID_SIZE as f32 / 2.0, 0.0, GRID_SIZE as f32 / 2.0);
    let mut zoom = 1.0;
    let mut rot_angle: f32 = 0.0;

    // Commit replay speed
    let commits_per_frame = 1;
    let erosion_steps = 1000;
    let mut auto_rotate = true;

    loop {
        // --- Input ---
        if is_key_down(KeyCode::W) {
            cam_target.z -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            cam_target.z += 1.0;
        }
        if is_key_down(KeyCode::A) {
            cam_target.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            cam_target.x += 1.0;
        }
        if is_key_down(KeyCode::Q) {
            zoom *= 1.01;
        }
        if is_key_down(KeyCode::E) {
            zoom *= 0.99;
        }
        if is_key_pressed(KeyCode::Space) {
            auto_rotate = !auto_rotate;
        }

        if auto_rotate {
            rot_angle += 0.005;
        }

        // --- Simulation ---
        // Replay commits
        for _ in 0..commits_per_frame {
            if history_index < history.len() {
                let commit = &history[history_index];
                history_index += 1;

                for change in &commit.changes {
                    if let Some((x, y)) = terrain.get_coords(&change.path) {
                        // "Rain"
                        // Add water? The erosion model spawns drops randomly.
                        // Here we can spawn drops specifically at these coords.
                        // For now, let's just Uplift (add lines)
                        // Add height for added lines
                        let idx = y * GRID_SIZE + x;

                        // Scale: 100 lines = 1.0 height unit?
                        let uplift = (change.added as f32).sqrt() / 10.0;
                        terrain.heightmap[idx] += uplift;

                        // Erosion from "Rain" of deletions/churn?
                        // Let's say churn causes local erosion
                        let _churn = change.added + change.deleted;
                        // Maybe spawn erosion drops here?
                        // For simplicity, we just uplift now and let global rain erode it.
                    }
                }
            }
        }

        // Erode
        erosion::erode(&mut terrain, erosion_steps);

        // --- Render ---
        clear_background(BLACK);

        // 3D Camera setup
        let orbit_radius = GRID_SIZE as f32 * 1.5 * zoom;
        let cam_x = cam_target.x + orbit_radius * rot_angle.cos();
        let cam_z = cam_target.z + orbit_radius * rot_angle.sin();
        let cam_y = cam_pos.y * zoom + 100.0; // elevated

        set_camera(&Camera3D {
            position: vec3(cam_x, cam_y, cam_z),
            target: cam_target,
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        draw_grid(20, 1.0, BLACK, GRAY);

        // Render Terrain as Points or Mesh
        // Mesh is expensive to rebuild every frame?
        // 65k points.
        // Let's draw vertical lines for non-zero height

        for y in (0..GRID_SIZE).step_by(2) {
            // Skip every other for perf
            for x in (0..GRID_SIZE).step_by(2) {
                let h = terrain.get_height(x, y);
                if h > 0.1 {
                    let pos = vec3(x as f32, 0.0, y as f32);
                    let color = if h > 10.0 {
                        WHITE
                    } else if h > 5.0 {
                        GRAY
                    } else if h > 2.0 {
                        BROWN
                    } else {
                        GREEN
                    };

                    // Draw a box or line
                    draw_cube(vec3(pos.x, h / 2.0, pos.z), vec3(1.0, h, 1.0), None, color);
                }
            }
        }

        set_default_camera();

        // HUD
        draw_text(
            &format!("Commit: {}/{}", history_index, history.len()),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        if history_index < history.len() {
            let date = history[history_index].date.to_string();
            draw_text(&format!("Date: {}", date), 10.0, 60.0, 20.0, LIGHTGRAY);
        }

        draw_text(
            "WASD: Pan | QE: Zoom | SPACE: Rotate",
            10.0,
            screen_height() - 20.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await
    }
}
