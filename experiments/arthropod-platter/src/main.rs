use arthropod::Button;
use macroquad::prelude::*;
use platter::Platter;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod × Platter".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Ensure CI headless builds do not panic due to X11 dependency
fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting early to avoid X11 panic.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut platter = Platter::new(80, 60);

    let btn_heat = Button::new("Heat +", 20.0, 20.0, 100.0, 40.0)
        .with_colors(RED, ORANGE, YELLOW);

    let btn_clear = Button::new("Clear", 20.0, 70.0, 100.0, 40.0)
        .with_colors(GRAY, LIGHTGRAY, WHITE);

    loop {
        clear_background(BLACK);

        if btn_heat.draw() {
            // Drop a large blob of heat in the center
            for y in 20..40 {
                for x in 30..50 {
                    let dx = x as f32 - 40.0;
                    let dy = y as f32 - 30.0;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 10.0 {
                        platter.accumulate(x, y, 1.0);
                    }
                }
            }
        }

        if btn_clear.draw() {
            platter.clear();
        }

        // Decay the platter over time
        platter.decay(0.98);

        // Render the platter
        let cell_w = screen_width() / platter.width() as f32;
        let cell_h = screen_height() / platter.height() as f32;

        for y in 0..platter.height() {
            for x in 0..platter.width() {
                let heat = platter.get(x, y) as f32;
                if heat > 0.05 {
                    let color = Color::new(heat.min(1.0), (heat - 0.5).max(0.0).min(1.0), 0.0, 1.0);
                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, color);
                }
            }
        }

        // Draw buttons again so they're on top of the heat map
        btn_heat.draw();
        btn_clear.draw();

        next_frame().await;
    }
}
