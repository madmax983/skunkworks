//! # Mandala Cipher ☸️
//!
//! > "The universe is built on a plan the profound symmetry of which is somehow present in the inner structure of our intellect." - Paul Valéry
//!
//! **Mandala Cipher** is a visual steganography tool that encodes information into sacred geometry. It transforms text into a radially symmetric mandala, where the placement, color, and shape of "Jewels" carry the data.
//!
//! ## 🌟 Concept
//!
//! The cipher uses a **Canonical Sector** (a slice of the pie) to store the payload. This sector is then replicated `N` times to form a complete mandala.
//! - **Data Encoding**: Each byte is split into two 4-bit nibbles.
//! - **Jewels**: Each nibble determines a Jewel's attributes:
//!   - **Shape** (2 bits): Circle or Square.
//!   - **Color** (3 bits): Red, Green, Blue, Yellow, Purple, Cyan, White, Black.
//! - **Chaff**: Unused slots are filled with "Chaff" jewels (Triangles and Diamonds), which are visually consistent but ignored by the decoder.
//!
//! ## 🎮 Usage
//!
//! Run the interactive visualizer:
//!
//! ```bash
//! cargo run -p mandala-cipher
//! ```
//!
//! - **Type**: Enter text to encode it in real-time.
//! - **Backspace**: Delete characters.
//! - **S**: Save the current mandala as `mandala_output.png`.
//!
//! ## 🧠 Decoding
//!
//! The `decode` function in `lib.rs` reverses the process. It ignores Triangles and Diamonds (noise) and reconstructs the byte stream from Circles and Squares.
//!
//! ## 🧪 Status
//!
//! - [x] Core Logic (Encode/Decode)
//! - [x] Visual Renderer (Macroquad)
//! - [x] Rotation & Bloom
//! - [ ] Computer Vision Decoder (Recover from PNG)
//!
//! ## 🌌 Moonshot
//!
//! This experiment combines **Visual Ciphers** with **Source Code Obfuscation** principles. The goal is to distribute code as art.
//!
use macroquad::prelude::*;
use mandala_cipher::{encode, Color as JewelColor, Mandala, MandalaConfig, Shape};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode, exiting immediately to avoid XOpenDisplay panic.");
        return;
    }
    // Block on async main to run macroquad
    macroquad::Window::from_config(
        macroquad::window::Conf {
            window_title: "Mandala Cipher".to_string(),
            ..Default::default()
        },
        async_main(),
    );
}

async fn async_main() {
    let mut payload = String::from("Genesis");
    let config = MandalaConfig {
        rings: 20,
        segments_per_ring: 12,
        symmetry_order: 12,
    };

    let mut rotation = 0.0;

    loop {
        clear_background(BLACK);

        rotation += 0.002;

        // Input handling
        while let Some(c) = get_char_pressed() {
            if c.is_ascii_graphic() || c == ' ' {
                payload.push(c);
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            payload.pop();
        }

        if is_key_pressed(KeyCode::S) {
            let img = get_screen_data();
            if let Some(buffer) =
                image::RgbaImage::from_raw(img.width as u32, img.height as u32, img.bytes)
            {
                match buffer.save("mandala_output.png") {
                    Ok(_) => println!("Saved to mandala_output.png"),
                    Err(e) => println!("Error saving image: {}", e),
                }
            }
        }

        // Encode
        let mandala = encode(payload.as_bytes(), &config);

        // Draw
        draw_mandala(&mandala, rotation);

        // UI
        draw_text(&format!("Payload: {}", payload), 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "Type to encode... [S] to Save PNG",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}

fn draw_mandala(mandala: &Mandala, global_rotation: f32) {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let max_radius = screen_width().min(screen_height()) * 0.45;
    let ring_step = max_radius / mandala.config.rings as f32;

    let sector_angle = std::f32::consts::PI * 2.0 / mandala.config.symmetry_order as f32;

    for symmetry_idx in 0..mandala.config.symmetry_order {
        let base_angle = symmetry_idx as f32 * sector_angle + global_rotation;

        for (idx, jewel_opt) in mandala.jewels.iter().enumerate() {
            if let Some(jewel) = jewel_opt {
                let ring_idx = idx / mandala.config.segments_per_ring;
                let seg_idx = idx % mandala.config.segments_per_ring;

                let r_inner = ring_idx as f32 * ring_step;
                // let r_outer = (ring_idx + 1) as f32 * ring_step;
                let r_center = r_inner + ring_step * 0.5;

                let angle_step = sector_angle / mandala.config.segments_per_ring as f32;
                let theta = base_angle + seg_idx as f32 * angle_step + angle_step / 2.0;

                let pos = center + vec2(theta.cos(), theta.sin()) * r_center;
                let size = ring_step * 0.4;

                let color = match jewel.color {
                    JewelColor::Red => RED,
                    JewelColor::Green => GREEN,
                    JewelColor::Blue => BLUE,
                    JewelColor::Yellow => YELLOW,
                    JewelColor::Purple => PURPLE,
                    JewelColor::Cyan => SKYBLUE,
                    JewelColor::White => WHITE,
                    JewelColor::Black => DARKGRAY,
                };

                // Rotation for poly
                let rot = theta * 180.0 / std::f32::consts::PI;

                match jewel.shape {
                    Shape::Circle => draw_circle(pos.x, pos.y, size, color),
                    Shape::Square => {
                        draw_rectangle(pos.x - size, pos.y - size, size * 2.0, size * 2.0, color)
                    }
                    Shape::Triangle => draw_poly(pos.x, pos.y, 3, size, rot, color),
                    Shape::Diamond => draw_poly(pos.x, pos.y, 4, size, rot + 45.0, color),
                }
            }
        }
    }
}
