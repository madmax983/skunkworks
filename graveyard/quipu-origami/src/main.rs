use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use quipu::{Cord, Quipu};

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Quipu Origami Morphogenesis".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

// Implement headless bypass
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode enabled. Exiting.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    // Generate some discrete data in a Quipu
    let mut q = Quipu::new();

    // Add cords with some data
    q.add_cord(Cord::from(420)); // Extends folding range
    q.add_cord(Cord::from(77)); // Extends folding range less
    q.add_cord(Cord::from(108)); // Medium extension

    // Sum the data to determine base parameters
    let total_value: u64 = q.cords.iter().map(|c| c.value()).sum();

    // Base parameters for the origami mesh
    let base_a = 0.5 + (total_value as f32 % 100.0) / 200.0; // Varies slightly by sum
    let base_b = 0.5;

    let mut time = 0.0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        time += get_frame_time();

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Loop through each cord to render an independent origami patch
        // The discrete data (knots) determines the folding extension
        for (i, cord) in q.cords.iter().enumerate() {
            let val = cord.value() as f32;

            // Map the integer value to a breathing extension factor
            // A higher value pulses faster and larger
            let speed = 1.0 + (val % 10.0) / 5.0;
            let extension_factor = 0.4 + (val % 100.0) / 300.0 + (time * speed).sin() * 0.3;

            let params = MiuraParams {
                a: base_a,
                b: base_b,
                gamma: 70.0f32.to_radians(),
                orientation: Orientation::Horizontal,
            };

            let points = generate_miura_grid(params, (15, 15), extension_factor);

            // Layout multiple patches in a circle
            let angle = (i as f32 / q.cords.len() as f32) * std::f32::consts::PI * 2.0;
            let radius = 200.0;
            let offset_x = center_x + angle.cos() * radius;
            let offset_y = center_y + angle.sin() * radius;

            let scale = 100.0;

            for p in points.iter() {
                let screen_x = offset_x + p.x * scale;
                let screen_y = offset_y + p.y * scale;

                // Color based on cord index and z-depth
                let base_color = match i % 3 {
                    0 => Color::new(0.8, 0.2, 0.2, 1.0),
                    1 => Color::new(0.2, 0.8, 0.2, 1.0),
                    _ => Color::new(0.2, 0.2, 0.8, 1.0),
                };

                // Whiten the mountain folds, darken the valleys
                let intensity = (p.z + 1.0) / 2.0; // Map roughly -1..1 to 0..1
                let point_color = Color::new(
                    base_color.r * intensity,
                    base_color.g * intensity,
                    base_color.b * intensity,
                    1.0,
                );

                draw_circle(screen_x, screen_y, 2.0, point_color);
            }

            draw_text(
                &format!("Cord {}: {}", i, val),
                offset_x - 40.0,
                offset_y - 120.0,
                20.0,
                WHITE,
            );
        }

        draw_text(
            "Quipu Origami - Knotted Data Morphogenesis",
            20.0,
            30.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
