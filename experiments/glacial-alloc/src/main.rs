use macroquad::prelude::*;

mod glacier;
use glacier::GlacierGrid;

#[macroquad::main("Glacial Alloc")]
async fn main() {
    let width = 64;
    let height = 64;
    let mut grid = GlacierGrid::new(width, height);

    loop {
        clear_background(BLACK);

        // Simulation Step
        // Random Allocation (Snowfall) - Simulating heap activity
        // Concentrated in the center like a mountain peak
        for _ in 0..5 {
            let x = rand::gen_range(width / 4, 3 * width / 4);
            let y = rand::gen_range(height / 4, 3 * height / 4);
            grid.allocate(x, y, 2.0);
        }

        // Flow
        grid.update_flow();

        // Interaction: Melt (Free Memory)
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let cw = screen_width() / grid.width as f32;
            let ch = screen_height() / grid.height as f32;

            // Check bounds to avoid negative casting or out of bounds
            if mx >= 0.0 && my >= 0.0 && mx < screen_width() && my < screen_height() {
                let gx = (mx / cw) as usize;
                let gy = (my / ch) as usize;

                // Brush size
                for dy in 0..3 {
                    for dx in 0..3 {
                         grid.free(gx + dx, gy + dy, 5.0);
                    }
                }
            }
        }

        // Render
        let cell_w = screen_width() / grid.width as f32;
        let cell_h = screen_height() / grid.height as f32;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let idx = y * grid.width + x;
                let h = grid.cells[idx];
                if h > 0.1 {
                    // Color gradient based on depth: White (fresh) -> Blue (deep/old)
                    let intensity = (h / 50.0).min(1.0);
                    let color = Color::new(1.0 - intensity, 1.0 - intensity * 0.5, 1.0, 1.0);
                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, color);
                }
            }
        }

        // UI
        draw_text("HEAP GLACIER", 10.0, 30.0, 30.0, WHITE);
        draw_text(format!("Total Allocations (Mass): {:.0}", grid.get_total_mass()).as_str(), 10.0, 60.0, 20.0, LIGHTGRAY);
        draw_text("Snow = Alloc | Click = Free | Flow = Memory Pressure", 10.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await;
    }
}
