mod lbm;
mod particles;
mod text_render;

use lbm::{FluidSim, HEIGHT, WIDTH};
use macroquad::prelude::*;
use particles::ParticleSystem;
use text_render::TextManager;

#[macroquad::main("Typographic Turbulence")]
async fn main() {
    // 1. Initialize Simulation
    let mut sim = FluidSim::new();
    let mut particle_system = ParticleSystem::new(20000); // 20k particles
    let mut text_manager = TextManager::new();

    // 2. Setup Visualization
    let mut fluid_image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let fluid_texture = Texture2D::from_image(&fluid_image);
    fluid_texture.set_filter(FilterMode::Nearest);

    // UI State
    let mut show_fluid = true;
    let mut show_particles = true;

    // Auto-spawn timer
    let mut last_spawn_time = 0.0;

    loop {
        let dt = get_frame_time();
        let time = get_time();

        let sw = screen_width();
        let sh = screen_height();
        let cell_w = sw / WIDTH as f32;
        let cell_h = sh / HEIGHT as f32;

        // 3. Input Handling
        // Mouse interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;

            sim.add_density(gx, gy, 5.0);

            let delta = mouse_delta_position();
            // Scale delta to be meaningful impulse
            sim.add_velocity(gx, gy, delta.x * 10.0, delta.y * 10.0);
        }

        // Typing to spawn text at mouse cursor
        while let Some(c) = get_char_pressed() {
            if !c.is_control() {
                let (mx, my) = mouse_position();
                let gx = mx / cell_w;
                let gy = my / cell_h;

                // Random velocity
                let vx = rand::gen_range(-10.0, 10.0);
                let vy = rand::gen_range(-5.0, 5.0);

                text_manager.spawn(&c.to_string(), gx, gy, vx, vy);
            }
        }

        // Background scrolling text
        if time - last_spawn_time > 3.0 {
            let words = ["FLOW", "FLUID", "CHAOS", "VORTEX", "TURBULENCE", "RUST", "MACROQUAD", "SIMULATION", "GENESIS", "MOONSHOT"];
            let word = words[rand::gen_range(0, words.len())];
            let y = rand::gen_range(10.0, HEIGHT as f32 - 10.0);
            // Move left
            text_manager.spawn(word, WIDTH as f32, y, -10.0, 0.0);
            last_spawn_time = time;
        }

        // Toggle views
        if is_key_pressed(KeyCode::F) {
            show_fluid = !show_fluid;
        }
        if is_key_pressed(KeyCode::P) {
            show_particles = !show_particles;
        }

        // 4. Updates
        // First update text (which sets obstacles and imparts velocity)
        text_manager.update(&mut sim, dt);

        // Then step fluid
        sim.step();

        // Then move particles
        particle_system.update(&sim, dt);

        // 5. Rendering
        clear_background(BLACK);

        if show_fluid {
            // Update fluid texture
            // Map density/curl to colors
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let idx = y * WIDTH + x;

                    if sim.obstacles[idx] {
                        fluid_image.set_pixel(x as u32, y as u32, WHITE);
                    } else {
                        let rho = sim.density[idx];
                        let curl = sim.curl[idx];

                        // Visualize Curl
                        let c = (curl * 10.0).tanh(); // Amplify curl
                        let r = if c > 0.0 { c } else { 0.0 };
                        let b = if c < 0.0 { -c } else { 0.0 };

                        // Add some density visualization
                        let g = (rho - 1.0).abs() * 2.0;

                        fluid_image.set_pixel(x as u32, y as u32, Color::new(r, g, b, 1.0));
                    }
                }
            }
            fluid_texture.update(&fluid_image);
            draw_texture_ex(
                &fluid_texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(sw, sh)),
                    ..Default::default()
                },
            );
        }

        if show_particles {
             for p in particle_system.particles() {
                 let x = p.position.x * cell_w;
                 let y = p.position.y * cell_h;
                 // Draw char
                 draw_text(&p.char.to_string(), x, y, cell_h * 1.5, p.color);
             }
        }

        // Draw UI
        draw_text("Typographic Turbulence", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Particles: {}", particle_system.count()), 10.0, 60.0, 20.0, WHITE);
        draw_text("Type to spawn text. Mouse to stir.", 10.0, 80.0, 20.0, GRAY);
        draw_text("F: Toggle Fluid | P: Toggle Particles", 10.0, 100.0, 20.0, GRAY);

        next_frame().await
    }
}
