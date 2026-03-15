use macroquad::prelude::*;

mod git;
mod simulation;

use simulation::{World, WORLD_SIZE};

#[macroquad::main("Git Swarm")]
async fn main() {
    let mut world = World::new();

    // Parse Git Commits
    let commits = git::get_commit_history().unwrap_or_else(|_| Vec::new());
    let mut commit_idx = 0;

    // Create Render Buffer
    let width = 800;
    let height = 800;
    request_new_screen_size(width as f32, height as f32);

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut time_since_last_commit = 0.0;

    loop {
        clear_background(BLACK);

        if !commits.is_empty() {
            time_since_last_commit += get_frame_time();

            // Move to next commit every 2 seconds
            if time_since_last_commit > 2.0 {
                let commit = &commits[commit_idx];

                // Derive target coordinates from hash
                let hex_val1 = u32::from_str_radix(&commit.hash[0..4], 16).unwrap_or(0);
                let hex_val2 = u32::from_str_radix(&commit.hash[4..8], 16).unwrap_or(0);

                let target_x = (hex_val1 as f32 / 65535.0) * WORLD_SIZE;
                let target_y = (hex_val2 as f32 / 65535.0) * WORLD_SIZE;

                world.set_target(vec2(target_x, target_y));

                println!("Commit: {} - Target: ({}, {})", commit.message, target_x, target_y);

                commit_idx = (commit_idx + 1) % commits.len();
                time_since_last_commit = 0.0;
            }
        }

        // Run simulation
        world.update();

        // Render to buffer
        world.render_to_buffer(image.bytes.as_mut_slice(), width, height);

        // Upload and draw
        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Current Target info
        if !commits.is_empty() {
            let commit = &commits[commit_idx];
            draw_text(
                &format!("Targeting: {} ({})", commit.author, commit.message),
                20.0,
                30.0,
                30.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
