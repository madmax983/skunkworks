use gray_scott::GrayScott;
use macroquad::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod git;
use git::get_commit_history;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;
const DT: f32 = 1.0;

fn hash_string_to_coord(s: &str, max: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    (hash % max as u64) as usize
}

#[macroquad::main("Git Gray-Scott Morphogenesis")]
async fn main() {
    let commits = get_commit_history().unwrap_or_default();
    let mut commits_iter = commits.into_iter().rev().peekable();

    // Initialize the continuous Gray-Scott grid
    let mut gs = GrayScott::new(WIDTH, HEIGHT);

    // Default Gray-Scott parameters for generic patterns (e.g. coral/spots)
    let feed = 0.0545;
    let kill = 0.062;

    // We'll advance the "time" over git commits
    let mut frame_count = 0;

    // We will use a texture to render the grid
    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Drop V chemical if a new commit occurs
        if frame_count % 30 == 0 {
            if let Some(commit) = commits_iter.next() {
                // Determine deposit location based on commit hash
                let cx = hash_string_to_coord(&commit.hash, WIDTH - 20) + 10;
                let cy = hash_string_to_coord(&commit.author, HEIGHT - 20) + 10;

                let r = 5;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            let nx = (cx as isize + dx).rem_euclid(WIDTH as isize) as usize;
                            let ny = (cy as isize + dy).rem_euclid(HEIGHT as isize) as usize;
                            let idx = gs.get_index(nx, ny);
                            gs.v_mut()[idx] = 1.0;
                        }
                    }
                }
            }
        }

        // Run simulation steps
        for _ in 0..10 {
            gs.update(feed, kill, DT);
        }

        // Render
        let v_data = gs.v();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = gs.get_index(x, y);
                let v = v_data[idx].clamp(0.0, 1.0);

                // Color mapping: map the V chemical to a fiery / organic color
                let color = Color::new(v, v * 0.5, v * 0.2, 1.0);
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);

        clear_background(BLACK);

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

        draw_text("Git Gray-Scott Morphogenesis", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Codebase history acting as catalysts",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        frame_count += 1;
        next_frame().await;
    }
}
