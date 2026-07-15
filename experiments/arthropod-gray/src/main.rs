use arthropod::Button;
use gray_scott::GrayScott;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Gray Hybrid".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Exiting.");
        return;
    }

    macroquad::Window::from_config(window_conf(), amain());
}

async fn amain() {
    let width = 200;
    let height = 150;
    let mut gs = GrayScott::new(width, height);

    // Seed center
    gs.add_chemical(width / 2, height / 2, 1.0);
    gs.add_chemical(width / 2 + 1, height / 2, 1.0);
    gs.add_chemical(width / 2, height / 2 + 1, 1.0);

    // Initial chemical parameters
    let mut feed = 0.055;
    let mut kill = 0.062;

    // UI Buttons
    // In immediate mode, we'll just draw them and let them mutate state
    let btn_spots = Button::new("Spots", 10.0, 10.0, 100.0, 30.0);
    let btn_stripes = Button::new("Stripes", 10.0, 50.0, 100.0, 30.0);
    let btn_mitosis = Button::new("Mitosis", 10.0, 90.0, 100.0, 30.0);
    let btn_chaos = Button::new("Chaos", 10.0, 130.0, 100.0, 30.0);
    let btn_feed = Button::new("Seed Cells", 10.0, 170.0, 100.0, 30.0);

    loop {
        clear_background(BLACK);

        if btn_spots.draw() {
            feed = 0.055;
            kill = 0.062;
        }
        if btn_stripes.draw() {
            feed = 0.022;
            kill = 0.051;
        }
        if btn_mitosis.draw() {
            feed = 0.036;
            kill = 0.053;
        }
        if btn_chaos.draw() {
            feed = 0.026;
            kill = 0.051;
        }
        if btn_feed.draw() {
            let cx = rand::gen_range(10, width - 10);
            let cy = rand::gen_range(10, height - 10);
            for y in cy - 5..cy + 5 {
                for x in cx - 5..cx + 5 {
                    gs.add_chemical(x, y, 1.0);
                }
            }
        }

        // Run simulation steps per frame
        for _ in 0..10 {
            gs.update(feed, kill, 1.0);
        }

        // Draw the simulation using macroquad textures or raw pixels
        // To be safe and simple, we'll draw rectangles for V concentration
        let scale = 4.0;
        let offset_x = 150.0; // Shift simulation to the right of UI

        for y in 0..height {
            for x in 0..width {
                // v is typically between 0 and 1
                let idx = gs.get_index(x, y).unwrap();
                let v = gs.v()[idx];
                if v > 0.1 {
                    let color = Color::new(v.min(1.0), v.min(1.0) * 0.5, 0.5, 1.0);
                    draw_rectangle(
                        offset_x + x as f32 * scale,
                        y as f32 * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }

        draw_text(
            format!("Feed: {:.3}", feed).as_str(),
            10.0,
            230.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Kill: {:.3}", kill).as_str(),
            10.0,
            250.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
