mod simulation;

use anyhow::Result;
use image::{ImageBuffer, Rgb};
use simulation::{World, Agent};
use std::fs;
use std::path::Path;
use std::time::Instant;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const NUM_CITIES: usize = 12; // More cities for a "Moonshot" network
const NUM_AGENTS: usize = 1_000_000;
const FRAMES: usize = 100; // Generate 100 frames for a nice clip

fn main() -> Result<()> {
    println!("🍄 Slime City: Inoculating Substrate...");

    // Create output directory
    let output_dir = Path::new("experiments/slime-city/output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Initialize World
    let (mut world, mut agents) = World::with_cities_and_agents(WIDTH, HEIGHT, NUM_CITIES, NUM_AGENTS);

    println!("🌱 Spores released: {} agents.", agents.len());
    println!("🏙️  Cities established: {}", world.cities.len());
    println!("🎨 Rendering {} frames to {:?}...", FRAMES, output_dir);

    let start_time = Instant::now();

    for frame in 0..FRAMES {
        let frame_start = Instant::now();

        // 1. Update Simulation
        world.update_agents_parallel(&mut agents);
        world.diffuse_and_decay();

        // 2. Render
        if frame % 1 == 0 { // Render every frame
            let img = render_frame(&world, &agents);
            let filename = output_dir.join(format!("frame_{:04}.png", frame));
            img.save(&filename)?;

            let duration = frame_start.elapsed();
            println!("   Frame {:04}/{:04} done in {:.2?}", frame, FRAMES, duration);
        }
    }

    let total_duration = start_time.elapsed();
    println!("✨ Simulation Complete! Total time: {:.2?}", total_duration);
    println!("📂 Output located in: {:?}", output_dir);

    Ok(())
}

fn render_frame(world: &World, agents: &[Agent]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    // 1. Render Trails (Background)
    // Map 0..255 float to color
    // We can use a color palette.
    // Low value = Black/Dark Blue
    // High value = Cyan/White
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let val = world.get_trail(x, y);
            let pixel = if val > 0.0 {
                // Non-linear mapping for better contrast
                let intensity = (val / 50.0).min(1.0);
                let b = (intensity * 255.0) as u8;
                let g = (intensity * 200.0) as u8;
                let r = (intensity * 50.0) as u8;
                Rgb([r, g, b])
            } else {
                Rgb([0, 0, 0])
            };
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // 2. Render Cities (Bright White/Yellow)
    for (cx, cy) in &world.cities {
        let cx = *cx as i32;
        let cy = *cy as i32;
        // Draw a diamond or circle
        for dy in -3..=3 {
            for dx in -3..=3 {
                if dx*dx + dy*dy < 10 {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                        img.put_pixel(px as u32, py as u32, Rgb([255, 255, 0]));
                    }
                }
            }
        }
    }

    // 3. Render Agents (Optional - can be noise or nice sparkles)
    // Rendering 1M agents might just white-out the image if they are all drawn.
    // Let's draw a subset or use additive blending?
    // ImageBuffer doesn't support additive blending easily without custom logic.
    // Let's just draw them as bright dots on top, maybe just 10% of them to show flow?
    // Or skip rendering agents and just trust the trails (Physarum visualization usually relies on trails).
    // Let's render trails mainly.
    // But let's verify agents are moving by rendering a few.

    // Actually, let's skip agent rendering for the "Mycelium Network" look.
    // The trails *are* the network.

    img
}
