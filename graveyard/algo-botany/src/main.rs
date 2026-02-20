use macroquad::prelude::*;

mod botany;
mod grid;

use botany::Plant;
use grid::{CellType, Grid};

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;

#[macroquad::main("Algo-Botany")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut plant = Plant::new((0, 0));
    let mut running = false;

    // Set default start and goal
    grid.set_type(0, 0, CellType::Seed);
    grid.set_type(GRID_WIDTH - 1, GRID_HEIGHT - 1, CellType::Water);

    // Re-init plant with correct start
    if let Some(start) = grid.start {
        plant = Plant::new(start);
    }

    loop {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let cell_size =
            (screen_width.min(screen_height) / GRID_WIDTH.max(GRID_HEIGHT) as f32 * 0.9).floor();
        let offset_x = (screen_width - cell_size * GRID_WIDTH as f32) / 2.0;
        let offset_y = (screen_height - cell_size * GRID_HEIGHT as f32) / 2.0;

        // Input
        // Draw Walls
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = ((mx - offset_x) / cell_size) as i32;
            let gy = ((my - offset_y) / cell_size) as i32;

            if gx >= 0 && gx < GRID_WIDTH as i32 && gy >= 0 && gy < GRID_HEIGHT as i32 {
                grid.set_type(gx as usize, gy as usize, CellType::Rock);
            }
        }

        // Place Goal
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let gx = ((mx - offset_x) / cell_size) as i32;
            let gy = ((my - offset_y) / cell_size) as i32;

            if gx >= 0 && gx < GRID_WIDTH as i32 && gy >= 0 && gy < GRID_HEIGHT as i32 {
                grid.set_type(gx as usize, gy as usize, CellType::Water);
            }
        }

        // Place Seed (Start)
        if is_key_down(KeyCode::S) && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let gx = ((mx - offset_x) / cell_size) as i32;
            let gy = ((my - offset_y) / cell_size) as i32;

            if gx >= 0 && gx < GRID_WIDTH as i32 && gy >= 0 && gy < GRID_HEIGHT as i32 {
                grid.set_type(gx as usize, gy as usize, CellType::Seed);
                // Reset plant
                plant = Plant::new((gx as usize, gy as usize));
            }
        }

        if is_key_pressed(KeyCode::R) {
            if let Some(start) = grid.start {
                plant = Plant::new(start);
            } else {
                plant = Plant::new((0, 0));
            }
        }

        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }

        // Logic
        if running {
            // Speed up: do multiple updates per frame if needed, or adjust growth_speed in Plant
            plant.growth_speed = 5;
            plant.update(&grid);
        }

        // Drawing
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // Use manual offsets instead of push_matrix/translate which are not in prelude
        grid.draw(cell_size, offset_x, offset_y);
        plant.draw(cell_size, offset_x, offset_y);

        draw_text("Left Click: Wall | Right Click: Goal | S+Click: Seed | R: Reset Plant | Space: Pause/Resume", 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
