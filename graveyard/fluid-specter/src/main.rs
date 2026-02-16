mod audio;
mod fluid;

use audio::AudioSystem;
use fluid::FluidSolver;
use macroquad::prelude::*;

const FLUID_SIZE: usize = 128;

#[macroquad::main("Fluid Specter")]
async fn main() -> anyhow::Result<()> {
    // Init Audio
    let mut audio = AudioSystem::new()?;

    // Init Fluid
    // dt=0.1, diff=0.0001, visc=0.0001
    let mut fluid = FluidSolver::new(FLUID_SIZE, 0.1, 0.00001, 0.00001);

    // Init Texture
    let mut image = Image::gen_image_color(FLUID_SIZE as u16, FLUID_SIZE as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Linear); // Bilinear filtering for smooth look

    loop {
        clear_background(BLACK);

        // --- Input ---
        let spectrum = audio.get_spectrum();

        // Map Spectrum to Fluid
        // Spectrum size ~ 512. Fluid width 128.
        // We can map 4 bins per cell, or just sample.

        for i in 0..FLUID_SIZE {
            // Map x (0..128) to freq bins (0..512)
            // Low freqs are more interesting usually, so linear mapping is fine
            let bin_idx = (i * spectrum.len()) / FLUID_SIZE;
            let val = spectrum[bin_idx.min(spectrum.len() - 1)];

            // Logarithmic scaling for audio? Or simple threshold?
            // "Ghost mode" produces vals around 0.0-3.0.

            if val > 0.1 {
                // Add density at bottom
                fluid.add_density(i, 1, val * 10.0);

                // Add upward velocity
                fluid.add_velocity(i, 1, 0.0, -val * 2.0);
            }
        }

        // Add random "spark" from high freqs at top?
        let high_energy: f32 = spectrum.iter().skip(300).sum();
        if high_energy > 10.0 {
            let x = macroquad::rand::gen_range(0, FLUID_SIZE);
            let y = macroquad::rand::gen_range(0, FLUID_SIZE / 2);
            fluid.add_density(x, y, 5.0);
            fluid.add_velocity(
                x,
                y,
                macroquad::rand::gen_range(-1.0, 1.0),
                macroquad::rand::gen_range(-1.0, 1.0),
            );
        }

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            let fx = (mx / sw * FLUID_SIZE as f32) as usize;
            let fy = (my / sh * FLUID_SIZE as f32) as usize;

            if fx < FLUID_SIZE && fy < FLUID_SIZE {
                fluid.add_density(fx, fy, 100.0);

                // Add velocity based on mouse delta?
                // For now just random burst
                fluid.add_velocity(
                    fx,
                    fy,
                    macroquad::rand::gen_range(-2.0, 2.0),
                    macroquad::rand::gen_range(-2.0, 2.0),
                );
            }
        }

        // --- Physics ---
        fluid.step();

        // --- Render ---
        // Update image pixels
        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                let idx = x + y * FLUID_SIZE; // Matches ix implementation
                let d = fluid.density[idx];

                // Color mapping: "Oil Painting"
                // d goes from 0 to ... potentially infinity but usually decays.
                // Let's clamp 0..255 effectively.

                let r = (d * 5.0).min(1.0);
                let g = (d * 2.0).min(1.0);
                let b = (d * 0.5 + 0.2 * (d * 10.0).sin()).clamp(0.0, 1.0); // Synesthetic wobble

                image.set_pixel(x as u32, y as u32, Color::new(r, g, b, 1.0));
            }
        }

        // Update texture
        texture.update(&image);

        // Draw texture
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

        // Draw HUD
        draw_text("Fluid Specter", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
