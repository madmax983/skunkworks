use macroquad::prelude::*;

mod botany;
mod grid;

use botany::Plant;
use grid::{CellType, Grid};

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;

#[macroquad::main("Chimera Roots")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    // Set default start and goal
    let start = (GRID_WIDTH / 2, 0); // Start at top center
    let goal = (GRID_WIDTH / 2, GRID_HEIGHT - 1); // Goal at bottom center

    grid.set_type(start.0, start.1, CellType::Seed);
    grid.set_type(goal.0, goal.1, CellType::Water);

    // Initialize plant with population of 50 roots
    let mut plant = Plant::new(start, 50);

    let mut running = false;
    let mut auto_evolve = false;
    let mut speed = 1;

    loop {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let cell_size =
            (screen_width.min(screen_height) / GRID_WIDTH.max(GRID_HEIGHT) as f32 * 0.9).floor();
        let offset_x = (screen_width - cell_size * GRID_WIDTH as f32) / 2.0;
        let offset_y = (screen_height - cell_size * GRID_HEIGHT as f32) / 2.0;

        // Input
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = ((mx - offset_x) / cell_size) as i32;
            let gy = ((my - offset_y) / cell_size) as i32;

            if gx >= 0 && gx < GRID_WIDTH as i32 && gy >= 0 && gy < GRID_HEIGHT as i32 {
                grid.set_type(gx as usize, gy as usize, CellType::Rock);
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let gx = ((mx - offset_x) / cell_size) as i32;
            let gy = ((my - offset_y) / cell_size) as i32;

            if gx >= 0 && gx < GRID_WIDTH as i32 && gy >= 0 && gy < GRID_HEIGHT as i32 {
                grid.set_type(gx as usize, gy as usize, CellType::Water);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }

        if is_key_pressed(KeyCode::E) {
            auto_evolve = !auto_evolve;
        }

        if is_key_pressed(KeyCode::N) {
            plant.next_generation(&grid);
        }

        if is_key_pressed(KeyCode::Up) {
            speed += 1;
        }
        if is_key_pressed(KeyCode::Down) {
            if speed > 1 {
                speed -= 1;
            }
        }

        // Logic
        if running {
            for _ in 0..speed {
                plant.update(&mut grid);
            }
        }

        if auto_evolve {
            // Check if all tips are finished
            if plant.tips.iter().all(|t| t.finished) {
                plant.next_generation(&grid);
            }
        }

        // Drawing
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        grid.draw(cell_size, offset_x, offset_y);
        plant.draw(cell_size, offset_x, offset_y);

        // UI
        draw_text(
            &format!(
                "Generation: {} | Best Fitness: {:.2}",
                plant.generation, plant.best_fitness
            ),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!(
                "Speed: {} | Auto-Evolve: {}",
                speed,
                if auto_evolve { "ON" } else { "OFF" }
            ),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Space: Pause | N: Next Gen | E: Auto-Evolve | Click: Wall/Water",
            10.0,
            screen_height - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
