use arthropod::Button;
use macroquad::prelude::*;
use myco_transit::{World, Agent};

fn conf() -> Conf {
    Conf {
        window_title: "Arthropod Mycelium".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn window_conf() -> Conf {
    conf()
}

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad initialization.");
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        run_headless();
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let width = 800;
    let height = 600;
    let (mut world, mut agents) = World::with_cities_and_agents(width, height, 5);

    let btn_add_city = Button::new("Add City (Pheromone Burst)", 20.0, 20.0, 250.0, 40.0)
        .with_colors(GREEN, LIME, DARKGREEN);

    let btn_clear = Button::new("Clear Trails", 20.0, 70.0, 250.0, 40.0)
        .with_colors(RED, ORANGE, DARKGRAY);

    let mut frame_image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&frame_image);

    loop {
        clear_background(BLACK);

        if btn_add_city.draw() {
            // Add a new city at a random location
            let x = rand::gen_range(50.0, width as f32 - 50.0) as f64;
            let y = rand::gen_range(50.0, height as f32 - 50.0) as f64;
            world.cities.push((x, y));

            // Add some agents for the new city
            for _ in 0..100 {
                let target_idx = rand::gen_range(0, world.cities.len());
                let agent = Agent::new(x, y, rand::gen_range(0.0, std::f64::consts::PI * 2.0), world.cities.len() - 1, target_idx);
                agents.push(agent);
            }
        }

        if btn_clear.draw() {
            for v in world.trails.iter_mut() {
                *v = 0.0;
            }
            for v in world.next_trails.iter_mut() {
                *v = 0.0;
            }
        }

        // Run simulation step
        world.update_agents_parallel(&mut agents);
        world.diffuse_and_decay();

        // Update image based on trails
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let val = world.trails[idx] as f32;

                // Color mapping
                let r = (val * 2.0).min(1.0);
                let g = val.min(1.0);
                let b = (val * 0.5).min(1.0);

                frame_image.set_pixel(x as u32, y as u32, Color::new(r, g, b, 1.0));
            }
        }

        // Draw cities
        for &(cx, cy) in &world.cities {
            let px = cx as u32;
            let py = cy as u32;
            for dy in -2..=2 {
                for dx in -2..=2 {
                    if px as i32 + dx >= 0 && px as i32 + dx < width as i32 &&
                       py as i32 + dy >= 0 && py as i32 + dy < height as i32 {
                           frame_image.set_pixel((px as i32 + dx) as u32, (py as i32 + dy) as u32, RED);
                       }
                }
            }
        }

        texture.update(&frame_image);
        draw_texture(&texture, 0.0, 0.0, WHITE);


        next_frame().await;
    }
}
