use clap::Parser;
use codex_void::{starmap::SPACING, Glyph, Scanner};
use image::io::Reader as ImageReader;
use macroquad::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "codex.png")]
    image_path: PathBuf,
}

#[macroquad::main("Observatory")]
async fn main() {
    // Parse args
    // Note: macroquad usually handles its own args or ignores them.
    // Clap might conflict if macroquad consumes args.
    // But let's try.
    let cli = Cli::parse();

    let img_path = &cli.image_path;
    if !img_path.exists() {
        eprintln!("Image file not found: {:?}", img_path);
        // We can't exit easily in macroquad main, just return.
        return;
    }

    // Load image data for scanning
    let img_dynamic = match ImageReader::open(img_path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to decode image: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            return;
        }
    };
    let img_rgb8 = img_dynamic.to_rgb8();

    // Scan for stars
    let mut stars = Scanner::find_stars(&img_rgb8);

    // Sort stars to recover order
    stars.sort_by(|a, b| {
        let row_a = a.1 / SPACING;
        let row_b = b.1 / SPACING;
        if row_a != row_b {
            row_a.cmp(&row_b)
        } else {
            a.0.cmp(&b.0)
        }
    });

    // Decode message string
    let mut decoded_bytes = Vec::new();
    for (x, y) in &stars {
        let b = Glyph::decode(*x, *y, &img_rgb8);
        decoded_bytes.push(b);
    }

    let decoded_text = String::from_utf8(decoded_bytes.clone())
        .unwrap_or_else(|_| "Error: Invalid UTF-8 sequence".to_string());

    // Load texture for rendering
    let texture = match load_texture(img_path.to_str().unwrap()).await {
        Ok(tex) => tex,
        Err(e) => {
            eprintln!("Failed to load texture: {}", e);
            return;
        }
    };

    loop {
        clear_background(BLACK);

        // Draw Texture
        draw_texture(&texture, 0., 0., WHITE);

        let (mx, my) = mouse_position();

        // Draw Overlay
        for (x, y) in &stars {
            let cx = *x as f32;
            let cy = *y as f32;

            // Check hover
            let hovered = (mx - cx).abs() < 8.0 && (my - cy).abs() < 8.0;

            let color = if hovered { YELLOW } else { GREEN };

            // Draw circle around star
            draw_circle_lines(cx, cy, 6.0, 1.0, color);

            if hovered {
                let b = Glyph::decode(*x, *y, &img_rgb8);
                // Draw value
                draw_text(&format!("0x{:02X}", b), cx + 10., cy - 10., 20., YELLOW);
                if b.is_ascii_graphic() || b == 0x20 {
                    draw_text(&format!("'{}'", b as char), cx + 10., cy + 10., 20., ORANGE);
                }

                // Draw rays for active bits
                for i in 0..8 {
                    if (b >> i) & 1 == 1 {
                        let angle = (i as f32) * 45.0f32.to_radians();
                        // 0 is East (Right)
                        // 2 is South (Down)
                        let rx = cx + angle.cos() * 8.0;
                        let ry = cy + angle.sin() * 8.0;
                        draw_line(cx, cy, rx, ry, 1.0, RED);
                    }
                }
            }
        }

        // Draw Decoded Text Panel
        let text_y = texture.height() + 20.;
        draw_text("Decoded Signal:", 10., text_y, 20., LIGHTGRAY);
        draw_text(&decoded_text, 10., text_y + 25., 20., WHITE);

        next_frame().await;
    }
}
