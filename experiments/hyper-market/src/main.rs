//! 🧬 Splice: hyper-market
//!
//! Lineage:
//! - Parent A (market-sim): Provides discrete Continuous Double Auction physics, trade events, and market grid.
//! - Parent B (hyper-system): Provides 4D math primitives (`Vec4`) for higher-dimensional spatial projection.
//!
//! Emergent Phenotype: Financial market physics drive hyper-dimensional geometric torque, visualizing liquidity as 4D rotation.

use hyper_system::Vec4;
use macroquad::prelude::*;
use market_sim::{Grid, Particle};

fn conf() -> Conf {
    Conf {
        window_title: "Hyper Market".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad initialization.");
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        run_headless();
        return;
    }

    macroquad::Window::from_config(conf(), async_main());
}

async fn async_main() {
    // Parent A allele: Market Simulation Grid
    let mut market = Grid::new(20, 20);

    let mut rotation_xw = 0.0;
    let mut rotation_yw = 0.0;

    loop {
        clear_background(BLACK);

        // Spawn bids and asks randomly
        if macroquad::rand::gen_range(0, 100) < 10 {
            let x = macroquad::rand::gen_range(0, market.width as u32) as usize;
            let y = market.height - 1;
            market.set(x, y, Particle::Bid(macroquad::rand::gen_range(1, 1000)));
        }
        if macroquad::rand::gen_range(0, 100) < 10 {
            let x = macroquad::rand::gen_range(0, market.width as u32) as usize;
            market.set(x, 0, Particle::Ask(macroquad::rand::gen_range(1, 1000)));
        }

        let trades = market.update();
        let trade_volume = trades.len() as f32;

        if trade_volume > 0.0 {
            draw_text(
                format!("Trades this tick: {}", trade_volume).as_str(),
                20.0,
                30.0,
                30.0,
                GREEN,
            );
        }

        // Novel trait: Using market trading volume to drive 4D rotation torque
        rotation_xw += trade_volume * 0.05 + 0.01;
        rotation_yw += trade_volume * 0.03 + 0.005;

        // Parent B allele: 4D vector primitive
        let mut v = Vec4::new(100.0, 50.0, 20.0, 100.0);

        // Apply 4D rotation (XW plane)
        let cos_xw = rotation_xw.cos();
        let sin_xw = rotation_xw.sin();
        let x1 = v.x * cos_xw - v.w * sin_xw;
        let w1 = v.x * sin_xw + v.w * cos_xw;
        v.x = x1;
        v.w = w1;

        // Apply 4D rotation (YW plane)
        let cos_yw = rotation_yw.cos();
        let sin_yw = rotation_yw.sin();
        let y1 = v.y * cos_yw - v.w * sin_yw;
        let w2 = v.y * sin_yw + v.w * cos_yw;
        v.y = y1;
        v.w = w2;

        // Project 4D to 2D
        let distance = 200.0;
        let w_factor = 1.0 / (distance - v.w).max(0.1);

        let proj_x = v.x * w_factor * 200.0;
        let proj_y = v.y * w_factor * 200.0;

        draw_circle(
            400.0 + proj_x,
            300.0 + proj_y,
            10.0 * w_factor * 50.0,
            YELLOW,
        );

        next_frame().await;
    }
}
