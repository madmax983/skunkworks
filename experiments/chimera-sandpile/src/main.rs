use macroquad::prelude::*;
use clap::Parser;

mod grid;
mod agent;
mod world;

use world::World;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 128)]
    width: usize,

    #[arg(short, long, default_value_t = 128)]
    height: usize,

    #[arg(short, long, default_value_t = 100)]
    agents: usize,
}

#[macroquad::main("Chimera Sandpile")]
async fn main() {
    let args = Args::parse();

    // Create World
    let mut world = World::new(args.width, args.height, args.agents);

    loop {
        // Update window size
        let window_width = screen_width();
        let window_height = screen_height();

        let cell_w = window_width / args.width as f32;
        let cell_h = window_height / args.height as f32;
        let scale = cell_w.min(cell_h);

        // Input Handling
        if is_key_pressed(KeyCode::R) {
            world = World::new(args.width, args.height, args.agents);
        }

        // Simulation Update
        world.update();

        // Rendering
        clear_background(BLACK);

        // Draw Grid via Image/Texture
        let mut image = Image::gen_image_color(args.width as u16, args.height as u16, BLACK);

        for y in 0..args.height {
            for x in 0..args.width {
                let load = world.grid.get_load(x, y);
                let color = match load {
                    0 => BLACK,
                    1 => BLUE,
                    2 => GREEN,
                    3 => YELLOW,
                    _ => RED, // Critical
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(window_width, window_height)),
            ..Default::default()
        });

        // Draw Agents
        for agent in &world.agents {
            let (ax, ay) = agent.pos;
            let sx = (ax as f32 / args.width as f32) * window_width;
            let sy = (ay as f32 / args.height as f32) * window_height;
            let size = scale * 0.8;

            draw_circle(sx + size/2.0, sy + size/2.0, size/2.0, WHITE);
        }

        // Draw UI
        draw_text(&format!("Agents: {}", world.agents.len()), 10.0, 20.0, 20.0, WHITE);
        draw_text("R: Reset", 10.0, 40.0, 20.0, WHITE);

        next_frame().await;
    }
}
