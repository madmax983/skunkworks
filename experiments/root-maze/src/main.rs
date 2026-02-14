pub mod maze;
pub mod root;

use macroquad::prelude::*;
use maze::{Grid, Soil};
use root::RootSystem;

fn window_conf() -> Conf {
    Conf {
        window_title: "Genesis: Root Maze".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = 80;
    let height = 60;

    let mut grid = Grid::new(width, height);
    let mut root_system = RootSystem::new(vec2(grid.start.0 as f32, grid.start.1 as f32));

    let mut paused = false;
    let mut show_grid = true;

    loop {
        // --- Input ---
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            grid = Grid::new(width, height);
            root_system = RootSystem::new(vec2(grid.start.0 as f32, grid.start.1 as f32));
        }
        if is_key_pressed(KeyCode::G) {
            show_grid = !show_grid;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let cell_w = screen_width() / width as f32;
            let cell_h = screen_height() / height as f32;
            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;
            if gx < width && gy < height {
                root_system = RootSystem::new(vec2(gx as f32, gy as f32));
            }
        }

        // --- Update ---
        if !paused {
            root_system.grow(&grid);
        }

        // --- Draw ---
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let cell_w = screen_width() / width as f32;
        let cell_h = screen_height() / height as f32;

        // Draw Grid
        if show_grid {
            for x in 0..width {
                for y in 0..height {
                    let soil = grid.get(x, y);
                    // Skip empty/hardrock if we want faster drawing? No, draw all for now.
                    let color = match soil {
                        Soil::HardRock => Color::new(0.2, 0.2, 0.2, 1.0),
                        Soil::SoftSoil => Color::new(0.3, 0.2, 0.1, 1.0),
                        Soil::Empty => Color::new(0.05, 0.05, 0.05, 1.0),
                        Soil::Water => Color::new(0.0, 0.4, 0.8, 1.0),
                    };

                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, color);
                }
            }
        }

        // Draw Roots
        let offset = vec2(cell_w * 0.5, cell_h * 0.5);
        for segment in &root_system.segments {
            if let Some(parent_idx) = segment.parent {
                let parent = &root_system.segments[parent_idx];

                let start = parent.pos * vec2(cell_w, cell_h) + offset;
                let end = segment.pos * vec2(cell_w, cell_h) + offset;

                let thickness = segment.thickness * (cell_w / 10.0).max(0.5);
                let color = Color::new(0.9, 0.8, 0.6, 1.0); // Root color

                draw_line(start.x, start.y, end.x, end.y, thickness, color);
            }
        }

        // Draw Tips
        for tip in &root_system.tips {
            let pos = tip.pos * vec2(cell_w, cell_h) + offset;
            draw_circle(pos.x, pos.y, 2.0, GREEN);
        }

        // Draw Start/Goal Indicators if grid hidden
        if !show_grid {
            let start_pos =
                vec2(grid.start.0 as f32, grid.start.1 as f32) * vec2(cell_w, cell_h) + offset;
            draw_circle(start_pos.x, start_pos.y, 5.0, GREEN);
            let goal_pos =
                vec2(grid.goal.0 as f32, grid.goal.1 as f32) * vec2(cell_w, cell_h) + offset;
            draw_circle(goal_pos.x, goal_pos.y, 5.0, BLUE);
        }

        // UI
        draw_text("Genesis: Root Maze", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "Space: Pause | R: Reset | G: Toggle Soil | Click: Re-seed",
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Tips: {}", root_system.tips.len()),
            20.0,
            screen_height() - 20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
