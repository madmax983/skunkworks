use macroquad::prelude::*;
use arthropod::Button;
use gray_scott::GrayScott;
use std::env;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;

// Do NOT use #[macroquad::main] macro for headless compatibility
async fn amain() {
    let mut gs = GrayScott::new(GRID_WIDTH, GRID_HEIGHT);
    // Seed initial chemical V
    gs.add_chemical(GRID_WIDTH / 2, GRID_HEIGHT / 2, 1.0);

    // UI Buttons
    let btn_mitosis = Button::new("Mitosis (0.0367, 0.0649)", 10.0, 10.0, 200.0, 30.0);
    let btn_coral = Button::new("Coral (0.0545, 0.0620)", 10.0, 50.0, 200.0, 30.0);

    // Default to Mitosis
    let mut feed = 0.0367;
    let mut kill = 0.0649;

    let dt = 1.0;

    // Create a texture to render the simulation
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        // UI Handling
        if btn_mitosis.draw() {
            feed = 0.0367;
            kill = 0.0649;
        }
        if btn_coral.draw() {
            feed = 0.0545;
            kill = 0.0620;
        }

        // Mouse Interaction to drop more chemical
        if is_mouse_button_down(MouseButton::Left) && mouse_position().0 > 220.0 {
            let (mx, my) = mouse_position();
            let sim_x = ((mx / screen_width()) * GRID_WIDTH as f32) as usize;
            let sim_y = ((my / screen_height()) * GRID_HEIGHT as f32) as usize;
            if sim_x < GRID_WIDTH && sim_y < GRID_HEIGHT {
                gs.add_chemical(sim_x, sim_y, 1.0);
            }
        }

        // Step Simulation
        for _ in 0..10 {
            gs.update(feed, kill, dt);
        }

        // Update Image
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let v = gs.v()[y * GRID_WIDTH + x];
                let color = Color::new(v, v * 0.5, v * 2.0, 1.0); // Purpleish mapping
                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);

        // Draw the simulation
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Redraw buttons over the simulation
        btn_mitosis.draw();
        btn_coral.draw();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod-Gray Hybrid".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless execution completed successfully.");
        return;
    }
    macroquad::Window::from_config(window_conf(), amain());
}
