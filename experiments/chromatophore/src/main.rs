use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use macroquad::prelude::*;
use image::{GenericImageView, Pixel};

mod stego;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default to view mode if args < 2 (but needs image).
    // If we have specific commands, run CLI mode.
    if args.len() > 1 && (args[1] == "embed" || args[1] == "decode" || args[1] == "generate") {
        cli_main(args);
    } else {
        // Check if we have an image arg for view mode, or just run view mode (which might print usage if empty)
        macroquad::Window::new("Chromatophore", view_wrapper(args));
    }
}

fn cli_main(args: Vec<String>) {
    let command = &args[1];
    match command.as_str() {
        "embed" => {
            if args.len() != 5 {
                println!("Usage: chromatophore embed <image> <payload> <output>");
                return;
            }
            if let Err(e) = embed_mode(&args[2], &args[3], &args[4]) {
                println!("Error: {}", e);
            }
        }
        "generate" => {
             if args.len() != 3 {
                println!("Usage: chromatophore generate <output.png>");
                return;
            }
            let output_path = &args[2];
            let img = image::ImageBuffer::from_fn(100, 100, |x, y| {
                image::Rgba([x as u8, y as u8, 0, 255])
            });
            let _ = img.save(output_path);
            println!("Generated {}", output_path);
        }
        "decode" => {
            if args.len() != 3 {
                println!("Usage: chromatophore decode <image>");
                return;
            }
            if let Err(e) = decode_mode(&args[2]) {
                 println!("Error: {}", e);
            }
        }
        _ => {}
    }
}

async fn view_wrapper(args: Vec<String>) {
    // If no args, print usage and exit (or wait)
    if args.len() < 2 {
        print_usage();
        // Wait a bit or just return
        return;
    }

    let command = &args[1];
    // If command is "view", image is args[2].
    // If command is just image path, image is args[1].
    let image_path = if command == "view" {
        if args.len() < 3 {
            println!("Usage: chromatophore view <image>");
            return;
        }
        &args[2]
    } else {
        &args[1]
    };

    view_mode(image_path).await;
}

fn print_usage() {
    println!("Chromatophore - LSB Steganography Tool");
    println!("Usage:");
    println!("  chromatophore embed <image> <payload> <output>");
    println!("  chromatophore decode <image>");
    println!("  chromatophore view <image>");
    println!("  chromatophore <image>");
}

fn embed_mode(image_path: &str, payload_path: &str, output_path: &str) -> Result<()> {
    println!("Loading image: {}", image_path);
    let mut img = image::open(image_path)?;

    println!("Loading payload: {}", payload_path);
    let payload = fs::read(payload_path)?;

    println!("Encoding {} bytes...", payload.len());
    stego::encode(&mut img, &payload)?;

    println!("Saving to: {}", output_path);
    img.save(output_path)?;
    println!("Done.");
    Ok(())
}

fn decode_mode(image_path: &str) -> Result<()> {
    let img = image::open(image_path)?;
    let payload = stego::decode(&img)?;
    println!("Decoded {} bytes.", payload.len());
    if let Ok(s) = String::from_utf8(payload.clone()) {
        println!("Content:\n{}", s);
    } else {
        println!("Content is binary.");
    }
    Ok(())
}

async fn view_mode(image_path: &str) {
    let img_result = image::open(image_path);
    if let Err(e) = img_result {
        println!("Failed to open image: {}", e);
        return;
    }
    let img = img_result.unwrap();
    let (w, h) = img.dimensions();

    // Create Texture
    let rgba8 = img.to_rgba8();
    let mq_image = macroquad::texture::Image {
        width: w as u16,
        height: h as u16,
        bytes: rgba8.into_raw(),
    };
    let texture = Texture2D::from_image(&mq_image);

    // Create Glitch Texture (amplify LSBs)
    let mut glitch_bytes = Vec::with_capacity((w * h * 4) as usize);

    for i in (0..mq_image.bytes.len()).step_by(4) {
        let r = (mq_image.bytes[i] & 3) * 85;
        let g = (mq_image.bytes[i+1] & 3) * 85;
        let b = (mq_image.bytes[i+2] & 3) * 85;
        glitch_bytes.extend_from_slice(&[r, g, b, 255]);
    }
    let glitch_image = macroquad::texture::Image {
        width: w as u16,
        height: h as u16,
        bytes: glitch_bytes,
    };
    let glitch_texture = Texture2D::from_image(&glitch_image);

    // Decode Payload
    let payload = stego::decode(&img).unwrap_or_default();
    let script = String::from_utf8(payload).ok();

    // Initialize VM
    let mut vm = vm::VM::new();
    if let Some(code) = &script {
        println!("Script found! Compiling...");
        if let Err(e) = vm.load_script(code) {
             println!("Script Error: {}", e);
        } else {
             println!("Script loaded successfully.");
        }
    } else {
        println!("No valid script found in payload.");
    }

    let mut show_glitch = false;

    loop {
        clear_background(BLACK);

        let screen_ratio = screen_width() / screen_height();
        let image_ratio = w as f32 / h as f32;

        let draw_w;
        let draw_h;

        // Scale to fit
        if screen_ratio > image_ratio {
            draw_h = screen_height();
            draw_w = draw_h * image_ratio;
        } else {
            draw_w = screen_width();
            draw_h = draw_w / image_ratio;
        }

        let offset_x = (screen_width() - draw_w) / 2.0;
        let offset_y = (screen_height() - draw_h) / 2.0;

        let params = DrawTextureParams {
            dest_size: Some(vec2(draw_w, draw_h)),
            ..Default::default()
        };

        if show_glitch {
            draw_texture_ex(&glitch_texture, offset_x, offset_y, WHITE, params.clone());
        } else {
            draw_texture_ex(&texture, offset_x, offset_y, WHITE, params.clone());
        }

        // Run Script
        // Script draws on top of image
        if let Some(_) = &script {
             if let Err(_e) = vm.run() {
                 // println!("Runtime Error: {}", e);
             }
        }

        // UI Overlay
        draw_text("Chromatophore", 10.0, 20.0, 30.0, WHITE);
        if show_glitch {
             draw_text("GLITCH MODE", 10.0, 50.0, 30.0, RED);
        } else {
             draw_text("Press 'G' for Glitch Mode", 10.0, 50.0, 20.0, GRAY);
        }

        if is_key_pressed(KeyCode::G) {
            show_glitch = !show_glitch;
        }

        next_frame().await;
    }
}
