mod grid;
mod mechanism;

use grid::Grid;
use mechanism::{Differential, Integrator};
use macroquad::prelude::*;
use ::rand::Rng;

const GRID_WIDTH: usize = 256;
const GRID_HEIGHT: usize = 256;

#[macroquad::main("Sandpile Market")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut diff = Differential::new();
    let mut sma = Integrator::new();

    // Texture for grid
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut price = 100.0;
    let mut history: Vec<(f32, f32)> = Vec::new(); // (time, price)
    let mut t = 0.0;

    loop {
        let dt = get_frame_time();

        // Input / Drop Sand
        let mut rng = ::rand::thread_rng();
        // Random drops (Order Flow)
        // Adjust probability for decent activity
        if rng.gen_bool(0.1) {
            // Bid (Left)
            let rx = rng.gen_range(0..GRID_WIDTH/2);
            let ry = rng.gen_range(0..GRID_HEIGHT);
            grid.add_load(rx, ry, 1);
        }
        if rng.gen_bool(0.1) {
            // Ask (Right)
            let rx = rng.gen_range(GRID_WIDTH/2..GRID_WIDTH);
            let ry = rng.gen_range(0..GRID_HEIGHT);
            grid.add_load(rx, ry, 1);
        }

        // Manual Input
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Map to grid
            // Simple mapping if clicked on top part
            if my < screen_height() * 0.6 {
                let gx = (mx / screen_width() * GRID_WIDTH as f32) as usize;
                let gy = (my / (screen_height() * 0.6) * GRID_HEIGHT as f32) as usize;
                grid.add_load(gx, gy, 10);
            }
        }

        // Count Activity (Topples) BEFORE update (based on current state)
        let (bids, asks) = grid.count_activity();

        // Update Grid
        grid.update(0.01); // 1% processing rate (dissipation)

        // Update Mechanism
        // Bids push price UP, Asks push price DOWN
        // Scale factor to make movement visible but not crazy
        let sensitivity = 0.05;

        // Differential takes two inputs.
        let delta_price = diff.update(bids as f32 * sensitivity, -(asks as f32) * sensitivity);
        price += delta_price;
        if price < 0.0 { price = 0.0; }

        // Integrator (SMA / Deviation)
        // Input: Constant Time (rotation)
        // Carriage: Price deviation from 100
        sma.carriage_pos = (price - 100.0) / 10.0;
        let _ = sma.update(dt * 10.0); // Rotate based on time

        // History
        t += dt;
        history.push((t, price));
        if history.len() > 500 { history.remove(0); }

        // DRAW
        clear_background(DARKGRAY);

        // 1. Draw Grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = grid.get_index(x, y);
                let cell = grid.cells[idx];
                let color = if cell.load == 0 {
                    BLACK
                } else if cell.load < 4 {
                    // Stable sand
                    if x < GRID_WIDTH/2 {
                         Color::new(0.0, 0.3 * cell.load as f32, 0.0, 1.0) // Greenish
                    } else {
                         Color::new(0.3 * cell.load as f32, 0.0, 0.0, 1.0) // Reddish
                    }
                } else {
                    // Toppling! Bright!
                    WHITE
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height() * 0.6)),
            ..Default::default()
        });

        // 2. Draw Mechanism (Bottom 40%)
        let mech_y = screen_height() * 0.6;

        draw_rectangle(0.0, mech_y, screen_width(), screen_height() - mech_y, Color::new(0.1, 0.1, 0.1, 1.0));

        // Draw Gauges
        draw_text(&format!("Price: {:.2}", price), 10.0, mech_y + 30.0, 30.0, GOLD);
        draw_text(&format!("Bids: {}", bids), 10.0, mech_y + 60.0, 20.0, GREEN);
        draw_text(&format!("Asks: {}", asks), 10.0, mech_y + 80.0, 20.0, RED);
        draw_text("Differential Output", 10.0, mech_y + 110.0, 15.0, GRAY);

        // Plot
        if history.len() > 1 {
            let plot_w = screen_width() - 200.0;
            let plot_h = screen_height() - mech_y - 20.0;
            let plot_x = 180.0;
            let plot_y = mech_y + 10.0;

            draw_rectangle(plot_x, plot_y, plot_w, plot_h, BLACK);
            draw_rectangle_lines(plot_x, plot_y, plot_w, plot_h, 1.0, GRAY);

            let min_p = history.iter().map(|(_, p)| *p).fold(f32::INFINITY, |a, b| a.min(b));
            let max_p = history.iter().map(|(_, p)| *p).fold(f32::NEG_INFINITY, |a, b| a.max(b));
            let range = (max_p - min_p).max(10.0);

            let t_start = history[0].0;
            let t_end = history[history.len()-1].0;
            let t_range = (t_end - t_start).max(0.1);

            let mut prev_pos: Option<Vec2> = None;
            for (ht, hp) in &history {
                let px = plot_x + ((*ht - t_start) / t_range) * plot_w;
                // Invert Y for drawing (top is 0)
                // Normalize hp: (hp - min) / range -> 0..1
                // Screen Y: plot_y + plot_h - (norm * plot_h)
                let py = plot_y + plot_h - ((*hp - min_p) / range) * plot_h;
                let pos = vec2(px, py);

                if let Some(prev) = prev_pos {
                    draw_line(prev.x, prev.y, pos.x, pos.y, 2.0, BLUE);
                }
                prev_pos = Some(pos);
            }
        }

        next_frame().await
    }
}
