use macroquad::prelude::*;
use terrain::Terrain;
use git::GitScanner;

mod terrain;
mod git;

#[cfg(test)]
mod terrain_test;

#[macroquad::main("Sediment Flow")]
async fn main() {
    let width = 300;
    let height = 300;

    let mut terrain = Terrain::new(width, height);

    // Load git history
    println!("Loading git history...");
    let commits = match GitScanner::load_history() {
        Ok(c) => {
            println!("Loaded {} commits.", c.len());
            c
        },
        Err(e) => {
            eprintln!("Failed to load git history: {}", e);
            vec![]
        }
    };

    let mut commit_idx = 0;
    let mut paused = false;
    let mut speed = 5; // Commits per frame
    let mut erosion_steps = 2000; // Droplets per frame

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    // Camera / View
    // Just fit to screen

    loop {
        // Controls
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            terrain = Terrain::new(width, height);
            commit_idx = 0;
            image = Image::gen_image_color(width as u16, height as u16, BLACK);
        }
        if is_key_down(KeyCode::Right) {
            speed += 1;
        }
        if is_key_down(KeyCode::Left) && speed > 0 {
            speed -= 1;
        }
        if is_key_down(KeyCode::Up) {
            erosion_steps += 100;
        }
        if is_key_down(KeyCode::Down) && erosion_steps > 100 {
            erosion_steps -= 100;
        }
        if is_key_pressed(KeyCode::S) {
            get_screen_data().export_png("sediment_flow.png");
            println!("Saved screenshot to sediment_flow.png");
        }

        // 1. Uplift (Geological Time)
        if !paused && commit_idx < commits.len() {
            let end_idx = (commit_idx + speed).min(commits.len());
            for i in commit_idx..end_idx {
                let commit = &commits[i];
                for file in &commit.files {
                    let path_str = file.to_string_lossy();
                    let (x, y) = GitScanner::map_path(&path_str, width, height);

                    // Uplift amount based on file length? Or constant?
                    // Constant for now, maybe small random
                    terrain.uplift(x, y, 2.0);

                    // Also uplift neighbors slightly for a "mountain" feel
                    terrain.uplift(x+1, y, 1.0);
                    terrain.uplift(x, y+1, 1.0);
                    if x > 0 { terrain.uplift(x-1, y, 1.0); }
                    if y > 0 { terrain.uplift(x, y-1, 1.0); }
                }
            }
            commit_idx = end_idx;
        }

        // 2. Erosion (Hydraulic Process)
        for _ in 0..erosion_steps {
            let rx = rand::gen_range(0.0, width as f64);
            let ry = rand::gen_range(0.0, height as f64);
            terrain.erode_droplet(rx, ry);
        }
        terrain.decay_water();

        // 3. Render to Texture
        for y in 0..height {
            for x in 0..width {
                let color = terrain.get_color(x, y);
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);

        // 4. Draw to Screen
        clear_background(BLACK);

        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale = (screen_w / width as f32).min(screen_h / height as f32);

        let draw_w = width as f32 * scale;
        let draw_h = height as f32 * scale;
        let offset_x = (screen_w - draw_w) / 2.0;
        let offset_y = (screen_h - draw_h) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );

        // UI
        draw_text(
            &format!("Commits: {}/{} | Speed: {} | Erosion: {}", commit_idx, commits.len(), speed, erosion_steps),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text("[Space] Pause/Resume [R] Reset [Arrows] Speed/Erosion", 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
