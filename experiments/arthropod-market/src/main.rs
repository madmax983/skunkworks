//! # Arthropod Market 📈
//!
//! **Concept:** Interactive Market Injection.
//!
//! This hybrid visualizer crosses the immediate-mode UI library of `arthropod` with the Continuous Double Auction simulation of `market-sim`. Instead of a passive market simulation driven purely by automated agents, the user can manually inject liquidity (Bids and Asks) into the continuous physical market grid using discrete GUI buttons.
//!
//! ## Lineage
//! - **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
//! - **Parent B (crates/market-sim):** Provides the 2D grid-based physical order book simulation where particles bubble up/fall down and collide to form trades.
//!
//! ## Novel Trait
//! The discrete interaction of UI clicks (`arthropod`) maps directly to biological market pressure (`market-sim`). Clicking the Buy button spawns Bid particles that bubble up from the bottom; clicking Sell spawns Ask particles that fall from the top.
//!
//! ## Predicted Phenotype
//! An interactive financial laboratory where manual, abstract button clicks manifest as physical, colliding market particles, allowing the user to observe the price discovery process visually as a consequence of their direct input.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p arthropod-market --release
//! ```
//!
//! *(Note: Use `cargo run -p arthropod-market --release -- --headless` to safely bypass X11 UI panics in CI environments.)*

use ::rand::Rng;
use arthropod::Button;
use macroquad::prelude::*;
use market_sim::{Grid, Particle};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Market".to_owned(),
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

    let grid_width = 100;
    let grid_height = 100;
    let mut market = Grid::new(grid_width, grid_height);

    let btn_buy =
        Button::new("Buy (Bid)", 20.0, 20.0, 150.0, 40.0).with_colors(DARKGREEN, GREEN, LIME);
    let btn_sell =
        Button::new("Sell (Ask)", 20.0, 80.0, 150.0, 40.0).with_colors(MAROON, RED, ORANGE);

    let cell_size = 4.0;
    let grid_offset_x = (800.0 - (grid_width as f32 * cell_size)) / 2.0;
    let grid_offset_y = (600.0 - (grid_height as f32 * cell_size)) / 2.0;

    let mut id_counter = 1;

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        if btn_buy.draw() {
            // Spawn multiple bids at the bottom
            for _ in 0..5 {
                let x = rng.gen_range(0..grid_width);
                market.set(x, grid_height - 1, Particle::Bid(id_counter));
                id_counter += 1;
            }
        }

        if btn_sell.draw() {
            // Spawn multiple asks at the top
            for _ in 0..5 {
                let x = rng.gen_range(0..grid_width);
                market.set(x, 0, Particle::Ask(id_counter));
                id_counter += 1;
            }
        }

        // Add some ambient noise
        if rng.gen_bool(0.1) {
            market.set(
                rng.gen_range(0..grid_width),
                grid_height - 1,
                Particle::Bid(id_counter),
            );
            id_counter += 1;
        }
        if rng.gen_bool(0.1) {
            market.set(rng.gen_range(0..grid_width), 0, Particle::Ask(id_counter));
            id_counter += 1;
        }

        let _trades = market.update();

        // Draw market grid
        for y in 0..grid_height {
            for x in 0..grid_width {
                let particle = market.get(x, y);
                let color = match particle {
                    Particle::Empty => continue,
                    Particle::Wall => DARKGRAY,
                    Particle::Bid(_) => GREEN,
                    Particle::Ask(_) => RED,
                    Particle::Trade { age } => {
                        let intensity = (age as f32 / market_sim::DEFAULT_TRADE_AGE as f32) * 255.0;
                        color_u8!(255, 255, 0, intensity as u8)
                    }
                };

                draw_rectangle(
                    grid_offset_x + (x as f32 * cell_size),
                    grid_offset_y + (y as f32 * cell_size),
                    cell_size - 0.5,
                    cell_size - 0.5,
                    color,
                );
            }
        }

        draw_text(
            format!("Trades: {}", market.trade_count),
            650.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Bids: {}", market.total_bids),
            650.0,
            70.0,
            20.0,
            GREEN,
        );
        draw_text(
            format!("Asks: {}", market.total_asks),
            650.0,
            100.0,
            20.0,
            RED,
        );

        next_frame().await;
    }
}
