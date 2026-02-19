use macroquad::prelude::*;

mod grid;
mod root;
mod scheduler;

use grid::{Grid, Cell};
use scheduler::Scheduler;

#[macroquad::main("Silicon Canopy")]
async fn main() {
    let mut grid = Grid::new(50, 50);
    let mut scheduler = Scheduler::new();

    // Spawn processes (Trees)
    // ID 0: Red
    scheduler.spawn_process(&mut grid, 0, 10, 0, RED);
    // ID 1: Blue
    scheduler.spawn_process(&mut grid, 1, 40, 0, BLUE);
    // ID 2: Green
    scheduler.spawn_process(&mut grid, 2, 25, 0, GREEN);
    // ID 3: Yellow
    scheduler.spawn_process(&mut grid, 3, 10, 49, YELLOW);
    // ID 4: Purple
    scheduler.spawn_process(&mut grid, 4, 40, 49, PURPLE);

    // Let's fix aspect ratio or just use min dimension.

    loop {
        let cell_w = screen_width() / grid.width as f32;
        let cell_h = screen_height() / grid.height as f32;

        // Update
        // Run multiple ticks per frame for speed
        for _ in 0..5 {
            scheduler.tick(&mut grid);
        }

        clear_background(BLACK);

        // Draw Grid
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Some(cell) = grid.get(x, y) {
                    let color = match cell {
                        Cell::Empty => Color::new(0.2, 0.1, 0.05, 1.0), // Dirt
                        Cell::Memory => Color::new(0.0, 0.8, 0.0, 0.5), // RAM (Green)
                        Cell::IO => Color::new(0.0, 0.5, 1.0, 0.5), // IO (Blue)
                        Cell::Corrupt => Color::new(0.5, 0.0, 0.0, 0.5), // Bad Sector
                        Cell::Root(id) => {
                            // Find process color
                            if let Some(p) = scheduler.processes.iter().find(|p| p.id == id) {
                                p.color
                            } else {
                                WHITE
                            }
                        }
                    };

                    draw_rectangle(
                        x as f32 * cell_w,
                        y as f32 * cell_h,
                        cell_w,
                        cell_h,
                        color,
                    );
                }
            }
        }

        // Draw HUD
        draw_text("Silicon Canopy: Resource Competition", 10.0, 20.0, 30.0, WHITE);

        let mut y_off = 50.0;
        for p in &scheduler.processes {
            draw_text(
                &format!("PID {}: Res {}, IO {}", p.id, p.resources, p.io_ops),
                10.0,
                y_off,
                20.0,
                p.color,
            );
            y_off += 20.0;
        }

        next_frame().await
    }
}
