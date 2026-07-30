use flocking::{compute_force, FlockingParams};
use hyper_system::SystemMonitor;
use locus::Vec2;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hyper Flock".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode detected. Exiting immediately.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let num_boids = 100;
    let mut positions = Vec::with_capacity(num_boids);
    let mut velocities = Vec::with_capacity(num_boids);

    for _ in 0..num_boids {
        positions.push(Vec2::new(
            rand::gen_range(0.0, 800.0),
            rand::gen_range(0.0, 600.0),
        ));
        velocities.push(Vec2::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ));
    }

    let mut monitor = SystemMonitor::new();

    loop {
        clear_background(BLACK);
        monitor.update();

        // Map CPU usage to flocking parameters
        let cpu = monitor.cpu_usage; // 0.0 to 1.0 roughly
        let max_speed = 2.0 + (cpu * 5.0);
        let separation = 1.0 + (cpu * 3.0); // More stress = more separation
        let cohesion = 1.5 - (cpu * 1.0).clamp(0.0, 1.5); // More stress = less cohesion

        let params = FlockingParams {
            view_radius: 50.0,
            separation_radius: 15.0,
            max_speed: max_speed as f64,
            max_force: 0.1,
            separation_weight: separation as f64,
            alignment_weight: 1.0,
            cohesion_weight: cohesion as f64,
        };

        let forces: Vec<Vec2> = (0..positions.len())
            .map(|i| compute_force(&positions, &velocities, i, &params))
            .collect();

        for (i, force) in forces.iter().enumerate() {
            velocities[i] += *force;
            // Hacky limit implementation for Vec2 since locus doesn't have it explicitly documented here
            let speed =
                (velocities[i].x * velocities[i].x + velocities[i].y * velocities[i].y).sqrt();
            if speed > params.max_speed {
                velocities[i].x = (velocities[i].x / speed) * params.max_speed;
                velocities[i].y = (velocities[i].y / speed) * params.max_speed;
            }
            positions[i] += velocities[i];

            // Wrap around screen
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

            // Draw boid
            let r = 3.0;
            let _p1 = vec2(
                positions[i].x as f32 + velocities[i].x.signum() as f32 * r as f32,
                positions[i].y as f32 + velocities[i].y.signum() as f32 * r as f32,
            );
            draw_circle(
                positions[i].x as f32,
                positions[i].y as f32,
                r as f32,
                GREEN,
            );
        }

        draw_text(
            "Hyper Flock: Biological Swarm Monitor",
            10.0,
            20.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("CPU: {:.1}%", cpu * 100.0),
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
