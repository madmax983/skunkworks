use etymological_bridge::model::{State, Terrain, World};
use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: f32 = 12.0;

#[macroquad::main("Etymological Bridge")]
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

        if is_key_pressed(KeyCode::E) {
            world.evolve_all(0.3); // 30% chance to evolve per press
        }

        world.update();

        clear_background(LIGHTGRAY);

        // Draw Terrain
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let t = world.get_terrain(x as i32, y as i32);
                let color = match t {
                    Terrain::Solid => GRAY,
                    Terrain::Gap => BLACK,
                    Terrain::Bridge => DARKBLUE,
                };

                // Draw Pheromones overlay
                let p_idx = y * GRID_WIDTH + x;
                let p = world.pheromones[p_idx];

                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );

                if p > 0.05 {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        Color::new(0.0, 1.0, 0.0, p * 0.5),
                    );
                }
            }
        }

        let (mx, my) = mouse_position();

        // Draw Ants
        for ant in &world.ants {
            // Color based on state usually, but let's mix in word hash for 'dialect'
            let s = ant.word.to_string();
            let hash = s.chars().fold(0u32, |acc, c| acc.wrapping_add(c as u32));

            // Base color from state
             let mut color = match ant.state {
                State::Foraging => RED,
                State::Bridging => SKYBLUE,
                State::Returning => GREEN,
            };

            // Tint with dialect color
            let dr = (hash & 0xFF) as f32 / 255.0;
            let dg = ((hash >> 8) & 0xFF) as f32 / 255.0;
            let db = ((hash >> 16) & 0xFF) as f32 / 255.0;

            color.r = (color.r + dr) * 0.5;
            color.g = (color.g + dg) * 0.5;
            color.b = (color.b + db) * 0.5;

            let ax = ant.x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let ay = ant.y as f32 * CELL_SIZE + CELL_SIZE / 2.0;

            draw_circle(
                ax,
                ay,
                CELL_SIZE * 0.4,
                color,
            );

            // Draw text if mouse is close
            if (mx - ax).abs() < 50.0 && (my - ay).abs() < 50.0 {
                draw_text(&s, ax - 10.0, ay - 10.0, 14.0, WHITE);
            }
        }

        draw_text(
            "Left Click: Dig Gap | Right Click: Fill | Space: Spawn Ants | R: Reset",
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            "E: Trigger Sound Change (Grimm's Law)",
            10.0,
            40.0,
            20.0,
            YELLOW,
        );

        next_frame().await
    }
}
