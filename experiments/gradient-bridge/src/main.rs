mod landscape;
mod ant;
mod model;

use macroquad::prelude::*;
use landscape::*;
use model::World;
use ant::AntState;

#[macroquad::main("Gradient Bridge")]
async fn main() {
    let mut current_idx = 0;

    // Initial World
    let make_world = |idx: usize| -> World {
        let func: Box<dyn ObjectiveFunction> = match idx {
            0 => Box::new(GaussianHills),
            1 => Box::new(Rastrigin),
            2 => Box::new(Rosenbrock),
            3 => Box::new(Ackley),
            4 => Box::new(EggHolder),
            _ => Box::new(GaussianHills),
        };
        let mut w = World::new(func);
        w.add_ants(500);
        w
    };

    let mut world = make_world(current_idx);

    // Camera
    let camera = Camera2D {
        zoom: vec2(1.0 / 10.0, -1.0 / 10.0), // -10 to 10 view
        target: vec2(0.0, 0.0),
        ..Default::default()
    };

    loop {
        // Input
        if is_key_pressed(KeyCode::R) {
            world = make_world(current_idx);
        }
        if is_key_pressed(KeyCode::Space) {
            current_idx = (current_idx + 1) % 5;
            world = make_world(current_idx);
        }

        // Update
        world.update();

        // Draw
        clear_background(BLACK);
        set_camera(&camera);

        // Draw Heatmap (Low res for speed)
        // 50x50 blocks covering -10 to 10
        let step = 20.0 / 50.0;
        for y in 0..50 {
            for x in 0..50 {
                let wx = (x as f32 * step) - 10.0;
                let wy = (y as f32 * step) - 10.0;

                let val = world.func.value(wx + step/2.0, wy + step/2.0);

                // Color mapping: -5 (Dark) to 5 (Light)
                let t = (val + 5.0) / 10.0;
                let c = color_map(t);

                draw_rectangle(wx, wy, step, step, c);
            }
        }

        // Draw Ants
        for ant in &world.ants {
            let color = match ant.state {
                AntState::Foraging => RED,
                AntState::Bridging => BLUE,
            };
            draw_circle(ant.pos.x, ant.pos.y, 0.15, color);
        }

        set_default_camera();

        draw_text(&format!("Function: {}", world.func.name()), 20.0, 30.0, 30.0, WHITE);
        draw_text("Space: Next | R: Reset", 20.0, 60.0, 20.0, GRAY);

        next_frame().await
    }
}

fn color_map(t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    // Dark Blue -> Cyan -> Green -> Yellow
    if t < 0.33 {
        Color::new(0.0, t*3.0, 0.5 + t, 1.0)
    } else if t < 0.66 {
        Color::new(0.0, 1.0, (t-0.33)*3.0, 1.0)
    } else {
        Color::new((t-0.66)*3.0, 1.0, 1.0, 1.0)
    }
}
