use macroquad::prelude::*;
use std::path::PathBuf;
use stego_attack::config::AttackConfig;
use stego_attack::simulation::World;
use stego_attack::stego;

#[macroquad::main("Stego Attack")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("assets/demo.png")
    };

    println!("Loading image from {:?}", path);

    let img = image::open(&path)
        .map(|i| i.to_rgba8())
        .unwrap_or_else(|_| {
            println!("Failed to load image, generating noise fallback");
            // Fallback: Generate noise image
            let width = 500;
            let height = 500;
            let mut img = image::RgbaImage::new(width, height);
            for pixel in img.pixels_mut() {
                *pixel = image::Rgba([::rand::random(), ::rand::random(), ::rand::random(), 255]);
            }
            img
        });

    let config = match stego::extract(&img) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_else(|_| {
            println!("Failed to parse config JSON, using default");
            AttackConfig::default()
        }),
        Err(e) => {
            println!("No stego data found: {}, using default", e);
            AttackConfig::default()
        }
    };

    println!("Config loaded: {:?}", config);

    let mut world = World::new(&img, &config);

    // Texture for display
    let width = world.width as u16;
    let height = world.height as u16;
    // Create a Macroquad Image to hold the pixel buffer
    let mut display_image = Image::gen_image_color(width, height, BLACK);
    let texture = Texture2D::from_image(&display_image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        world.update();

        // Render world to the image buffer
        world.render_to_buffer(&mut display_image.bytes);

        // Upload to GPU
        texture.update(&display_image);

        clear_background(BLACK);

        // Draw centered and scaled
        let scale = (screen_width() / width as f32).min(screen_height() / height as f32);
        let dw = width as f32 * scale;
        let dh = height as f32 * scale;
        let dx = (screen_width() - dw) / 2.0;
        let dy = (screen_height() - dh) / 2.0;

        draw_texture_ex(
            &texture,
            dx,
            dy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                ..Default::default()
            },
        );

        draw_text("Stego Attack", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", world.agents.len()),
            10.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Target: {:.2}, {:.2}", config.target_x, config.target_y),
            10.0,
            90.0,
            20.0,
            WHITE,
        );

        // Draw target marker
        let tx = dx + config.target_x * dw;
        let ty = dy + config.target_y * dh;
        draw_circle_lines(tx, ty, 10.0 * scale, 2.0, RED);

        next_frame().await
    }
}
