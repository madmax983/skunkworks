use arthropod::Button;
use gray_scott::GrayScott;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Gray-Scott".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Custom bypass instead of #[macroquad::main] directly to allow early return without X11 init
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), amain());
}

async fn amain() {
    let mut simulation = GrayScott::new(200, 200);

    // Initial pattern
    for i in 90..110 {
        for j in 90..110 {
            simulation.add_chemical(i, j, 1.0);
        }
    }

    // GUI Buttons
    let mut feed_rate = 0.055;
    let mut kill_rate = 0.062;
    let mut brush_size = 5;

    let inc_feed = Button::new("Inc Feed", 620.0, 50.0, 150.0, 30.0);
    let dec_feed = Button::new("Dec Feed", 620.0, 90.0, 150.0, 30.0);

    let inc_kill = Button::new("Inc Kill", 620.0, 140.0, 150.0, 30.0);
    let dec_kill = Button::new("Dec Kill", 620.0, 180.0, 150.0, 30.0);

    let inc_brush = Button::new("Brush +", 620.0, 230.0, 150.0, 30.0);
    let dec_brush = Button::new("Brush -", 620.0, 270.0, 150.0, 30.0);

    let reset_btn = Button::new("Reset", 620.0, 320.0, 150.0, 30.0);

    // Buffer for texture rendering
    let mut image = Image::gen_image_color(200, 200, WHITE);
    let texture = Texture2D::from_image(&image);

    // Scale the 200x200 sim to the screen
    let sim_draw_size = 600.0;

    loop {
        clear_background(BLACK);

        if inc_feed.draw() {
            feed_rate += 0.001;
        }
        if dec_feed.draw() {
            feed_rate -= 0.001;
        }
        if inc_kill.draw() {
            kill_rate += 0.001;
        }
        if dec_kill.draw() {
            kill_rate -= 0.001;
        }
        if inc_brush.draw() {
            brush_size = (brush_size + 1).min(20);
        }
        if dec_brush.draw() {
            brush_size = (brush_size - 1).max(1);
        }
        if reset_btn.draw() {
            simulation = GrayScott::new(200, 200);
            for i in 90..110 {
                for j in 90..110 {
                    simulation.add_chemical(i, j, 1.0);
                }
            }
        }

        draw_text(
            format!("Feed: {:.3}", feed_rate).as_str(),
            620.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Kill: {:.3}", kill_rate).as_str(),
            620.0,
            130.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Brush: {}", brush_size).as_str(),
            620.0,
            220.0,
            20.0,
            WHITE,
        );

        // Multiple steps per frame for visibility
        for _ in 0..10 {
            simulation.update(feed_rate, kill_rate, 1.0);
        }

        let (mx, my) = mouse_position();

        // Ensure mouse is within the simulation draw area before injecting
        if is_mouse_button_down(MouseButton::Left) && mx < sim_draw_size && my < sim_draw_size {
            let sim_x = (mx / sim_draw_size * 200.0) as i32;
            let sim_y = (my / sim_draw_size * 200.0) as i32;

            for dy in -brush_size..=brush_size {
                for dx in -brush_size..=brush_size {
                    if dx * dx + dy * dy <= brush_size * brush_size {
                        let px = sim_x + dx;
                        let py = sim_y + dy;
                        if (0..200).contains(&px) && (0..200).contains(&py) {
                            simulation.add_chemical(px as usize, py as usize, 1.0);
                        }
                    }
                }
            }
        }

        // Render simulation to texture
        let v_buffer = simulation.v();
        for y in 0..200 {
            for x in 0..200 {
                let idx = y * 200 + x;
                let v = v_buffer[idx];
                // Simple color mapping based on V concentration
                let color_val = (v * 255.0).clamp(0.0, 255.0) as u8;
                let color = Color::from_rgba(color_val, color_val, 255, 255);
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(sim_draw_size, sim_draw_size)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
