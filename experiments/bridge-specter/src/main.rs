use macroquad::prelude::*;
use log::info;

mod fluid;
mod audio;
mod ants;

use fluid::FluidSolver;
use audio::AudioSystem;
use ants::{AntColony, Terrain, State};

const FLUID_SIZE: usize = 128; // Simulation grid size
const SCALE: f32 = 4.0; // Visual scale

#[macroquad::main("Bridge Specter")]
async fn main() {
    env_logger::init();
    info!("Starting Bridge Specter");

    let mut fluid = FluidSolver::new(FLUID_SIZE, 0.1, 0.0001, 0.0001);
    let mut audio = AudioSystem::new().expect("Failed to init audio");
    let mut colony = AntColony::new(FLUID_SIZE, FLUID_SIZE);

    // Initial fluid setup (e.g. a river in the middle)
    for y in 0..FLUID_SIZE {
        for x in 40..80 {
            fluid.add_density(x, y, 1.0);
        }
    }

    // Image for rendering fluid
    let mut image = Image::gen_image_color(FLUID_SIZE as u16, FLUID_SIZE as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        let _dt = get_frame_time();

        // 1. Audio Input
        let spectrum = audio.get_spectrum();
        let bass_energy: f32 = spectrum.iter().take(10).sum::<f32>() / 10.0;
        let _mid_energy: f32 = spectrum.iter().skip(10).take(50).sum::<f32>() / 50.0;

        // 2. Fluid Update
        // Add velocity based on audio
        let t = get_time() as f32;
        let cx = FLUID_SIZE / 2;
        let cy = FLUID_SIZE / 2;

        // Bass creates waves from center or bottom
        if bass_energy > 5.0 {
            fluid.add_velocity(cx, cy, (t.cos()) * bass_energy, (t.sin()) * bass_energy);
            fluid.add_density(cx, cy, bass_energy * 0.1);
        }

        // Mouse interaction
        let (mx, my) = mouse_position();
        let mx = (mx / SCALE) as usize;
        let my = (my / SCALE) as usize;

        if is_mouse_button_down(MouseButton::Left) {
             if mx < FLUID_SIZE && my < FLUID_SIZE {
                 fluid.add_density(mx, my, 5.0);
                 fluid.add_velocity(mx, my, 1.0, 1.0);
             }
        }

        // Right click to spawn ants
        if is_mouse_button_down(MouseButton::Right) {
             if mx < FLUID_SIZE && my < FLUID_SIZE {
                 colony.add_ant(mx as i32, my as i32);
             }
        }

        fluid.step();

        // 3. Ant Colony Update
        // Sync terrain (Fluid > 0.5 is Gap)
        colony.sync_with_fluid(&fluid.density, 0.5);
        colony.update(bass_energy);

        // 4. Render
        clear_background(BLACK);

        // Update fluid texture
        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                let d = fluid.density[y * FLUID_SIZE + x];
                let color = if d > 0.05 {
                    Color::new(0.2, 0.2, 1.0, d.min(1.0))
                } else {
                    BLACK
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(FLUID_SIZE as f32 * SCALE, FLUID_SIZE as f32 * SCALE)),
            ..Default::default()
        });

        // Render Ants
        for ant in &colony.ants {
            let color = match ant.state {
                State::Foraging => GREEN,
                State::Bridging => RED, // Bridges are strong red
                State::Returning => YELLOW,
                State::Panicking => PURPLE,
            };

            draw_rectangle(
                ant.x as f32 * SCALE,
                ant.y as f32 * SCALE,
                SCALE,
                SCALE,
                color
            );
        }

        // Render Bridges specifically (in case logic differs)
        for y in 0..FLUID_SIZE {
            for x in 0..FLUID_SIZE {
                if colony.terrain[y * FLUID_SIZE + x] == Terrain::Bridge {
                    draw_rectangle(
                        x as f32 * SCALE,
                        y as f32 * SCALE,
                        SCALE,
                        SCALE,
                        Color::new(0.8, 0.4, 0.0, 0.5) // Brownish overlay for bridge
                    );
                }
            }
        }

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Ants: {}", colony.ants.len()), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Bass: {:.2}", bass_energy), 10.0, 60.0, 20.0, WHITE);

        // Visualizer bar
        let spectrum_h = 100.0;
        let bar_w = screen_width() / spectrum.len() as f32;
        for (i, val) in spectrum.iter().enumerate() {
            let h = (val * 5.0).min(spectrum_h);
            draw_rectangle(i as f32 * bar_w, screen_height() - h, bar_w, h, Color::new(1.0, 1.0, 1.0, 0.5));
        }

        next_frame().await;
    }
}
