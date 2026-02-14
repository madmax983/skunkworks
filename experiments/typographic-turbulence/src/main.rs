mod lbm;
mod particles;
use lbm::{FluidSim, HEIGHT, WIDTH};
use macroquad::color::hsl_to_rgb;
use macroquad::prelude::*;
use particles::ParticleSystem;

#[macroquad::main("Typographic Turbulence")]
async fn main() {
    let mut sim = FluidSim::new();
    let mut particle_system = ParticleSystem::new();

    // UI State
    let mut cursor_x = WIDTH / 2;
    let mut cursor_y = HEIGHT / 2;
    let mut show_fluid = true;
    let mut wind_tunnel = false;

    loop {
        let sw = screen_width();
        let sh = screen_height();
        let cell_w = sw / WIDTH as f32;
        let cell_h = sh / HEIGHT as f32;
        // Use a font size slightly larger than cell height to ensure visibility?
        // Or exactly cell height.
        // For drawing, let's use cell_h.
        let font_size = cell_h * 1.2;

        // Input: Interaction
        let dt = get_frame_time();

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;

            sim.add_density(gx, gy, 5.0);

            let delta = mouse_delta_position();
            // Scale delta to be meaningful impulse
            sim.add_velocity(gx, gy, delta.x * 200.0, delta.y * 200.0);
        }

        // Keyboard Interaction: Spawn Particles
        // We capture chars.
        while let Some(c) = get_char_pressed() {
            if !c.is_control() && c != '\n' && c != '\r' && c != '\u{8}' {
                // Spawn particle at cursor
                // Let's spawn a cluster or just one?
                // Just one for now.
                // Color: White?
                // Let's cycle colors or random.
                let color = hsl_to_rgb(rand::gen_range(0.0, 1.0), 0.8, 0.8);

                particle_system.spawn(cursor_x as f32, cursor_y as f32, c, color);

                // Add some velocity to fluid at cursor to "shoot" it?
                // Or let it float.
                // Let's add a small impulse in direction of typing?
                // Assuming left-to-right typing.
                sim.add_velocity(cursor_x, cursor_y, 0.5, 0.0);

                // Move cursor
                cursor_x += 1;
                if cursor_x >= WIDTH {
                    cursor_x = 0;
                    cursor_y = (cursor_y + 1).min(HEIGHT - 1);
                }
            }
        }

        // Navigation (Arrow keys)
        if is_key_pressed(KeyCode::Left) {
            cursor_x = cursor_x.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Right) {
            cursor_x = (cursor_x + 1).min(WIDTH - 1);
        }
        if is_key_pressed(KeyCode::Up) {
            cursor_y = cursor_y.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) {
            cursor_y = (cursor_y + 1).min(HEIGHT - 1);
        }

        // Toggle Fluid View
        if is_key_pressed(KeyCode::F) {
            show_fluid = !show_fluid;
        }

        // Toggle Wind Tunnel
        if is_key_pressed(KeyCode::W) {
            wind_tunnel = !wind_tunnel;
        }

        if wind_tunnel {
            // Inject velocity at left boundary
            for y in 1..HEIGHT - 1 {
                // Add velocity to the left edge
                sim.add_velocity(1, y, 0.2, 0.0);
            }
        }

        // Simulation Steps
        sim.step();
        particle_system.update(&sim, dt);

        // Rendering
        clear_background(BLACK);

        // 1. Draw Fluid Background (optional)
        if show_fluid {
            // We can draw a coarse grid or pixels.
            // Drawing 200x100 pixels is fast.
            // `draw_texture` with a constructed image?
            // Or just iterating rectangles is too slow? 20k rects might be okay for macroquad.
            // But let's try just drawing characters for fluid?
            // Or dots.
            // Let's draw small rects where Curl is high.

            for y in (0..HEIGHT).step_by(2) {
                for x in (0..WIDTH).step_by(2) {
                    let idx = y * WIDTH + x;
                    let curl = sim.curl[idx];
                    let abs_curl = curl.abs();

                    if abs_curl > 0.05 {
                        let color = if curl > 0.0 {
                            Color::new(1.0, 0.2, 0.2, abs_curl * 5.0) // Red for positive curl
                        } else {
                            Color::new(0.2, 0.2, 1.0, abs_curl * 5.0) // Blue for negative curl
                        };

                        draw_rectangle(
                            x as f32 * cell_w,
                            y as f32 * cell_h,
                            cell_w * 2.0,
                            cell_h * 2.0,
                            color,
                        );
                    }
                }
            }
        }

        // 2. Draw Particles
        for p in particle_system.particles() {
            // Fade out
            let alpha = 1.0 - (p.lifetime / p.max_lifetime).powf(2.0);
            if alpha <= 0.0 {
                continue;
            }

            let mut color = p.color;
            color.a = alpha;

            let px = p.position.x * cell_w;
            let py = p.position.y * cell_h;

            // Draw text
            draw_text(
                &p.char.to_string(),
                px,
                py + font_size, // Offset because draw_text y is baseline? No, usually top-left for some, baseline for others. Macroquad draw_text y is baseline.
                font_size,
                color,
            );
        }

        // 3. Draw Cursor
        let cursor_screen_x = cursor_x as f32 * cell_w;
        let cursor_screen_y = cursor_y as f32 * cell_h;
        draw_rectangle_lines(cursor_screen_x, cursor_screen_y, cell_w, cell_h, 2.0, GREEN);

        // UI Info
        draw_text(
            &format!("Particles: {}", particle_system.count()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Type to add particles. Mouse to stir. F: Fluid, W: Wind Tunnel.",
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
