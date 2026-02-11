mod archivist;
mod art;
mod stego;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pixel-archivist")]
#[command(about = "Steganographic Archiver", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pack a directory into an image
    Pack {
        #[arg(value_name = "DIR")]
        dir: PathBuf,
        #[arg(value_name = "OUTPUT_IMAGE")]
        output: PathBuf,
    },
    /// Unpack an image into a directory
    Unpack {
        #[arg(value_name = "IMAGE")]
        image: PathBuf,
        #[arg(value_name = "OUTPUT_DIR")]
        output: PathBuf,
    },
    /// Visualize an archive image
    Gui {
        #[arg(value_name = "IMAGE")]
        image: Option<PathBuf>,
    },
    /// Pack the tool's own source code into an image
    SelfPortrait {
        #[arg(value_name = "OUTPUT_IMAGE")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pack { dir, output } => {
            run_pack(&dir, &output)?;
        }
        Commands::Unpack { image, output } => {
            println!("Opening {:?}...", image);
            let img = image::open(&image)?;

            println!("Extracting data...");
            let data = stego::extract(&img)?;
            println!("Extracted {} bytes", data.len());

            println!("Unpacking to {:?}...", output);
            archivist::unpack(&data, &output)?;
            println!("Done.");
        }
        Commands::Gui { image } => {
            // Start macroquad
            let future = gui_main(image);
            let mut conf = macroquad::conf::Conf::default();
            conf.miniquad_conf.window_title = "Pixel Archivist".to_string();
            conf.miniquad_conf.window_width = 800;
            conf.miniquad_conf.window_height = 600;

            macroquad::Window::from_config(conf, future);
        }
        Commands::SelfPortrait { output } => {
            println!("Taking a selfie...");
            let current_dir = std::env::current_dir()?;
            let mut root = current_dir.clone();
            // Simple heuristic: walk up until Cargo.toml found or root
            loop {
                if root.join("Cargo.toml").exists() {
                    break;
                }
                if !root.pop() {
                    return Err(anyhow::anyhow!("Could not find Cargo.toml"));
                }
            }
            println!("Found project root at {:?}", root);
            run_pack(&root, &output)?;
        }
    }
    Ok(())
}

fn run_pack(dir: &PathBuf, output: &PathBuf) -> Result<()> {
    println!("Packing {:?}...", dir);
    let data = archivist::pack(dir)?;
    println!("Compressed size: {} bytes", data.len());

    println!("Calculating dimensions...");
    let (width, height) = art::calculate_dimensions(data.len());
    println!("Target Resolution: {}x{}", width, height);

    println!("Mapping stars...");
    let starmap = art::generate_star_map(dir, width, height);
    println!("Found {} stars (files)", starmap.stars.len());

    println!("Generating cover...");
    let cover = art::generate_cover(&data, width, height, Some(&starmap));

    println!("Embedding data...");
    let stego_image = stego::embed(cover, &data)?;

    stego_image.save(output)?;
    println!("Saved to {:?}", output);
    Ok(())
}

use art::StarMap;
use macroquad::prelude::*;

async fn gui_main(path: Option<PathBuf>) {
    let mut texture: Option<Texture2D> = None;
    let mut starmap: Option<StarMap> = None;

    if let Some(p) = path {
        if let Ok(img) = image::open(&p) {
            let rgba = img.to_rgba8();
            let width = rgba.width();
            let height = rgba.height();
            let bytes = rgba.into_raw();

            // Try to extract starmap from image data
            if let Ok(data) = stego::extract(&img) {
                println!("Data extracted successfully in GUI. Size: {}", data.len());
                if let Ok(entries) = archivist::list_entries(&data) {
                    println!("Found {} entries in archive.", entries.len());
                    // Reconstruct StarMap
                    starmap = Some(art::generate_star_map_from_entries(&entries, width, height));
                } else {
                    println!("Failed to list entries (not a valid tar.gz?)");
                }
            } else {
                println!("Failed to extract stego data.");
            }

            // Convert to macroquad Image
            let mq_img = Image {
                bytes,
                width: width as u16,
                height: height as u16,
            };

            texture = Some(Texture2D::from_image(&mq_img));
        }
    }

    loop {
        clear_background(BLACK);

        if let Some(tex) = &texture {
            let screen_w = screen_width();
            let screen_h = screen_height();

            // Draw texture scaling to screen
            draw_texture_ex(
                tex,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_w, screen_h)),
                    ..Default::default()
                },
            );

            // Draw Stars Overlay
            if let Some(map) = &starmap {
                let mouse_pos = mouse_position();
                let tex_w = tex.width();
                let tex_h = tex.height();

                // Scale factor
                let scale_x = screen_w / tex_w;
                let scale_y = screen_h / tex_h;

                for star in &map.stars {
                    let screen_x = star.x * scale_x;
                    let screen_y = star.y * scale_y;
                    let screen_r = star.radius * scale_x.max(scale_y); // Approximate

                    // Draw subtle indicator
                    draw_circle_lines(
                        screen_x,
                        screen_y,
                        screen_r + 2.0,
                        1.0,
                        Color::new(1.0, 1.0, 1.0, 0.3),
                    );

                    // Hover check
                    let dx = screen_x - mouse_pos.0;
                    let dy = screen_y - mouse_pos.1;
                    if dx * dx + dy * dy < (screen_r + 5.0).powi(2) {
                        // Highlight
                        draw_circle_lines(screen_x, screen_y, screen_r + 4.0, 2.0, YELLOW);

                        // Tooltip
                        let text = format!("{} ({})", star.path, star.size);
                        let dims = measure_text(&text, None, 20, 1.0);
                        draw_rectangle(
                            mouse_pos.0 + 10.0,
                            mouse_pos.1 - 20.0,
                            dims.width + 10.0,
                            dims.height + 10.0,
                            Color::new(0.0, 0.0, 0.0, 0.8),
                        );
                        draw_text(&text, mouse_pos.0 + 15.0, mouse_pos.1 - 5.0, 20.0, WHITE);
                    }
                }
            }

            draw_text("Pixel Archivist - Visual Mode", 20.0, 30.0, 30.0, GREEN);
        } else {
            draw_text(
                "Usage: cargo run -- gui output.png",
                20.0,
                300.0,
                30.0,
                WHITE,
            );
        }

        next_frame().await
    }
}
