use mnem_bridge::model::{State, Terrain, World};
use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: f32 = 12.0;

#[macroquad::main("Mnemonic Bridge")]
async fn main() {
    let mut world = World::new(GRID_WIDTH, GRID_HEIGHT);

    // Initialize with a central gap
    for y in 0..GRID_HEIGHT {
        // Create a vertical canyon
        if y > 10 && y < 50 {
            for x in 45..55 {
                world.set_terrain(x, y, Terrain::Gap);
            }
        }
    }

    // Spawn ants on the left
    for _ in 0..500 {
        world.add_ant(20, 30);
    }

    loop {
        if is_key_down(KeyCode::R) {
            world = World::new(GRID_WIDTH, GRID_HEIGHT);
            for y in 0..GRID_HEIGHT {
                if y > 10 && y < 50 {
                    for x in 45..55 {
                        world.set_terrain(x, y, Terrain::Gap);
                    }
                }
            }
            for _ in 0..500 {
                world.add_ant(20, 30);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / CELL_SIZE) as usize;
            let gy = (my / CELL_SIZE) as usize;
            world.set_terrain(gx, gy, Terrain::Gap);
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let gx = (mx / CELL_SIZE) as usize;
            let gy = (my / CELL_SIZE) as usize;
            world.set_terrain(gx, gy, Terrain::Solid);
        }

        if is_key_pressed(KeyCode::Space) {
            // Spawn more ants at mouse
            let (mx, my) = mouse_position();
            let gx = (mx / CELL_SIZE) as i32;
            let gy = (my / CELL_SIZE) as i32;
            for _ in 0..50 {
                world.add_ant(gx, gy);
            }
        }

        world.update();

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // Draw Terrain
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let t = world.get_terrain(x as i32, y as i32);
                let color = match t {
                    Terrain::Solid => GRAY,
                    Terrain::Gap => BLACK,
                    Terrain::Bridge => Color::new(0.0, 0.0, 0.3, 1.0),
                };

                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );

                // Draw Pheromones overlay (faint green)
                let p_idx = y * GRID_WIDTH + x;
                let p = world.pheromones[p_idx];
                if p > 0.1 {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        Color::new(0.0, 1.0, 0.0, p * 0.3),
                    );
                }
            }
        }

        // Draw Ants
        let mut hovered_payload = None;
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);

        for ant in &world.ants {
            let pos = vec2(
                ant.x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                ant.y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            );

            let color = match ant.state {
                State::Foraging => RED,
                State::Bridging => {
                    // Health gradient: Green (1.0) -> Yellow (0.5) -> Red (0.0)
                    let h = ant.health.clamp(0.0, 1.0);
                    if h > 0.5 {
                        Color::new((1.0 - h) * 2.0, 1.0, 0.0, 1.0)
                    } else {
                        Color::new(1.0, h * 2.0, 0.0, 1.0)
                    }
                },
                State::Returning => GREEN,
            };

            draw_circle(pos.x, pos.y, CELL_SIZE * 0.4, color);

            // Hover check
            if (pos - mouse_pos).length() < CELL_SIZE {
                hovered_payload = Some(ant.payload.clone());
            }
        }

        // UI
        draw_text(
            "Left Click: Dig Gap | Right Click: Fill | Space: Spawn Ants | R: Reset",
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        if let Some(payload) = hovered_payload {
            // Draw background for text
            let text_w = measure_text(&payload, None, 20, 1.0).width;
            draw_rectangle(mouse_pos.x + 15.0, mouse_pos.y - 15.0, text_w + 10.0, 30.0, BLACK);
            draw_text(&payload, mouse_pos.x + 20.0, mouse_pos.y + 5.0, 20.0, WHITE);
        }

        next_frame().await
    }
}
