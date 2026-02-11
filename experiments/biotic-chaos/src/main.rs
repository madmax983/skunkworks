mod simulation;

use macroquad::prelude::*;
use simulation::Lattice;

const GRID_W: usize = 400;
const GRID_H: usize = 300;

#[macroquad::main("Biotic Chaos")]
async fn main() {
    let mut lattice = Lattice::new(GRID_W, GRID_H);

    // Texture for the lattice
    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    // Bifurcation history
    // We track the middle row over time
    // History buffer: Ring buffer of columns?
    // Actually, standard bifurcation diagram has Parameter on X, Value on Y.
    // Here we have Space on X (which maps to Parameter r), Time on Y?
    // Let's do a "Space-Time" slice at the bottom.
    // X axis = Space (Lattice X), Y axis = Time (History).
    const HISTORY_H: usize = 100;
    let mut history_image = Image::gen_image_color(GRID_W as u16, HISTORY_H as u16, BLACK);
    let history_texture = Texture2D::from_image(&history_image);
    history_texture.set_filter(FilterMode::Nearest);

    // Scroll offset for history (simulating ring buffer by shifting texture UVs? No, easier to just shift pixels)
    // Shifting pixels is expensive.
    // Let's just draw the current row at the bottom and scroll the rest up?
    // Or just use a ring buffer index and draw it with UV shift.

    let mut frame_count = 0;
    let mut paused = false;

    loop {
        // --- Update ---
        if !paused {
            lattice.update();
            frame_count += 1;
        }

        // --- Interaction ---
        let screen_w = screen_width();
        let screen_h = screen_height();
        let lattice_h = screen_h * 0.8; // Lattice takes top 80%

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if my <= lattice_h {
                // Map mouse to grid
                let gx = (mx / screen_w * GRID_W as f32) as usize;
                let gy = (my / lattice_h * GRID_H as f32) as usize;

                // Paint high R (Chaos)
                lattice.paint_r(gx, gy, 20, 3.9);
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
             let (mx, my) = mouse_position();
             if my <= lattice_h {
                 let gx = (mx / screen_w * GRID_W as f32) as usize;
                 let gy = (my / lattice_h * GRID_H as f32) as usize;
                 lattice.perturb(gx, gy, 10);
             }
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            lattice = Lattice::new(GRID_W, GRID_H);
        }

        // Adjust epsilon
        if is_key_down(KeyCode::Up) {
            lattice.epsilon = (lattice.epsilon + 0.001).min(1.0);
        }
        if is_key_down(KeyCode::Down) {
            lattice.epsilon = (lattice.epsilon - 0.001).max(0.0);
        }

        // --- Render to Texture ---
        // Update main lattice texture
        for (i, val) in lattice.cells.iter().enumerate() {
            let x = i % GRID_W;
            let y = i / GRID_W;

            // Color mapping
            // 0.0 -> Black
            // 0.0 - 0.5 -> Blue gradient
            // 0.5 - 0.8 -> Cyan/Green (Stable)
            // 0.8 - 1.0 -> White/Red (High pop)
            // Ideally we'd visualize instability (variance), but raw value is okay.

            let color = if *val < 0.01 {
                BLACK
            } else if *val < 0.5 {
                Color::new(0.0, 0.0, *val * 2.0, 1.0)
            } else if *val < 0.8 {
                Color::new(0.0, (*val - 0.5) * 3.3, 1.0, 1.0)
            } else {
                 Color::new((*val - 0.8) * 5.0, 1.0, 1.0, 1.0)
            };

            image.set_pixel(x as u32, y as u32, color);
        }
        texture.update(&image);

        // Update history slice (Space-Time diagram of middle row)
        // Shift history up? Too slow.
        // Let's just draw the history as a separate loop if needed,
        // Or write to a rolling line.
        let current_line_idx = frame_count % HISTORY_H;
        let mid_y = GRID_H / 2;
        let start_idx = mid_y * GRID_W;
        for x in 0..GRID_W {
            let val = lattice.cells[start_idx + x];
             let color = if val < 0.01 {
                BLACK
            } else {
                Color::new(val, val, val, 1.0)
            };
            history_image.set_pixel(x as u32, current_line_idx as u32, color);
        }
        history_texture.update(&history_image);

        // --- Draw to Screen ---
        clear_background(DARKGRAY);

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Draw Lattice
        // Take up 80% of height
        let lattice_h = screen_h * 0.8;
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_w, lattice_h)),
            ..Default::default()
        });

        // Draw History (Bifurcation Slice) at bottom
        // We need to draw it with UV shift to make it look like scrolling
        // We want to draw from uv_y_start to 1.0, then 0.0 to uv_y_start
        // Actually, easiest is just draw it fixed for now, allowing the "scanline" to move.

        let history_h = screen_h - lattice_h;
        draw_texture_ex(&history_texture, 0.0, lattice_h, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_w, history_h)),
            ..Default::default()
        });

        // Draw scanline on history
        let scan_y = lattice_h + (current_line_idx as f32 / HISTORY_H as f32) * history_h;
        draw_line(0.0, scan_y, screen_w, scan_y, 2.0, RED);

        // --- UI ---
        draw_text("Biotic Chaos", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Epsilon: {:.3} (Up/Down)", lattice.epsilon), 10.0, 60.0, 20.0, YELLOW);
        draw_text("Left Click: Paint Chaos (r=3.9)", 10.0, 80.0, 20.0, RED);
        draw_text("Right Click: Perturb", 10.0, 100.0, 20.0, GREEN);
        draw_text("Space: Pause | R: Reset", 10.0, 120.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
