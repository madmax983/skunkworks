use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

mod chaos;
mod fungus;

use chaos::ChaosSubstrate;
use fungus::HyphaeNetwork;

const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 200;

#[macroquad::main("Chaotic Mycelium")]
async fn main() {
    let mut rng = thread_rng();

    // Initialize Chaos Substrate
    let mut substrate = ChaosSubstrate::new(GRID_WIDTH, GRID_HEIGHT);
    let mut sequence = "BAB".to_string(); // Initial sequence
    let mut a_range = (2.8, 4.0);
    let mut b_range = (2.8, 4.0);

    substrate.generate(&sequence, a_range, b_range);
    substrate.texture.set_filter(FilterMode::Nearest);

    // Initialize Fungus
    let start_pos = IVec2::new(GRID_WIDTH as i32 / 2, GRID_HEIGHT as i32 / 2);
    let mut fungus = HyphaeNetwork::new(GRID_WIDTH, GRID_HEIGHT, start_pos);

    // Initial Target
    let target = IVec2::new(
        rng.gen_range(10..GRID_WIDTH as i32 - 10),
        rng.gen_range(10..GRID_HEIGHT as i32 - 10),
    );
    fungus.set_target(target);

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let cell_w = screen_w / GRID_WIDTH as f32;
        let cell_h = screen_h / GRID_HEIGHT as f32;

        // --- Input ---
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let grid_x = (mpos.0 / cell_w) as i32;
            let grid_y = (mpos.1 / cell_h) as i32;

            if grid_x >= 0
                && grid_x < GRID_WIDTH as i32
                && grid_y >= 0
                && grid_y < GRID_HEIGHT as i32
            {
                fungus.set_target(IVec2::new(grid_x, grid_y));
            }
        }

        if is_key_pressed(KeyCode::Space) {
            // Regenerate Chaos
            let seqs = ["A", "B", "AB", "BA", "AAB", "ABB", "BBA", "BBAB", "AAAAAB"];
            sequence = seqs[rng.gen_range(0..seqs.len())].to_string();

            // Randomize ranges slightly
            a_range = (rng.gen_range(2.0..3.0), rng.gen_range(3.5..4.0));
            b_range = (rng.gen_range(2.0..3.0), rng.gen_range(3.5..4.0));

            substrate.generate(&sequence, a_range, b_range);

            // Reset fungus but keep target if possible
            fungus = HyphaeNetwork::new(GRID_WIDTH, GRID_HEIGHT, start_pos);
            // New random target
            let target = IVec2::new(
                rng.gen_range(10..GRID_WIDTH as i32 - 10),
                rng.gen_range(10..GRID_HEIGHT as i32 - 10),
            );
            fungus.set_target(target);
        }

        // --- Update ---
        // Speed up simulation
        fungus.update(&substrate, 50);

        // --- Draw ---
        clear_background(BLACK);

        // Draw Chaos
        draw_texture_ex(
            &substrate.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                ..Default::default()
            },
        );

        // Set camera to match grid coordinates 0..GRID_WIDTH, 0..GRID_HEIGHT
        // Note: macroquad coordinates are usually top-left origin, Y down.
        // Camera centers the view.
        // If we want 0,0 to be top left, we need to be careful with camera target.

        // Standard macroquad coordinate system: 0,0 is top left.
        // If we use a camera with zoom 1.0, 0,0 is center of screen.

        // Let's manually map coordinates instead of using camera, to match the texture draw exactly.
        // The fungus.draw took `cell_size`. Let's pass the dynamic cell size.
        // But we have different X and Y scales.
        // Fungus draw needs to handle non-square cells if we want perfect alignment.
        // I'll update fungus.draw to take cell_w and cell_h.

        // For now, let's just use min scale and center it? No, texture stretches.
        // We must stretch fungus drawing too.

        // Since I can't easily change fungus.draw signature without rewriting it (and I don't want to rewrite it right now),
        // I will use `set_camera` to squash the view space to match the grid aspect ratio?

        // Actually, let's just make `fungus.draw` take `vec2(cell_w, cell_h)`.
        // Wait, I can't change `fungus.rs` in this step easily without another tool call.
        // I will use a simple camera transform.

        let cam = Camera2D {
            // We want the viewport [0, 0] to [screen_w, screen_h]
            // to map to [0, 0] to [GRID_WIDTH, GRID_HEIGHT]

            // Macroquad camera:
            // target is the point in world space that is at the center of the screen.
            target: vec2(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0),

            // zoom is how much of world space is visible.
            // 2.0 / visible_world_width
            zoom: vec2(2.0 / GRID_WIDTH as f32, 2.0 / GRID_HEIGHT as f32),

            // We need to flip Y because macroquad's default camera Y is up, but our grid Y is down (like screen).
            // Actually macroquad 2D default is Y down. Camera2D default is Y up.
            // So we need negative Y zoom to flip it back to Y down.
            ..Default::default()
        };

        // But wait, if I flip Y, then 0 is at bottom?
        // Let's stick to positive Y zoom (Y up) and flip the drawing?
        // Or just accept Y up?
        // Texture drawing `draw_texture_ex` is screen space (Y down).
        // `fungus` logic uses Y down (0 is top).
        // If I use a camera, I need to align them.

        // If I set zoom.y to negative, Y goes down.
        let mut cam = cam;
        cam.zoom.y = -cam.zoom.y;

        set_camera(&cam);

        fungus.draw(1.0); // Draw in grid units

        // Draw Target
        if let Some(t) = fungus.target {
            draw_circle(t.x as f32 + 0.5, t.y as f32 + 0.5, 2.0, YELLOW);
        }

        set_default_camera();

        // --- UI ---
        draw_rectangle(0., 0., screen_w, 80., Color::new(0., 0., 0., 0.7));
        draw_text("Chaotic Mycelium", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Sequence: {}", sequence), 20.0, 50.0, 20.0, YELLOW);
        draw_text(
            "Left Click: Set Food | Space: New World",
            20.0,
            70.0,
            20.0,
            GRAY,
        );

        // Stats
        let mouse_pos = mouse_position();
        let gx = (mouse_pos.0 / cell_w) as i32;
        let gy = (mouse_pos.1 / cell_h) as i32;
        let cost = substrate.get_cost(gx, gy);
        draw_text(
            &format!("Cost at Mouse: {:.2}", cost),
            screen_w - 200.,
            30.,
            20.,
            WHITE,
        );

        next_frame().await
    }
}
