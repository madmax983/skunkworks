use clap::{Parser, Subcommand};
use macroquad::prelude::*;
use std::fs;
use std::path::PathBuf;

// We assume stego is exposed as lib
use stardust_compiler::stego;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a file into an image
    Encode {
        /// Input file to hide
        input: PathBuf,
        /// Output image path
        #[arg(short, long, default_value = "stardust.png")]
        output: PathBuf,
        /// Optional cover image. If not provided, generates a Nebula.
        #[arg(short, long)]
        cover: Option<PathBuf>,
    },
    /// Decode a file from an image
    Decode {
        /// Input image with hidden data
        input: PathBuf,
        /// Output file for extracted data (optional, prints to stdout if text)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[macroquad::main("Stardust Compiler")]
async fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Encode {
                input,
                output,
                cover,
            } => {
                run_encode(input, output, cover).await;
            }
            Commands::Decode { input, output } => {
                run_decode(input, output).await;
            }
        }
    } else {
        run_demo().await;
    }
}

async fn run_demo() {
    let mut data = vec![0u8; 1000];
    for i in 0..1000 {
        data[i] = (i % 255) as u8;
    }

    let mut img = stego::generate_nebula(800, 600, 12345);

    stego::encode(&mut img, &data).unwrap();

    let texture = Texture2D::from_image(&macroquad::texture::Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_raw().clone(),
    });

    let mut scan_y = 0.0;

    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        draw_line(0.0, scan_y, screen_width(), scan_y, 2.0, GREEN);
        scan_y += 5.0;
        if scan_y > screen_height() {
            scan_y = 0.0;
        }

        draw_text("DEMO MODE: Encoding Random Data", 20.0, 30.0, 30.0, WHITE);

        next_frame().await;
    }
}

async fn run_encode(input: PathBuf, output: PathBuf, cover: Option<PathBuf>) {
    let data = fs::read(&input).expect("Failed to read input file");

    let mut img = if let Some(path) = cover {
        image::open(path)
            .expect("Failed to open cover image")
            .to_rgba8()
    } else {
        stego::generate_nebula(800, 600, macroquad::rand::gen_range(0, 10000) as u64)
    };

    // Perform encoding
    stego::encode(&mut img, &data).expect("Encoding failed");

    // Save
    img.save(&output).expect("Failed to save output image");
    println!("Encoded {} bytes into {}", data.len(), output.display());

    // Visualizer
    let texture = Texture2D::from_image(&macroquad::texture::Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_raw().clone(),
    });

    let mut scan_y = 0.0;

    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Glitch effect on scanline
        let offset = macroquad::rand::gen_range(-5.0, 5.0);
        draw_line(0.0, scan_y, screen_width(), scan_y + offset, 2.0, BLUE);

        scan_y += 4.0;
        if scan_y > screen_height() {
            scan_y = 0.0;
            // For CLI usage, we might want to exit after visualization,
            // but for "Moonshot" let's keep it open until user closes.
            // Or maybe exit after a few passes?
            // Let's keep it open.
        }

        draw_text(
            &format!("ENCODED: {}", input.display()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Press ESC to exit",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

async fn run_decode(input: PathBuf, output: Option<PathBuf>) {
    let img = image::open(&input)
        .expect("Failed to open input image")
        .to_rgba8();

    // Perform decoding
    let data = stego::decode(&img).expect("Decoding failed");

    let message_display = if let Some(path) = output {
        fs::write(&path, &data).expect("Failed to save decoded data");
        println!("Decoded {} bytes to {}", data.len(), path.display());
        format!("SAVED TO {}", path.display())
    } else {
        // Try to print as string
        if let Ok(s) = String::from_utf8(data.clone()) {
            println!("Decoded Message:\n{}", s);
            // Show first 50 chars
            s.chars().take(50).collect::<String>()
        } else {
            println!("Decoded {} bytes (binary data)", data.len());
            format!("BINARY DATA ({} bytes)", data.len())
        }
    };

    // Visualizer
    let texture = Texture2D::from_image(&macroquad::texture::Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_raw().clone(),
    });

    let mut scan_y = 0.0;

    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        draw_line(0.0, scan_y, screen_width(), scan_y, 2.0, RED);
        scan_y += 4.0;
        if scan_y > screen_height() {
            scan_y = 0.0;
        }

        draw_text("DECODING...", 20.0, 30.0, 30.0, WHITE);
        draw_text(&message_display, 20.0, 60.0, 20.0, GREEN);

        draw_text(
            "Press ESC to exit",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
