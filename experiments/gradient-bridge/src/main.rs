mod landscape;
mod model;

use macroquad::prelude::*;
use landscape::*;
use model::{World, Terrain, State};
use ::rand::Rng;

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 80;
const CELL_SIZE: f32 = 10.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gradient Bridge".to_owned(),
        window_width: (GRID_WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * CELL_SIZE) as i32,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new(GRID_WIDTH, GRID_HEIGHT);
    let landscapes: Vec<Box<dyn ObjectiveFunction>> = vec![
        Box::new(GaussianHills),
        Box::new(Rastrigin),
        Box::new(Rosenbrock),
        Box::new(Ackley),
        Box::new(EggHolder),
    ];
    let mut current_idx = 0;

    // Initial generation
    world.generate_terrain(&*landscapes[current_idx]);

    // Spawn ants at random valid locations (Solid)
    spawn_ants(&mut world, 500);

    loop {
        // Input
        if is_key_pressed(KeyCode::Space) {
            current_idx = (current_idx + 1) % landscapes.len();
            world.generate_terrain(&*landscapes[current_idx]);
            spawn_ants(&mut world, 500);
        }
        if is_key_pressed(KeyCode::R) {
             world.generate_terrain(&*landscapes[current_idx]);
             spawn_ants(&mut world, 500);
        }
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = (mx / CELL_SIZE) as i32;
            let gy = (my / CELL_SIZE) as i32;
            for _ in 0..10 {
                world.add_ant(gx, gy);
            }
        }

        // Update
        world.update();

        // Draw
        clear_background(BLACK);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = y * GRID_WIDTH + x;
                let t = world.terrain[idx];
                let h = world.height_map[idx]; // Value roughly -1 to 1? Or much larger depending on func.

                let color = match t {
                    Terrain::Solid => {
                        // Color based on height
                        // Normalize h roughly for visualization.
                        // GaussianHills ~ -5 to 5.
                        // Rastrigin ~ -16 to 0.
                        // Let's use a dynamic range or fixed.
                        // Simple: Map height to brightness.
                        let val = (h + 5.0) / 10.0; // Shift and scale
                        let v = val.clamp(0.2, 1.0);
                        Color::new(v * 0.5, v, v * 0.5, 1.0) // Greenish
                    },
                    Terrain::Gap => {
                        // Water/Void
                        Color::new(0.0, 0.0, 0.2, 1.0)
                    },
                    Terrain::Bridge => {
                        // Bridge handled by ant drawing usually?
                        // But model sets terrain to Bridge.
                        // So we draw bridge structure here.
                        BROWN
                    }
                };

                // Draw Pheromones
                let p = world.pheromones[idx];
                let final_color = if p > 0.1 {
                     Color::new(color.r + p, color.g, color.b, 1.0)
                } else {
                    color
                };

                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    final_color,
                );
            }
        }

        // Draw Ants
        for ant in &world.ants {
             let color = match ant.state {
                State::Foraging => RED,
                State::Bridging => YELLOW, // Bridge ants are yellow
                State::Returning => MAGENTA,
            };

            // If bridging, we might have already drawn the cell as Bridge terrain (BROWN).
            // But let's draw the ant too.
            draw_circle(
                ant.x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                ant.y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                CELL_SIZE * 0.3,
                color,
            );
        }

        // UI
        draw_text(&format!("Landscape: {}", landscapes[current_idx].name()), 10.0, 20.0, 30.0, WHITE);
        draw_text("Space: Switch | R: Reset | Click: Spawn", 10.0, 50.0, 20.0, WHITE);
        draw_text(&format!("Ants: {}", world.ants.len()), 10.0, 70.0, 20.0, WHITE);

        next_frame().await
    }
}

fn spawn_ants(world: &mut World, count: usize) {
    let mut spawned = 0;
    let mut rng = ::rand::thread_rng();
    while spawned < count {
        let x = rng.gen_range(0..GRID_WIDTH) as i32;
        let y = rng.gen_range(0..GRID_HEIGHT) as i32;
        if world.get_terrain(x, y) == Terrain::Solid {
            world.add_ant(x, y);
            spawned += 1;
        }
    }
}
