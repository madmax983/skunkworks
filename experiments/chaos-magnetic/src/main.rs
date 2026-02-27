//! # Chaos Magnetic
//!
//! **Lineage:** `chaos-pendulum` (Chaotic Physics) × `ferrous-core` (Magnetic Fields).
//!
//! **Concept:**
//! A chaotic double pendulum system acts as a "stylus" that writes to a magnetic platter.
//! The platter retains this magnetic history and exerts forces back on the pendulum,
//! creating a feedback loop between the chaotic motion and its own memory.
//!
//! The "Ghosts" (divergent simulations) also write to the platter, creating a probability cloud
//! of magnetic influence.

use ::rand::Rng;
use ferrous_core::Platter;
use macroquad::prelude::*;

mod physics;
use physics::PendulumSystem;

#[macroquad::main("Chaos Magnetic")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    // 1. Initialize Platter (The Magnetic Memory)
    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);

    // Initialize random noise in platter to start the chaos
    for y in 0..grid_h {
        for x in 0..grid_w {
            platter.magnetize(x, y, rng.gen_range(0.0..0.1));
        }
    }

    // 2. Initialize Pendulum System
    let mut real_system = PendulumSystem::new();
    // Root
    let root = real_system.add_node(
        vec2(0.0, 0.0),
        10.0,
        true,
        "Root".to_string(),
        0.5, // Neutral
    );
    // Arm 1
    let joint1 = real_system.add_node(
        vec2(0.0, 100.0),
        5.0,
        false,
        "Joint".to_string(),
        0.5, // Neutral
    );
    // Arm 2 (The Stylus)
    let tip = real_system.add_node(
        vec2(100.0, 100.0),
        2.0,
        false,
        "Stylus".to_string(),
        1.0, // South Pole (Writer)
    );

    real_system.add_link(root, joint1, 100.0);
    real_system.add_link(joint1, tip, 100.0);

    // Initial Kick
    real_system.nodes[joint1].prev_pos.x += 2.0;

    // 3. Initialize Ghosts
    let ghost_count = 50;
    let init_ghosts = |base_sys: &PendulumSystem, count: usize| -> Vec<PendulumSystem> {
        let mut rng = ::rand::thread_rng();
        (0..count)
            .map(|_| {
                let mut g = base_sys.clone();
                for node in &mut g.nodes {
                    if !node.fixed {
                        // Slight variations in position
                        let offset_x = rng.gen_range(-0.1..0.1);
                        let offset_y = rng.gen_range(-0.1..0.1);
                        node.pos.x += offset_x;
                        node.pos.y += offset_y;
                        node.prev_pos.x += offset_x;
                        node.prev_pos.y += offset_y;
                    }
                }
                g
            })
            .collect()
    };
    let mut ghosts = init_ghosts(&real_system, ghost_count);

    // View Parameters
    let mut zoom = 1.0;

    // Mapping: Physics World <-> Platter Grid
    // Physics bounds approx -300 to 300?
    // Grid 200x150.
    // Scale: 1 Grid Unit = 4 Physics Units?
    let grid_scale = 4.0;
    // Grid Center in World Space
    let grid_offset = vec2(
        -(grid_w as f32 * grid_scale) / 2.0,
        -(grid_h as f32 * grid_scale) / 2.0 + 150.0 // Shift down
    );

    let texture = Texture2D::from_image(&Image::gen_image_color(
        grid_w as u16,
        grid_h as u16,
        BLACK,
    ));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time().min(0.05);

        // --- Input ---
        if is_key_down(KeyCode::Up) { zoom *= 1.01; }
        if is_key_down(KeyCode::Down) { zoom *= 0.99; }
        if is_key_pressed(KeyCode::R) {
            // Reset
            platter = Platter::new(grid_w, grid_h);
             for y in 0..grid_h {
                for x in 0..grid_w {
                    platter.magnetize(x, y, ::rand::thread_rng().gen_range(0.0..0.1));
                }
            }
        }

        // --- Physics Update ---

        // 1. Decay Platter
        platter.decay(0.999);

        // 2. Systems Step (Read/Write)
        // We need to iterate sequentially because they all write to the SAME platter.
        // Parallel writing to Platter would be racey without atomics or locking.
        // For 50 ghosts, sequential is fine.

        let all_systems = std::iter::once(&mut real_system).chain(ghosts.iter_mut());

        for sys in all_systems {
            // Step Physics (Reads from Platter for forces)
            sys.step(dt, &mut platter, grid_offset, grid_scale);

            // Write to Platter (Stylus Tip)
            let tip_node = &sys.nodes[2]; // Hardcoded tip index
            if !tip_node.fixed {
                // Map to Grid
                let grid_pos = (tip_node.pos - grid_offset) / grid_scale;
                let gx = grid_pos.x.round() as usize;
                let gy = grid_pos.y.round() as usize;

                // Magnetize
                // If tip is South (1.0), it adds to field.
                // sys.nodes[2].magnetism is 1.0.
                let strength = 0.05;
                let val = (tip_node.magnetism - 0.5) * strength;
                platter.magnetize(gx, gy, val as f64);
            }
        }

        // --- Render ---
        clear_background(BLACK);

        // 1. Draw Platter (Heatmap)
        // Update Texture
        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y); // 0.0 to 1.0
                // Color ramp: Blue (0.0) -> Black (0.5) -> Red (1.0)
                let c = if mag < 0.5 {
                    // 0.0 -> 0.5 maps to Blue 1.0 -> 0.0
                    let b = (0.5 - mag) * 2.0;
                    Color::new(0.0, 0.0, b as f32, 1.0)
                } else {
                    // 0.5 -> 1.0 maps to Red 0.0 -> 1.0
                    let r = (mag - 0.5) * 2.0;
                    Color::new(r as f32, 0.0, 0.0, 1.0)
                };
                image.set_pixel(x as u32, y as u32, c);
            }
        }
        texture.update(&image);

        set_camera(&Camera2D {
            target: vec2(0.0, 100.0), // Look at pendulum area
            zoom: vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

        // Draw Texture mapped to world
        draw_texture_ex(
            &texture,
            grid_offset.x,
            grid_offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    grid_w as f32 * grid_scale,
                    grid_h as f32 * grid_scale,
                )),
                ..Default::default()
            },
        );

        // Draw Grid Outline
        draw_rectangle_lines(
            grid_offset.x,
            grid_offset.y,
            grid_w as f32 * grid_scale,
            grid_h as f32 * grid_scale,
            2.0,
            DARKGRAY
        );

        // 2. Draw Ghosts
        for g in &ghosts {
            let p1 = g.nodes[0].pos;
            let p2 = g.nodes[1].pos;
            let p3 = g.nodes[2].pos;

            let alpha = 0.1;
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, Color::new(0.0, 1.0, 1.0, alpha));
            draw_line(p2.x, p2.y, p3.x, p3.y, 1.0, Color::new(0.0, 1.0, 1.0, alpha));
        }

        // 3. Draw Real System
        let p1 = real_system.nodes[0].pos;
        let p2 = real_system.nodes[1].pos;
        let p3 = real_system.nodes[2].pos;

        draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, WHITE);
        draw_line(p2.x, p2.y, p3.x, p3.y, 3.0, WHITE);

        draw_circle(p1.x, p1.y, 5.0, GRAY);
        draw_circle(p2.x, p2.y, 5.0, WHITE);
        draw_circle(p3.x, p3.y, 8.0, RED); // The Stylus

        set_default_camera();

        draw_text("Chaos Magnetic", 20.0, 30.0, 30.0, WHITE);
        draw_text("Pendulum writes to Magnetic Memory", 20.0, 50.0, 20.0, GRAY);
        draw_text("Ghosts create probability cloud", 20.0, 70.0, 20.0, SKYBLUE);

        next_frame().await
    }
}
