mod world;

use clap::Parser;
use macroquad::prelude::*;
use world::{AgentKind, Material, World, HEIGHT, WIDTH};
use ::rand::Rng; // Use the external rand crate trait

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in headless mode (no window, output image)
    #[arg(long)]
    headless: bool,
}

#[macroquad::main("Thermo-Termites")]
async fn main() {
    let args = Args::parse();

    let mut world = World::new();
    let mut rng = ::rand::thread_rng();

    // Init World
    // Add central server block
    world.add_server_block(WIDTH / 2 - 20, HEIGHT / 2 - 20, 40, 40);
    // Add some random server blocks
    for _ in 0..5 {
        let x = rng.gen_range(50..WIDTH - 50);
        let y = rng.gen_range(50..HEIGHT - 50);
        world.add_server_block(x, y, 20, 20);
    }

    // Add random walls (noise)
    for _ in 0..5000 {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let idx = world.get_index(x, y);
        if world.grid[idx].material == Material::Empty {
             world.grid[idx].material = Material::Wall;
        }
    }

    // Add Agents
    // 50k Air, 50k Termites
    for _ in 0..50000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(world::Agent::new_air(x, y));
    }
    for _ in 0..50000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(world::Agent::new_termite(x, y));
    }

    let texture = Texture2D::from_image(&Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK));
    let mut render_target = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    let max_frames = if args.headless { 100 } else { u64::MAX };

    loop {
        world.update();

        // Render to Image
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = world.get_cell(x, y);
                let color = match cell.material {
                    Material::Server => RED,
                    Material::Wall => WHITE,
                    Material::Empty => {
                        // Heat Map
                        let h = (cell.heat / 100.0).clamp(0.0, 1.0);
                        let p = (cell.pheromone / 50.0).clamp(0.0, 1.0);

                        // Blue -> Red gradient
                        Color::new(h, p * 0.5, 0.2 + (1.0 - h) * 0.2, 1.0)
                    }
                };
                render_target.set_pixel(x as u32, y as u32, color);
            }
        }

        // Draw Agents
        for agent in &world.agents {
            let x = agent.x as u32;
            let y = agent.y as u32;
            if x < WIDTH as u32 && y < HEIGHT as u32 {
                let color = match agent.kind {
                    AgentKind::Air => Color::new(0.5, 0.8, 1.0, 0.5), // Semi-transparent cyan
                    AgentKind::Termite => if agent.carrying { GREEN } else { BLUE },
                };
                // Set pixel directly
                render_target.set_pixel(x, y, color);
            }
        }

        texture.update(&render_target);

        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", world.agents.len()), 10.0, 50.0, 30.0, WHITE);
        draw_text(&format!("Step: {}", world.step), 10.0, 80.0, 30.0, WHITE);

        if args.headless {
            if world.step >= max_frames {
                // Save using image crate
                let bytes = &render_target.bytes;
                if let Err(e) = image::save_buffer("output.png", bytes, WIDTH as u32, HEIGHT as u32, image::ColorType::Rgba8) {
                    eprintln!("Failed to save image: {}", e);
                } else {
                    println!("Headless mode complete. Saved output.png");
                }
                break;
            }
        }

        next_frame().await;
    }
}
