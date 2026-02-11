use macroquad::prelude::*;
mod model;
use model::{World, Terrain, State};

#[macroquad::main("Chimera Bridge")]
async fn main() {
    let width = 100;
    let height = 60;
    let mut world = World::new(width, height);

    // Init gap
    for y in 0..height {
        for x in 40..60 {
            world.set_terrain(x, y, Terrain::Gap);
        }
    }

    // Spawn ants
    for _ in 0..200 {
        // Spawn on the left side
        world.add_ant(10, 30);
    }

    loop {
        clear_background(BLACK);

        world.update();

        // Draw terrain
        // Adjust cell size to fit screen
        let cell_w = screen_width() / width as f32;
        let cell_h = screen_height() / height as f32;

        for y in 0..height {
            for x in 0..width {
                let t = world.get_terrain(x as i32, y as i32);
                let color = match t {
                    Terrain::Solid => BROWN,
                    Terrain::Gap => BLACK,
                    Terrain::Bridge => GOLD,
                };
                if t != Terrain::Gap {
                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, color);
                }
            }
        }

        // Draw pheromones (optional visualization)
        for y in 0..height {
            for x in 0..width {
                let p = world.pheromones[y * width + x];
                if p > 0.1 {
                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, Color::new(0.0, 1.0, 0.0, p * 0.5));
                }
            }
        }

        // Draw ants
        for ant in &world.ants {
            let color = match ant.state {
                State::Foraging => RED,
                State::Bridging => YELLOW, // Should be invisible as bridge? No, visualize them.
                State::Returning => BLUE,
            };

            // If bridging, they are part of terrain (Bridge), so maybe draw them differently or not at all?
            // The terrain drawing handles the bridge color (GOLD).
            // But let's draw them to show the VM state maybe?
            // Use VM stack top to modulate color?

            let vm_val = ant.vm.stack.last().map(|v| match v {
                chimera_lang::vm::Value::Int(i) => *i as f32,
                _ => 0.0
            }).unwrap_or(0.0);

            // Modulate color based on VM value
            let dynamic_color = if ant.state == State::Bridging {
                color // Bridge remains GOLD
            } else {
                Color::new(color.r, color.g, (color.b + (vm_val / 100.0)).min(1.0), 1.0)
            };

            if ant.state != State::Bridging {
                draw_rectangle(ant.x as f32 * cell_w, ant.y as f32 * cell_h, cell_w, cell_h, dynamic_color);
            }
        }

        draw_text("Chimera Bridge", 10.0, 20.0, 30.0, WHITE);
        draw_text("Ants run ChimeraVM to decide bridging behavior", 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}
