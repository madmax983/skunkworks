use macroquad::prelude::*;
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};

mod memory;
use memory::Memory;
mod decay;
use decay::Decay;
mod audio;

#[macroquad::main("Mnemosyne")]
async fn main() {
    let mut width = 512;
    let mut height = 512;
    let mut memory = Memory::new(width, height);

    let mut mq_image = Image {
        bytes: memory.perceived.as_raw().clone(),
        width: width as u16,
        height: height as u16,
    };
    let mut texture = Texture2D::from_image(&mq_image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // Dimensions & Placement
        let screen_w = screen_width();
        let screen_h = screen_height();
        let dest_w = width as f32;
        let dest_h = height as f32;
        let img_x = (screen_w - dest_w) / 2.0;
        let img_y = (screen_h - dest_h) / 2.0;

        // Input
        let (mx, my) = mouse_position();
        let rel_x = mx - img_x;
        let rel_y = my - img_y;

        let is_hovering = rel_x >= 0.0 && rel_x < dest_w && rel_y >= 0.0 && rel_y < dest_h;

        if is_hovering && is_mouse_button_pressed(MouseButton::Left) {
            // Recall at mouse position
            // The more you recall, the more it degrades eventually
            memory.recall(rel_x as u32, rel_y as u32, 40);

            // Sonification: The sound of memory reconstructing
            // We await here, which causes a slight "glitch" pause, reinforcing the effort of recall.
            let wav_data = audio::generate_drone(&memory.perceived);
            if let Ok(sound) = load_sound_from_bytes(&wav_data).await {
                play_sound(&sound, PlaySoundParams{ looped: false, volume: 0.5 });
            }
        }

        if is_key_down(KeyCode::Space) {
             // Full recall (expensive? No, just iterates pixels if we implemented full recall)
             // For now, let's just recall a random spot every frame to simulate "trying to remember everything"
             let rx = rand::gen_range(0, width);
             let ry = rand::gen_range(0, height);
             memory.recall(rx, ry, 100);
        }

        if is_key_pressed(KeyCode::R) {
            memory = Memory::new(width, height);
        }

        // File Drop
        #[cfg(not(target_arch = "wasm32"))]
        if macroquad::miniquad::window::dropped_file_count() > 0 {
            if let Some(path) = macroquad::miniquad::window::dropped_file_path(0) {
                if let Ok(img) = image::open(&path) {
                    println!("Loaded image: {:?}", path);
                    let rgba = img.to_rgba8();
                    memory = Memory::from_image(rgba);
                    width = memory.width;
                    height = memory.height;

                    // Re-create texture
                    let new_mq_image = Image {
                        bytes: memory.perceived.as_raw().clone(),
                        width: width as u16,
                        height: height as u16,
                    };
                    texture = Texture2D::from_image(&new_mq_image);
                    texture.set_filter(FilterMode::Nearest);
                    mq_image = new_mq_image;
                } else {
                    eprintln!("Failed to load image: {:?}", path);
                }
            }
        }

        // Logic: Decay
        memory.erode();

        // Update Texture
        // Update the bytes of our local Image buffer from the RgbaImage
        mq_image.bytes.copy_from_slice(memory.perceived.as_raw());
        texture.update(&mq_image);

        // Draw
        clear_background(BLACK);

        draw_texture(&texture, img_x, img_y, WHITE);

        // Interaction Feedback
        if is_hovering {
            draw_circle_lines(mx, my, 40.0, 2.0, YELLOW);
        }

        // UI
        draw_rectangle(0.0, 0.0, screen_w, 40.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text("MNEMOSYNE: The Fading Gallery", 10.0, 25.0, 20.0, WHITE);
        draw_text("Left Click: Recall | Space: Panicked Recall | R: Reset", 10.0, screen_h - 10.0, 16.0, GRAY);

        // Debug
        // draw_text(&format!("FPS: {}", get_fps()), screen_w - 100.0, 25.0, 20.0, GREEN);

        next_frame().await
    }
}
