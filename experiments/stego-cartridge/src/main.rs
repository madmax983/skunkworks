use clap::Parser;
use macroquad::prelude::*;
use std::path::PathBuf;

use stego_cartridge::stego;
use stego_cartridge::vm::{SCREEN_HEIGHT, SCREEN_WIDTH, VM};

#[derive(Parser)]
#[command(name = "stego-cartridge")]
#[command(about = "Steganographic Fantasy Console", long_about = None)]
struct Cli {
    /// Optional cartridge file to load on start
    cartridge: Option<PathBuf>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "StegoCartridge".to_owned(),
        window_width: 512,
        window_height: 512,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cli = Cli::parse();
    play_cartridge(cli.cartridge).await;
}

async fn play_cartridge(initial_cartridge: Option<PathBuf>) {
    let mut vm = VM::new();
    let mut cartridge_loaded = false;

    // Texture for the VM screen
    let screen_texture = Texture2D::from_image(&Image::gen_image_color(
        SCREEN_WIDTH as u16,
        SCREEN_HEIGHT as u16,
        BLACK,
    ));
    screen_texture.set_filter(FilterMode::Nearest);

    // Buffer for texture update
    let mut screen_image = Image::gen_image_color(SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, BLACK);

    // Load initial if provided
    if let Some(path) = initial_cartridge {
        if let Err(e) = load_cartridge(&mut vm, &path) {
            eprintln!("Failed to load cartridge: {}", e);
        } else {
            cartridge_loaded = true;
        }
    }

    loop {
        // Handle File Drop
        #[cfg(not(target_arch = "wasm32"))]
        if macroquad::miniquad::window::dropped_file_count() > 0 {
            if let Some(file) = macroquad::miniquad::window::dropped_file_path(0) {
                println!("Dropped file: {:?}", file);
                if let Err(e) = load_cartridge(&mut vm, &PathBuf::from(file)) {
                    eprintln!("Failed to load cartridge: {}", e);
                } else {
                    cartridge_loaded = true;
                }
            }
        }

        if cartridge_loaded {
            // Run VM
            vm.run_frame(1000); // 1000 instructions per frame

            // Update Texture
            for i in 0..vm.screen.len() {
                let color_idx = vm.screen[i];
                let color = palette(color_idx);
                let x = (i % SCREEN_WIDTH) as u32;
                let y = (i / SCREEN_WIDTH) as u32;
                screen_image.set_pixel(x, y, color);
            }
            screen_texture.update(&screen_image);
        } else {
            // Draw "INSERT CARTRIDGE" static
            for pixel in screen_image.get_image_data_mut() {
                for i in 0..pixel.len() {
                    pixel[i] = rand::gen_range(0, 255);
                }
            }
            screen_texture.update(&screen_image);
        }

        clear_background(DARKGRAY);

        // Draw Screen Scaled
        let scale =
            (screen_width() / SCREEN_WIDTH as f32).min(screen_height() / SCREEN_HEIGHT as f32);
        let dest_w = SCREEN_WIDTH as f32 * scale;
        let dest_h = SCREEN_HEIGHT as f32 * scale;
        let dest_x = (screen_width() - dest_w) / 2.0;
        let dest_y = (screen_height() - dest_h) / 2.0;

        draw_texture_ex(
            &screen_texture,
            dest_x,
            dest_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                ..Default::default()
            },
        );

        if !cartridge_loaded {
            draw_text("DROP CARTRIDGE HERE", 10.0, 30.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}

fn load_cartridge(vm: &mut VM, path: &PathBuf) -> Result<(), String> {
    let img = image::open(path).map_err(|e| e.to_string())?.to_rgba8();
    let bytecode = stego::extract(&img)?;
    println!("Extracted {} bytes. Loading...", bytecode.len());
    vm.load_program(&bytecode);
    Ok(())
}

fn palette(idx: u8) -> Color {
    // Pico-8 paletteish
    match idx % 16 {
        0 => BLACK,
        1 => DARKBLUE,
        2 => DARKPURPLE,
        3 => DARKGREEN,
        4 => BROWN,
        5 => DARKGRAY,
        6 => LIGHTGRAY,
        7 => WHITE,
        8 => RED,
        9 => ORANGE,
        10 => YELLOW,
        11 => GREEN,
        12 => BLUE,
        13 => VIOLET, // Indigo
        14 => PINK,
        15 => BEIGE,
        _ => BLACK,
    }
}
