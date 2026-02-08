mod simulation;

use macroquad::prelude::*;
use simulation::{Material, World, HEIGHT, WIDTH};

#[macroquad::main("Gravity Termites")]
async fn main() {
    let mut world = World::new();

    // Texture for rendering the grid
    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut zoom = 2.0;
    let mut offset = Vec2::new(0.0, 0.0);

    loop {
        // Input
        if is_key_down(KeyCode::Minus) || is_key_down(KeyCode::KpSubtract) {
            zoom *= 0.98;
        }
        if is_key_down(KeyCode::Equal) || is_key_down(KeyCode::KpAdd) {
            zoom *= 1.02;
        }

        let move_speed = 5.0 / zoom;
        if is_key_down(KeyCode::Left) {
            offset.x += move_speed;
        }
        if is_key_down(KeyCode::Right) {
            offset.x -= move_speed;
        }
        if is_key_down(KeyCode::Up) {
            offset.y += move_speed;
        }
        if is_key_down(KeyCode::Down) {
            offset.y -= move_speed;
        }

        // Update
        world.update();

        // Render Grid to Image
        for i in 0..world.grid.len() {
            let x = (i % WIDTH) as u32;
            let y = (i / WIDTH) as u32;
            let cell = &world.grid[i];

            let color = match cell.material {
                Material::Rock => WHITE,
                Material::Dust => Color::new(0.6, 0.5, 0.4, 1.0), // Brownish
                Material::Empty => {
                    // Visualize Potential (Gravity)
                    // Potential roughly scales with distance from mass.
                    // Let's amplify it for visibility.
                    let p = (cell.potential * 0.2).clamp(0.0, 1.0);
                    // Blue gradient for gravity field
                    Color::new(0.0, p * 0.5, p, 1.0)
                }
            };
            image.set_pixel(x, y, color);
        }

        texture.update(&image);

        // Draw
        clear_background(BLACK);

        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        // Grid is 0..WIDTH, 0..HEIGHT
        // We want (WIDTH/2, HEIGHT/2) to be at screen_center + offset

        // Transform:
        // World Pos (wx, wy) -> Screen Pos (sx, sy)
        // sx = (wx - WIDTH/2 + offset.x) * zoom + screen_width/2
        // Actually simpler:
        // dest_pos is top-left corner of the grid on screen

        let grid_w = WIDTH as f32 * zoom;
        let grid_h = HEIGHT as f32 * zoom;

        let dest_x = screen_center.x - grid_w / 2.0 + offset.x * zoom;
        let dest_y = screen_center.y - grid_h / 2.0 + offset.y * zoom;

        draw_texture_ex(
            &texture,
            dest_x,
            dest_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(grid_w, grid_h)),
                ..Default::default()
            },
        );

        // Draw Agents
        // Agent pos is in grid coords (0..WIDTH)
        for agent in &world.agents {
            let ax = agent.pos.x * zoom + dest_x;
            let ay = agent.pos.y * zoom + dest_y;

            // Cull off-screen
            if ax > -10.0 && ax < screen_width() + 10.0 && ay > -10.0 && ay < screen_height() + 10.0
            {
                let color = if agent.carrying { GREEN } else { YELLOW };
                // Draw as small rect
                draw_rectangle(ax, ay, zoom.max(1.0), zoom.max(1.0), color);
            }
        }

        // UI
        draw_text("Gravity Termites", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Step: {}", world.step),
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Agents build planetoids in gravity wells.",
            10.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Controls: Arrows (Pan), +/- (Zoom)",
            10.0,
            100.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
