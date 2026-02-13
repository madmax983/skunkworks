use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};
use resonance_audio::physics::Material;

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 80;
const SIM_STEPS_PER_FRAME: usize = 12; // 60fps * 12 = 720Hz simulation rate (approx)

#[macroquad::main("Echo Chamber")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, _snap_rx) = bounded(1); // Latest snapshot only

    // Create the physics model
    // Note: In a real audio app, this would live on a separate thread.
    // Here, we run it in the main loop for simplicity as we lack cpal support.
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx);

    // Visualization buffer
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let draw_radius = 1;
    let brush_mode = Material::Wall;

    loop {
        // --- Input Handling ---
        let (mx, my) = mouse_position();
        let cell_w = screen_width() / GRID_WIDTH as f32;
        let cell_h = screen_height() / GRID_HEIGHT as f32;

        let grid_x = (mx / cell_w) as usize;
        let grid_y = (my / cell_h) as usize;
        let in_bounds = grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT;

        if is_mouse_button_down(MouseButton::Left) && in_bounds {
            // Paint walls
            for dy in -(draw_radius as isize)..=draw_radius as isize {
                for dx in -(draw_radius as isize)..=draw_radius as isize {
                    let gx = (grid_x as isize + dx) as usize;
                    let gy = (grid_y as isize + dy) as usize;
                    if gx < GRID_WIDTH && gy < GRID_HEIGHT {
                        cmd_tx.send(AudioCommand::PaintMaterial { x: gx, y: gy, material: brush_mode }).unwrap();
                    }
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) && in_bounds {
            // Pluck
             cmd_tx.send(AudioCommand::Pluck { x: grid_x, y: grid_y, strength: 1.0 }).unwrap();
        }

        if is_key_pressed(KeyCode::C) {
             cmd_tx.send(AudioCommand::ClearWaves).unwrap();
        }

        if is_key_pressed(KeyCode::X) {
             cmd_tx.send(AudioCommand::ClearWalls).unwrap();
        }

        if is_key_pressed(KeyCode::Space) && in_bounds {
             cmd_tx.send(AudioCommand::Oscillate { x: grid_x, y: grid_y, frequency: 10.0, strength: 0.5 }).unwrap();
        }

        // --- Simulation ---
        // Run physics steps
        let mut dummy_buffer = vec![0.0; SIM_STEPS_PER_FRAME];
        model.process(&mut dummy_buffer);

        // --- Visualization ---
        // Access model directly

        // Update texture
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = y * GRID_WIDTH + x;
                // Use accessors
                let u = model.grid.u()[idx];
                let mat = model.grid.materials()[idx];

                let color = match mat {
                    Material::Wall => WHITE,
                    Material::Void => BLACK, // Absorbing boundary
                    _ => {
                        // Pressure visualization
                        // Blue = Negative, Black = Zero, Red = Positive
                        if u > 0.0 {
                            Color::new(u.min(1.0), 0.0, 0.0, 1.0)
                        } else {
                            // Negative
                            Color::new(0.0, 0.0, (-u).min(1.0), 1.0)
                        }
                    }
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        // Draw the simulation texture stretched to screen
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

        // UI
        draw_text("Echo Chamber", 10.0, 20.0, 30.0, WHITE);
        draw_text("Left: Wall | Right: Pluck | Space: Oscillator", 10.0, 40.0, 20.0, GRAY);
        draw_text("C: Clear Waves | X: Clear Walls", 10.0, 60.0, 20.0, GRAY);

        next_frame().await
    }
}
