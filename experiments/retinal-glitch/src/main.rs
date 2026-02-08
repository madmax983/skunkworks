mod neuron;
mod retina;
mod input;

use macroquad::prelude::*;
use input::VisualField;
use retina::Retina;

#[macroquad::main("Retinal Glitch")]
async fn main() {
    let w = 128;
    let h = 128;

    let mut retina = Retina::new(w, h);
    let mut visual_field = VisualField::new(w, h);

    // Textures for visualization
    let input_texture = Texture2D::from_image(&Image::gen_image_color(w as u16, h as u16, BLACK));
    let bipolar_texture = Texture2D::from_image(&Image::gen_image_color(w as u16, h as u16, BLACK));
    let ganglion_texture = Texture2D::from_image(&Image::gen_image_color(w as u16, h as u16, BLACK));

    input_texture.set_filter(FilterMode::Nearest);
    bipolar_texture.set_filter(FilterMode::Nearest);
    ganglion_texture.set_filter(FilterMode::Nearest);

    let mut input_image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let mut bipolar_image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let mut ganglion_image = Image::gen_image_color(w as u16, h as u16, BLACK);

    loop {
        let dt = get_frame_time();

        // 1. Update Physics
        visual_field.update(dt);
        let spikes = retina.update(&visual_field.buffer);
        visual_field.inject_feedback(&spikes);

        if is_key_pressed(KeyCode::Space) {
            visual_field.warp_x.fill(0.0);
            visual_field.warp_y.fill(0.0);
        }

        // 2. Render to Images
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;

                // Input: Grayscale
                let val = visual_field.buffer[idx];
                let c = (val * 255.0).clamp(0.0, 255.0) as u8;
                input_image.set_pixel(x as u32, y as u32, Color::from_rgba(c, c, c, 255));

                // Bipolar: Red (Positive) / Blue (Negative)
                let b = retina.bipolar[idx];
                // Scale for visibility
                let b_norm = (b * 5.0).tanh(); // -1..1
                let r = if b_norm > 0.0 { (b_norm * 255.0) as u8 } else { 0 };
                let g = 0;
                let bl = if b_norm < 0.0 { (-b_norm * 255.0) as u8 } else { 0 };
                bipolar_image.set_pixel(x as u32, y as u32, Color::from_rgba(r, g, bl, 255));

                // Ganglion: Voltage (Green)
                let v = retina.ganglion[idx].v;
                // Map -65..30 to 0..1
                let v_norm = ((v + 70.0) / 100.0).clamp(0.0, 1.0);
                let g_val = (v_norm * 255.0) as u8;
                 ganglion_image.set_pixel(x as u32, y as u32, Color::from_rgba(0, g_val, 0, 255));
            }
        }

        // Draw Spikes as bright dots
        for (sx, sy) in &spikes {
             ganglion_image.set_pixel(*sx as u32, *sy as u32, WHITE);
        }

        // 3. Update Textures
        input_texture.update(&input_image);
        bipolar_texture.update(&bipolar_image);
        ganglion_texture.update(&ganglion_image);

        // 4. Draw to Screen
        clear_background(DARKGRAY);

        let sw = screen_width();
        let sh = screen_height();

        let view_w = sw / 3.0;
        let view_h = view_w; // Square aspect

        let y_pos = (sh - view_h) / 2.0;

        draw_texture_ex(&input_texture, 0.0, y_pos, WHITE, DrawTextureParams {
            dest_size: Some(vec2(view_w, view_h)),
            ..Default::default()
        });
        draw_rectangle_lines(0.0, y_pos, view_w, view_h, 2.0, GRAY);
        draw_text("Input (Retina)", 10.0, y_pos - 10.0, 20.0, WHITE);

        draw_texture_ex(&bipolar_texture, view_w, y_pos, WHITE, DrawTextureParams {
            dest_size: Some(vec2(view_w, view_h)),
            ..Default::default()
        });
        draw_rectangle_lines(view_w, y_pos, view_w, view_h, 2.0, GRAY);
        draw_text("Bipolar (Edges)", view_w + 10.0, y_pos - 10.0, 20.0, WHITE);

        draw_texture_ex(&ganglion_texture, view_w * 2.0, y_pos, WHITE, DrawTextureParams {
            dest_size: Some(vec2(view_w, view_h)),
            ..Default::default()
        });
        draw_rectangle_lines(view_w * 2.0, y_pos, view_w, view_h, 2.0, GRAY);
        draw_text("Ganglion (Spikes)", view_w * 2.0 + 10.0, y_pos - 10.0, 20.0, WHITE);

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, YELLOW);
        draw_text(&format!("Spikes: {}", spikes.len()), 10.0, 40.0, 20.0, GREEN);
        draw_text("SPACE to Reset Warp", 10.0, 60.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
