mod lbm;
mod renderer;

use macroquad::prelude::*;
use lbm::{FluidSim, WIDTH, HEIGHT};
use renderer::render_ascii;

#[macroquad::main("Typographic Turbulence")]
async fn main() {
    let mut sim = FluidSim::new();
    let mut text_grid = vec![' '; WIDTH * HEIGHT];
    let mut cursor_x = WIDTH / 2;
    let mut cursor_y = HEIGHT / 2;

    // Optional: Load a font if needed, but default is fine for ASCII
    // let font = load_ttf_font("assets/font.ttf").await.ok();

    // Simulation loop
    loop {
        // Input Handling
        // Keyboard: Type to add obstacles
        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Enter => {
                    cursor_x = 0;
                    cursor_y = (cursor_y + 1).min(HEIGHT - 1);
                }
                KeyCode::Backspace => {
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    } else if cursor_y > 0 {
                        cursor_y -= 1;
                        cursor_x = WIDTH - 1;
                    }
                    let idx = cursor_y * WIDTH + cursor_x;
                    text_grid[idx] = ' ';
                    sim.set_obstacle(cursor_x, cursor_y, false);
                }
                _ => {}
            }
        }

        // Char input (macroquad collects chars)
        while let Some(c) = get_char_pressed() {
            if c.is_control() || c == '\n' || c == '\r' || c == '\u{8}' { continue; }

            let idx = cursor_y * WIDTH + cursor_x;
            text_grid[idx] = c;
            sim.set_obstacle(cursor_x, cursor_y, true);

            cursor_x += 1;
            if cursor_x >= WIDTH {
                cursor_x = 0;
                cursor_y = (cursor_y + 1).min(HEIGHT - 1);
            }
        }

        // Navigation (Arrow keys)
        if is_key_pressed(KeyCode::Left) { cursor_x = cursor_x.saturating_sub(1); }
        if is_key_pressed(KeyCode::Right) { cursor_x = (cursor_x + 1).min(WIDTH - 1); }
        if is_key_pressed(KeyCode::Up) { cursor_y = cursor_y.saturating_sub(1); }
        if is_key_pressed(KeyCode::Down) { cursor_y = (cursor_y + 1).min(HEIGHT - 1); }

        // Mouse: Add Fluid
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            // Map screen to grid
            // Grid is rendered to fit screen?
            // Let's assume we scale grid to screen.
            let cell_w = sw / WIDTH as f32;
            let cell_h = sh / HEIGHT as f32;

            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;

            sim.add_density(gx, gy, 5.0); // Add ink

            // Add velocity based on mouse movement delta?
            // macroquad doesn't give delta easily without tracking.
            // Let's just add random velocity or radial?
            // Or use get_mouse_delta() if available? `mouse_delta_position()`
            let delta = mouse_delta_position();
            sim.add_velocity(gx, gy, delta.x * 100.0, delta.y * 100.0);
        }

        // Simulation Step
        sim.step();

        // Rendering
        clear_background(BLACK);

        // Render Fluid
        let fluid_ascii = render_ascii(&sim);

        // We draw line by line to ensure alignment
        let sw = screen_width();
        let sh = screen_height();
        let _font_size = (sw / WIDTH as f32).min(sh / HEIGHT as f32) * 1.5; // Aspect ratio fix?
        // Actually, for monospaced, width is usually ~0.6 * height.
        // Let's just use a fixed font size that fits or calculate.
        // `draw_text` uses pixel size.
        // Let's try to fit vertically.
        let line_height = sh / HEIGHT as f32;
        let _char_width = sw / WIDTH as f32;

        // Draw Fluid (Cyan)
        let mut y_pos = 0.0;
        for line in fluid_ascii.lines() {
            // draw_text(line, 0.0, y_pos + line_height, font_size, BLUE);
            // Wait, draw_text doesn't allow stretching.
            // We need `draw_text_ex` with params.
            // But TextParams doesn't allow separate scaling X/Y.
            // So we rely on the font's aspect ratio.
            // If we use default font, we can just position lines.
            // If we want exact grid, we might need to draw char by char...
            // Or just draw the whole string and hope.

            // Better: Draw the string as one block?
            // No, newlines reset X but Y spacing is determined by font.
            // Let's draw line by line.
            draw_text(line, 0.0, y_pos + line_height, line_height, Color::new(0.0, 0.5, 1.0, 1.0));
            y_pos += line_height;
        }

        // Draw Text Overlay (White)
        // We iterate `text_grid` and draw only non-spaces.
        // Optimization: Construct a string?
        // Constructing a full string for text_grid is fast (20k chars).
        let mut text_overlay = String::with_capacity((WIDTH + 1) * HEIGHT);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let c = text_grid[y * WIDTH + x];
                text_overlay.push(c);
            }
            text_overlay.push('\n');
        }

        y_pos = 0.0;
        for line in text_overlay.lines() {
             draw_text(line, 0.0, y_pos + line_height, line_height, WHITE);
             y_pos += line_height;
        }

        // Cursor
        let _cursor_screen_x = cursor_x as f32 * (sw / WIDTH as f32); // Approximate
        // Actually, draw_text advance is handled by font.
        // We can't easily predict cursor position if we don't know font metrics.
        // But if we used monospaced, char width is constant?
        // Measure 'M'.
        let m_size = measure_text("M", None, line_height as u16, 1.0);
        let advance_x = m_size.width;

        // Re-draw fluid with correct spacing?
        // If we draw line by line, X spacing is determined by font.
        // We should calculate `font_size` such that `WIDTH * advance_x <= sw`.

        // Let's do a debug draw of cursor at `cursor_x * advance_x`.
        draw_rectangle(cursor_x as f32 * advance_x, cursor_y as f32 * line_height, advance_x, line_height, RED);

        next_frame().await
    }
}
