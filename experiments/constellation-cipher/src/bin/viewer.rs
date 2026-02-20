use constellation_cipher::{recover_stars, Star};
use macroquad::prelude::*;
use std::env;

#[macroquad::main("Constellation Viewer")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        // If args are missing, show a message on screen
        loop {
            clear_background(BLACK);
            draw_text("Usage: viewer <image_path> <key>", 20.0, 30.0, 20.0, WHITE);
            next_frame().await;
        }
    }
    let image_path = &args[1];
    let key = &args[2];

    // Load Image using `image` crate (standard filesystem)
    // We assume this runs natively.
    let img_bytes = std::fs::read(image_path).expect("Failed to read image file");
    let img = image::load_from_memory(&img_bytes)
        .expect("Failed to decode image")
        .to_rgba8();
    let width = img.width();
    let height = img.height();

    // Create Texture
    let texture = Texture2D::from_rgba8(width as u16, height as u16, &img);

    // Recover Stars
    // This runs on CPU
    println!("Recovering stars with key '{}'...", key);
    let stars_result = recover_stars(&img, key);

    let stars = match stars_result {
        Ok(s) => s,
        Err(e) => {
            println!("Error recovering stars: {}", e);
            vec![]
        }
    };
    println!("Found {} stars.", stars.len());

    let mut cam = Camera2D {
        zoom: vec2(1.0 / width as f32 * 2.0, -1.0 / height as f32 * 2.0),
        target: vec2(width as f32 / 2.0, height as f32 / 2.0),
        ..Default::default()
    };

    let mut zoom_level = 1.0;

    loop {
        clear_background(BLACK);

        // Input Handling
        if is_key_down(KeyCode::Right) {
            cam.target.x += 10.0 / zoom_level;
        }
        if is_key_down(KeyCode::Left) {
            cam.target.x -= 10.0 / zoom_level;
        }
        if is_key_down(KeyCode::Up) {
            cam.target.y -= 10.0 / zoom_level;
        }
        if is_key_down(KeyCode::Down) {
            cam.target.y += 10.0 / zoom_level;
        }

        let (_, wheel) = mouse_wheel();
        if wheel != 0.0 {
            zoom_level *= if wheel > 0.0 { 1.1 } else { 0.9 };
            // Clamp zoom
            if zoom_level < 0.1 {
                zoom_level = 0.1;
            }
            if zoom_level > 50.0 {
                zoom_level = 50.0;
            }

            cam.zoom = vec2(
                1.0 / width as f32 * 2.0 * zoom_level,
                -1.0 / height as f32 * 2.0 * zoom_level,
            );
        }

        set_camera(&cam);

        // Draw Texture
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Hover Logic
        let (mx, my) = mouse_position();
        let mouse_world = cam.screen_to_world(vec2(mx, my));

        // Find nearest star
        let mut nearest_star: Option<&Star> = None;
        let detection_radius = 5.0 / zoom_level; // pixel radius scales with zoom
        let mut min_dist_sq = detection_radius * detection_radius;

        for star in &stars {
            let dx = star.x as f32 - mouse_world.x;
            let dy = star.y as f32 - mouse_world.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest_star = Some(star);
            }
        }

        // Highlight Nearest
        if let Some(star) = nearest_star {
            // Draw a circle around the star
            // +0.5 to center on pixel
            draw_circle_lines(
                star.x as f32 + 0.5,
                star.y as f32 + 0.5,
                3.0 / zoom_level,
                1.0 / zoom_level,
                GREEN,
            );
        }

        set_default_camera();

        // UI Overlay
        if let Some(star) = nearest_star {
            let char_display = if star.byte >= 32 && star.byte <= 126 {
                format!("'{}'", star.byte as char)
            } else {
                format!("0x{:02X}", star.byte)
            };

            draw_text(
                &format!("Star at ({}, {}): {}", star.x, star.y, char_display),
                20.0,
                30.0,
                30.0,
                WHITE,
            );

            // Draw Byte value as color swatch
            draw_rectangle(
                20.0,
                40.0,
                30.0,
                30.0,
                Color::from_rgba(star.color[0], star.color[1], star.color[2], 255),
            );
        } else {
            draw_text("Hover over a star to decode", 20.0, 30.0, 20.0, GRAY);
            draw_text(
                &format!("Zoom: {:.2} (Scroll to zoom, Arrows to pan)", zoom_level),
                20.0,
                50.0,
                20.0,
                GRAY,
            );
        }

        next_frame().await;
    }
}
