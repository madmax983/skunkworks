mod landscape;
mod optimizer;
mod agent;

use macroquad::prelude::*;
use landscape::*;
use optimizer::OptimizerType;
use agent::Plant;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gradient Garden".to_owned(),
        window_width: 1024,
        window_height: 768,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut plants: Vec<Plant> = Vec::new();
    let landscapes: Vec<Box<dyn ObjectiveFunction>> = vec![
        Box::new(GaussianHills),
        Box::new(Rastrigin),
        Box::new(Rosenbrock),
        Box::new(Ackley),
        Box::new(EggHolder),
    ];
    let mut current_landscape_idx = 0;
    let mut optimizer_type = OptimizerType::SGD;
    let learning_rate = 0.05;

    // Camera Setup
    // We want World (0,0) at Screen Center.
    // World range roughly -10 to 10.
    // Screen aspect ratio affects visible range.
    let aspect_ratio = screen_width() / screen_height();
    let view_height = 20.0;
    let view_width = view_height * aspect_ratio;

    let mut camera = Camera2D {
        zoom: vec2(1.0 / (view_width / 2.0), -1.0 / (view_height / 2.0)),
        target: vec2(0.0, 0.0),
        ..Default::default()
    };

    // Generate Initial Heatmap
    let mut heatmap_texture = generate_heatmap(&*landscapes[current_landscape_idx], 256, 256);

    loop {
        // Handle Inputs
        if is_key_pressed(KeyCode::Tab) {
            current_landscape_idx = (current_landscape_idx + 1) % landscapes.len();
            heatmap_texture = generate_heatmap(&*landscapes[current_landscape_idx], 256, 256);
            plants.clear();
        }
        if is_key_pressed(KeyCode::O) {
            optimizer_type = optimizer_type.next();
        }
        if is_key_pressed(KeyCode::C) {
            plants.clear();
        }

        // Update Camera Zoom (Scroll)
        let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        if mouse_wheel_y != 0.0 {
            camera.zoom *= 1.0 + mouse_wheel_y * 0.1;
        }

        // Mouse Click -> Plant
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let world_pos = camera.screen_to_world(vec2(mpos.0, mpos.1));
            plants.push(Plant::new(world_pos, optimizer_type));
        }

        // Update Plants
        let bounds = Rect::new(-50.0, -50.0, 100.0, 100.0); // Allow growing slightly outside view
        let current_func = &*landscapes[current_landscape_idx];

        // Parallel update if slow? No, simple loop is fine for < 1000 agents.
        for plant in &mut plants {
            plant.update(current_func, learning_rate, bounds);
        }

        // Draw
        clear_background(BLACK);

        set_camera(&camera);

        // Draw Heatmap
        // Drawn at center, covering 20x20 area roughly (or whatever generates matches)
        // We generate for -10 to 10 range.
        draw_texture_ex(
            &heatmap_texture,
            -10.0, -10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(20.0, 20.0)),
                // Flip Y?
                // In generate_heatmap, we map Image Y (0..h) to World Y (-10..10).
                // draw_texture_ex draws quad.
                // If Camera flips Y (zoom.y negative), World Y increases UP.
                // draw_texture_ex(x, y) is usually bottom-left in inverted-Y world?
                // Let's assume standard behavior: (x,y) is top-left of the quad in "Texture Space"?
                // No, it's World Space position of the texture.
                // If we see it flipped, we can flip here.
                flip_y: true, // Often needed with macroquad cameras
                ..Default::default()
            }
        );

        // Draw Grid Lines
        draw_rectangle_lines(-10.0, -10.0, 20.0, 20.0, 0.1, DARKGRAY);

        // Draw Plants
        for plant in &plants {
            plant.draw(vec2(0., 0.));
        }

        set_default_camera();

        // UI
        draw_text(&format!("Function: {}", current_func.name()), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Optimizer: {} (Press O)", optimizer_type.name()), 20.0, 60.0, 30.0, optimizer_type.color());
        draw_text(&format!("Plants: {}", plants.len()), 20.0, 90.0, 20.0, WHITE);
        draw_text("Click to Plant | Tab: Next Map | C: Clear | Scroll: Zoom", 20.0, screen_height() - 30.0, 20.0, GRAY);

        next_frame().await
    }
}

fn generate_heatmap(func: &dyn ObjectiveFunction, w: u16, h: u16) -> Texture2D {
    let mut image = Image::gen_image_color(w, h, BLACK);

    // We map pixel (x,y) to world coords in [-10, 10]
    for y in 0..h {
        for x in 0..w {
            let wx = (x as f32 / w as f32) * 20.0 - 10.0;
            // Map Image Y (0..h) to World Y (-10..10)
            let wy = (y as f32 / h as f32) * 20.0 - 10.0;

            let val = func.value(wx, wy);

            // Normalize value for color mapping
            // Rastrigin: 0 to ~40 (inverted in code? No, we inverted return value)
            // My functions return negative values for "valleys"?
            // Let's check ranges.
            // GaussianHills: -5 to 5 approx.
            // Rastrigin: 0 to 80 (if standard). I returned `-val * 0.2`. So -16 to 0.
            // Map roughly -10 to 10 to 0..1

            let t = (val + 5.0) / 10.0;
            let color = color_map(t);
            image.set_pixel(x as u32, y as u32, color);
        }
    }

    Texture2D::from_image(&image)
}

fn color_map(t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    // Dark Blue (Deep) -> Cyan -> Green (Ground) -> Yellow -> Red (Peak)
    if t < 0.25 {
        // Deep Blue to Blue
        Color::new(0.0, 0.0, 0.5 + t*2.0, 0.8)
    } else if t < 0.5 {
        // Blue to Green
        Color::new(0.0, (t-0.25)*4.0, 1.0 - (t-0.25)*2.0, 0.8)
    } else if t < 0.75 {
        // Green to Yellow
        Color::new((t-0.5)*4.0, 1.0, 0.0, 0.8)
    } else {
        // Yellow to Red
        Color::new(1.0, 1.0 - (t-0.75)*4.0, 0.0, 0.8)
    }
}
