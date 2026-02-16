use macroquad::prelude::*;
use num_rational::Ratio;
use ::rand::Rng; // Import rand trait for thread_rng

mod nile;
mod fungus;

use nile::EgyptianFraction;
use fungus::{Floodplain, ColonyState};

struct River {
    spawn_timer: f32,
    spawn_rate: f32, // Seconds per task
}

impl River {
    fn new() -> Self {
        Self {
            spawn_timer: 0.0,
            spawn_rate: 3.0,
        }
    }

    fn update(&mut self, dt: f32) -> Option<Ratio<u64>> {
        self.spawn_timer += dt;
        if self.spawn_timer >= self.spawn_rate {
            self.spawn_timer = 0.0;

            // Generate random demand
            // Keep it simple: n/d where d in [2..10]
            let mut rng = ::rand::thread_rng();
            let den = rng.gen_range(2..12);
            let num = rng.gen_range(1..den); // Proper fraction < 1

            Some(Ratio::new(num, den))
        } else {
            None
        }
    }
}

#[macroquad::main("Myco-Nile")]
async fn main() {
    let mut river = River::new();
    // Use smaller area reference to make colonies visible but not huge
    // Screen is usually 800x600 = 480,000 pixels.
    // Let's say total area reference is 100,000.
    // Then 1/2 = 50,000 pixels = huge blob.
    // 1/10 = 10,000 pixels = decent blob.
    let total_area_ref = 150_000.0;

    let mut floodplain = Floodplain::new(Rect::new(0.0, 50.0, screen_width(), screen_height() - 50.0));

    let mut last_alloc_msg = String::from("Waiting for the flood...");

    loop {
        let dt = get_frame_time();
        floodplain.bounds = Rect::new(0.0, 50.0, screen_width(), screen_height() - 50.0);

        // River Logic
        if let Some(demand) = river.update(dt) {
            let frac = EgyptianFraction::from(demand);
            last_alloc_msg = format!("Allocated {} -> {}", demand, frac);

            // Spawn colonies for each part
            let w = screen_width();
            for &d in &frac.parts {
                // Random x position along the river
                let x = ::macroquad::rand::gen_range(50.0, w - 50.0);
                let pos = vec2(x, 60.0); // Start just below river
                floodplain.add_colony(d, pos, total_area_ref);
            }
        }

        // Input
        if is_key_pressed(KeyCode::Space) {
            // Force spawn
             let mut rng = ::rand::thread_rng();
             let den = rng.gen_range(2..8);
             let num = rng.gen_range(1..den);
             let demand = Ratio::new(num, den);

             let frac = EgyptianFraction::from(demand);
             last_alloc_msg = format!("Manual {} -> {}", demand, frac);

             let w = screen_width();
             for &d in &frac.parts {
                let x = ::macroquad::rand::gen_range(50.0, w - 50.0);
                let pos = vec2(x, 60.0);
                floodplain.add_colony(d, pos, total_area_ref);
             }
        }

        // Update Simulation
        floodplain.update(dt);

        // Draw
        clear_background(Color::new(0.1, 0.05, 0.0, 1.0)); // Dark soil

        // Draw River
        draw_rectangle(0.0, 0.0, screen_width(), 50.0, BLUE);
        draw_text("THE NILE", 20.0, 30.0, 30.0, WHITE);

        // Draw Floodplain
        floodplain.draw();

        // Draw UI
        draw_text(&last_alloc_msg, 20.0, screen_height() - 20.0, 20.0, WHITE);

        let stats = format!("Colonies: {} | Nodes: {}", floodplain.colonies.len(), floodplain.nodes.len());
        draw_text(&stats, screen_width() - 300.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
