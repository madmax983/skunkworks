use git_associates::{model::Commit, GitModel};
use macroquad::miniquad::{ShaderSource, UniformDesc, UniformType};
use macroquad::prelude::*;
use std::fmt::Write; // For write! macro on String

mod decay;
mod shader;

struct Block {
    commit: Commit,
    pos: Vec3,
    size: Vec3,
    color: Color,
    decay: f32,
    restored: f32, // 0.0 to 1.0, 1.0 = fully restored
}

#[macroquad::main("Git Archaeology")]
async fn main() {
    // Load git history
    println!("Loading git history...");
    // We try to load from current directory
    let model = match GitModel::open(".") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to open git repo: {}", e);
            return;
        }
    };

    let history = match model.history_with_diffs(100) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to load history: {}", e);
            Vec::new()
        }
    };

    println!("Loaded {} commits", history.len());

    // Initialize blocks
    let mut blocks = Vec::new();
    for (i, commit) in history.iter().enumerate() {
        // Depth increases with index (older commits are deeper)
        let depth = i as f32 * 2.5;
        let decay = (i as f32 / 100.0).min(1.0); // Decay increases with depth

        blocks.push(Block {
            commit: commit.clone(),
            pos: vec3(0.0, -depth, 0.0),
            size: vec3(4.0, 1.0, 4.0),
            color: generate_color(&commit.author),
            decay,
            restored: 0.0,
        });
    }

    // Load Shader
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX_SHADER,
            fragment: shader::FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("time", UniformType::Float1),
                UniformDesc::new("decay", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .expect("Failed to load material");

    let mut camera_y = 0.0;
    let mut target_camera_y = 0.0;
    let mut selected_block_idx: Option<usize> = None;

    // Buffers for zero-allocation rendering
    let mut scratch_buf = String::with_capacity(256);
    let mut title_buf = String::with_capacity(256);
    let mut author_buf = String::with_capacity(256);
    let mut msg_buf = String::with_capacity(1024);

    loop {
        // Update
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            target_camera_y += 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            target_camera_y -= 1.0;
        }

        let (_, wheel_y) = mouse_wheel();
        target_camera_y += wheel_y * 2.0;

        // Smooth camera
        camera_y += (target_camera_y - camera_y) * 0.1;

        // Clamp camera
        let max_depth = blocks.len() as f32 * 2.5;
        if target_camera_y > 5.0 {
            target_camera_y = 5.0;
        }
        if target_camera_y < -max_depth - 10.0 {
            target_camera_y = -max_depth - 10.0;
        }

        // Find closest block to center
        let center_y_world = camera_y - 5.0;
        let mut closest_idx = None;
        let mut min_dist = 2.0; // Selection Threshold

        for (i, block) in blocks.iter().enumerate() {
            if (block.pos.y - center_y_world).abs() < min_dist {
                min_dist = (block.pos.y - center_y_world).abs();
                closest_idx = Some(i);
            }
        }
        selected_block_idx = closest_idx;

        // Repair mechanic
        if let Some(idx) = selected_block_idx {
            if is_key_down(KeyCode::Space) {
                blocks[idx].restored = (blocks[idx].restored + 0.02).min(1.0);
            } else {
                blocks[idx].restored = (blocks[idx].restored - 0.005).max(0.0);
            }
        }

        // Render
        clear_background(BLACK);

        // 3D Setup
        set_camera(&Camera3D {
            position: vec3(10.0, camera_y, 10.0),
            target: vec3(0.0, camera_y - 5.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Shaft
        for block in blocks.iter() {
            // Culling
            if (block.pos.y - (camera_y - 5.0)).abs() > 40.0 {
                continue;
            }

            // Apply Material
            let effective_decay = block.decay * (1.0 - block.restored);
            material.set_uniform("time", get_time() as f32);
            material.set_uniform("decay", effective_decay);
            gl_use_material(&material);

            draw_cube(block.pos, block.size, None, block.color);

            // Draw wireframe for "tech" look
            gl_use_default_material();
            draw_cube_wires(block.pos, block.size, DARKGRAY);
        }

        gl_use_default_material();

        // 2D Overlay (HUD)
        set_default_camera();

        draw_text("GIT ARCHAEOLOGY", 20.0, 30.0, 30.0, GREEN);
        draw_text(
            &format!("Depth: {:.1}m", -camera_y),
            20.0,
            60.0,
            20.0,
            GREEN,
        );
        draw_text("W/S: Move | SPACE: Restore", 20.0, 90.0, 20.0, GREEN);

        if let Some(idx) = selected_block_idx {
            let block = &blocks[idx];
            let effective_decay = block.decay * (1.0 - block.restored);

            // Draw Info Box
            let info_h = 240.0;
            let info_y = screen_height() - info_h - 10.0;
            draw_rectangle(
                10.0,
                info_y,
                screen_width() - 20.0,
                info_h,
                Color::new(0.0, 0.1, 0.0, 0.8),
            );
            draw_rectangle_lines(10.0, info_y, screen_width() - 20.0, info_h, 2.0, GREEN);

            // Corrupt Text using buffers
            scratch_buf.clear();
            let _ = write!(&mut scratch_buf, "Commit: {}", block.commit.short_hash);
            decay::corrupt_text_into(&scratch_buf, effective_decay, &mut title_buf);

            scratch_buf.clear();
            let _ = write!(&mut scratch_buf, "Author: {}", block.commit.author);
            decay::corrupt_text_into(&scratch_buf, effective_decay, &mut author_buf);

            scratch_buf.clear();
            let _ = write!(
                &mut scratch_buf,
                "{}",
                block.commit.message.lines().next().unwrap_or("")
            );
            decay::corrupt_text_into(&scratch_buf, effective_decay, &mut msg_buf);

            draw_text(&title_buf, 30.0, info_y + 40.0, 30.0, GREEN);
            draw_text(&author_buf, 30.0, info_y + 70.0, 20.0, GREEN);
            draw_text("Message:", 30.0, info_y + 100.0, 20.0, DARKGREEN);
            draw_text(&msg_buf, 30.0, info_y + 120.0, 20.0, LIGHTGRAY);

            if block.restored < 0.99 {
                draw_text(
                    "[HOLD SPACE TO RESTORE DATA]",
                    30.0,
                    info_y + 200.0,
                    20.0,
                    RED,
                );
                // Restoration progress bar
                let bar_width = 300.0;
                draw_rectangle(30.0, info_y + 210.0, bar_width, 10.0, DARKGRAY);
                draw_rectangle(30.0, info_y + 210.0, bar_width * block.restored, 10.0, BLUE);
            } else {
                draw_text("DATA INTEGRITY: 100%", 30.0, info_y + 200.0, 20.0, BLUE);
            }
        }

        next_frame().await
    }
}

fn generate_color(s: &str) -> Color {
    let mut sum = 0;
    for b in s.bytes() {
        sum += b as u32;
    }
    let r = ((sum * 13) % 255) as f32 / 255.0;
    let g = ((sum * 29) % 255) as f32 / 255.0;
    let b = ((sum * 47) % 255) as f32 / 255.0;
    Color::new(r, g, b, 1.0)
}
