use ::rand::Rng;
use arthropod::Button;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Flock".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Bypass macroquad::main to support headless execution
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut rng = ::rand::thread_rng();

    let num_boids = 150;
    let mut positions = vec![Vec2::zero(); num_boids];
    let mut velocities = vec![Vec2::zero(); num_boids];

    for i in 0..num_boids {
        positions[i] = Vec2::new(rng.gen_range(100.0..700.0), rng.gen_range(100.0..500.0));
        let angle = rng.gen_range(0.0..std::f64::consts::TAU);
        velocities[i] = Vec2::new(angle.cos() * 2.0, angle.sin() * 2.0);
    }

    let mut params = FlockingParams {
        view_radius: 50.0,
        separation_radius: 20.0,
        max_speed: 4.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    let btn_scatter =
        Button::new("Scatter", 20.0, 20.0, 100.0, 40.0).with_colors(RED, ORANGE, DARKGRAY);
    let btn_group =
        Button::new("Group", 20.0, 70.0, 100.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);
    let btn_align =
        Button::new("Align", 20.0, 120.0, 100.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_reset =
        Button::new("Reset", 20.0, 170.0, 100.0, 40.0).with_colors(GRAY, LIGHTGRAY, BLACK);

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        if btn_scatter.draw() {
            params.separation_weight = 3.0;
            params.cohesion_weight = 0.1;
            params.alignment_weight = 0.1;
        }

        if btn_group.draw() {
            params.cohesion_weight = 3.0;
            params.separation_weight = 0.5;
            params.alignment_weight = 0.5;
        }

        if btn_align.draw() {
            params.alignment_weight = 3.0;
            params.separation_weight = 0.5;
            params.cohesion_weight = 0.5;
        }

        if btn_reset.draw() {
            params.separation_weight = 1.5;
            params.alignment_weight = 1.0;
            params.cohesion_weight = 1.0;
        }

        // Steer & Move
        let forces: Vec<Vec2> = (0..num_boids)
            .map(|i| compute_force(&positions, &velocities, i, &params))
            .collect();

        for i in 0..num_boids {
            velocities[i] += forces[i];
            velocities[i] = velocities[i].limit(params.max_speed);
            positions[i] += velocities[i];

            // Wrap edges
            if positions[i].x < 0.0 {
                positions[i].x += 800.0;
            }
            if positions[i].x > 800.0 {
                positions[i].x -= 800.0;
            }
            if positions[i].y < 0.0 {
                positions[i].y += 600.0;
            }
            if positions[i].y > 600.0 {
                positions[i].y -= 600.0;
            }
        }

        // Draw boids
        for pos in &positions {
            draw_circle(pos.x as f32, pos.y as f32, 4.0, WHITE);
        }

        next_frame().await;
    }
}
