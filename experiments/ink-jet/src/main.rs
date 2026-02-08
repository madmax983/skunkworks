use ink_jet::fluid::FluidSolver;
use ink_jet::text::rasterize_text;
use macroquad::prelude::*;

const FLUID_SIZE: usize = 200;

#[macroquad::main("Ink Jet")]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");

    // Fluid parameters: dt, diffusion, viscosity
    // Viscosity 0.0001 allows nice swirls. Diffusion 0.0 keeps it sharp.
    let mut fluid = FluidSolver::new(FLUID_SIZE, 0.1, 0.0, 0.0001);

    // Rasterize text
    let text_grid = rasterize_text(font_data, "GENESIS", 40.0, FLUID_SIZE, FLUID_SIZE);

    // Set obstacles from text grid
    for y in 0..FLUID_SIZE {
        for x in 0..FLUID_SIZE {
            let idx = x + y * FLUID_SIZE;
            if text_grid[idx] {
                fluid.set_obstacle(x, y, true);
            }
        }
    }

    let mut image = Image::gen_image_color(FLUID_SIZE as u16, FLUID_SIZE as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Linear);

    loop {
        clear_background(BLACK);

        // --- Simulation ---

        // 1. Add "Wind" velocity from left
        for y in 0..FLUID_SIZE {
            // Add slight rightward velocity everywhere or just at inflow?
            // "Wind Tunnel" style: inflow boundary.
            fluid.add_velocity(1, y, 2.0, 0.0);

            // Add "Ink" density at inflow
            // Varying density to make it look like smoke
            let time = get_time() as f32;
            let noise = (y as f32 * 0.1 + time * 2.0).sin();
            if noise > 0.0 {
                fluid.add_density(1, y, noise * 2.0);
            }
        }

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            // Map screen coords to fluid grid
            // We draw the texture scaled to screen size.
            let fx = (mx / sw * FLUID_SIZE as f32) as usize;
            let fy = (my / sh * FLUID_SIZE as f32) as usize;

            if fx < FLUID_SIZE && fy < FLUID_SIZE {
                fluid.add_density(fx, fy, 50.0);
                fluid.add_velocity(
                    fx,
                    fy,
                    rand::gen_range(-5.0, 5.0),
                    rand::gen_range(-5.0, 5.0),
                );
            }
        }

        // 2. Step fluid
        fluid.step();

        // --- Rendering ---

        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                let idx = x + y * FLUID_SIZE;

                if fluid.obstacles[idx] {
                    image.set_pixel(x as u32, y as u32, MAGENTA); // Draw obstacles Magenta
                } else {
                    let d = fluid.density[idx];

                    let intensity = (d * 0.5).clamp(0.0, 1.0);
                    // Fluid color: Cyan
                    let color = Color::new(0.0, intensity, intensity, 1.0);

                    image.set_pixel(x as u32, y as u32, color);
                }
            }
        }

        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw UI
        draw_text("INK JET", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "Flowing past static text. Click to add ink.",
            20.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            screen_width() - 100.0,
            30.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
