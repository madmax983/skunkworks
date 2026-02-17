mod phonology;
mod simulation;

use macroquad::prelude::*;
use simulation::{World, WORLD_WIDTH, WORLD_HEIGHT, BASELINE_Y, TERRAIN_SCALE_Y};

#[macroquad::main("Locust Linguistics")]
async fn main() {
    // Initial text
    let initial_text = "The swarm consumes the silence.";
    let mut world = World::new(initial_text);

    // Camera / Viewport
    // We render to a fixed resolution texture to match simulation coordinates
    // then draw that texture scaled to the screen.
    let target_width = WORLD_WIDTH as u16;
    let target_height = WORLD_HEIGHT as u16;
    let render_target = render_target(target_width as u32, target_height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    loop {
        // --- Input ---
        if is_key_pressed(KeyCode::R) {
            world = World::new(&world.original_text);
        }

        // Allow typing to change text?
        let char_pressed = get_char_pressed();
        if let Some(c) = char_pressed {
            if c.is_alphabetic() || c == ' ' {
                let mut new_text = world.original_text.clone();
                new_text.push(c);
                world = World::new(&new_text);
            }
        }
        if is_key_pressed(KeyCode::Back) {
            let mut new_text = world.original_text.clone();
            if !new_text.is_empty() {
                new_text.pop();
                world = World::new(&new_text);
            }
        }

        // --- Update ---
        world.update();

        // --- Render to Texture ---
        set_camera(&Camera2D {
            zoom: vec2(2.0 / WORLD_WIDTH, 2.0 / WORLD_HEIGHT),
            target: vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0),
            render_target: Some(render_target.clone()),
            ..Default::default()
        });

        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Dark Sky

        // Draw Terrain
        // Map terrain points to screen lines
        let terrain_len = world.terrain.len();
        for i in 0..terrain_len.saturating_sub(1) {
            // Map index to X
            let x1 = (i as f32 / terrain_len as f32) * WORLD_WIDTH;
            let y1 = BASELINE_Y - world.terrain[i].height * TERRAIN_SCALE_Y;

            let x2 = ((i + 1) as f32 / terrain_len as f32) * WORLD_WIDTH;
            let y2 = BASELINE_Y - world.terrain[i+1].height * TERRAIN_SCALE_Y;

            // Height color
            let h = world.terrain[i].height;
            let color = Color::new(h, 1.0 - h, 0.5, 1.0);

            draw_line(x1, y1, x2, y2, 2.0, color);

            // Fill below
            draw_triangle(
                vec2(x1, y1),
                vec2(x2, y2),
                vec2(x2, WORLD_HEIGHT),
                Color::new(h*0.5, (1.0-h)*0.5, 0.25, 0.5),
            );
            draw_triangle(
                vec2(x1, y1),
                vec2(x2, WORLD_HEIGHT),
                vec2(x1, WORLD_HEIGHT),
                Color::new(h*0.5, (1.0-h)*0.5, 0.25, 0.5),
            );
        }

        // Draw Agents
        for agent in &world.agents {
            let color = if agent.state == 1 {
                RED // Eating
            } else {
                Color::new(0.0, 1.0, 1.0, 0.6) // Flying (Cyan)
            };
            draw_rectangle(agent.pos.x, agent.pos.y, 2.0, 2.0, color);
        }

        // --- Render to Screen ---
        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true, // Render targets are often flipped
                ..Default::default()
            },
        );

        // UI Overlay
        draw_text("LOCUST LINGUISTICS", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Input: {}", world.original_text), 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Current: {}", world.current_text), 20.0, 90.0, 20.0, GOLD);
        draw_text("Type to seed | R: Reset", 20.0, screen_height() - 30.0, 20.0, GRAY);
        draw_text(&format!("Agents: {}", world.agents.len()), screen_width() - 150.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
