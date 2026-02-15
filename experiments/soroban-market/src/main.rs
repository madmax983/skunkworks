use macroquad::prelude::*;

mod market;
mod soroban;

use crate::market::{Market, Signal, Trader};
use crate::soroban::Soroban;

const BEAD_RADIUS: f32 = 12.0;
const BEAD_HEIGHT: f32 = 18.0;
const ROD_SPACING: f32 = 35.0;
const ROD_COUNT: usize = 13;
const BEAM_Y: f32 = 200.0;
const FRAME_X: f32 = 50.0;
const FRAME_Y: f32 = 50.0;
const UPPER_BEAD_REST_Y: f32 = FRAME_Y + 30.0;
const UPPER_BEAD_ACTIVE_Y: f32 = BEAM_Y - BEAD_HEIGHT - 5.0;
const LOWER_BEAD_ACTIVE_START_Y: f32 = BEAM_Y + 5.0;

fn draw_bead(x: f32, y: f32, color: Color) {
    draw_poly(x, y + BEAD_HEIGHT / 2.0, 6, BEAD_RADIUS, 90.0, color);
    draw_poly(
        x - 3.0,
        y + BEAD_HEIGHT / 2.0 - 3.0,
        6,
        BEAD_RADIUS * 0.3,
        90.0,
        WHITE,
    );
}

fn draw_soroban(soroban: &Soroban) {
    // Draw Frame
    draw_rectangle(
        FRAME_X - 20.0,
        FRAME_Y - 20.0,
        (ROD_COUNT as f32) * ROD_SPACING + 40.0,
        400.0,
        BROWN,
    );
    draw_rectangle(
        FRAME_X - 10.0,
        FRAME_Y - 10.0,
        (ROD_COUNT as f32) * ROD_SPACING + 20.0,
        380.0,
        BEIGE,
    );

    // Draw Beam
    draw_line(
        FRAME_X - 10.0,
        BEAM_Y,
        FRAME_X + (ROD_COUNT as f32) * ROD_SPACING + 10.0,
        BEAM_Y,
        5.0,
        BLACK,
    );

    for (i, col) in soroban.columns.iter().enumerate() {
        // Reverse index for visualization (Right to Left is standard for numbers)
        let rod_x = FRAME_X + ((ROD_COUNT - 1 - i) as f32) * ROD_SPACING + ROD_SPACING / 2.0;

        // Draw Rod
        draw_line(rod_x, FRAME_Y, rod_x, FRAME_Y + 380.0, 3.0, DARKGRAY);

        // Draw Upper Bead (Heaven)
        let upper_y = if col.upper_active {
            UPPER_BEAD_ACTIVE_Y
        } else {
            UPPER_BEAD_REST_Y
        };
        draw_bead(rod_x, upper_y, RED);

        // Draw Lower Beads (Earth)
        let active_count = col.lower_active;
        for b in 0..4 {
            let is_up = (b as u8) < active_count;

            let y = if is_up {
                LOWER_BEAD_ACTIVE_START_Y + (b as f32) * BEAD_HEIGHT
            } else {
                LOWER_BEAD_ACTIVE_START_Y + (b as f32) * BEAD_HEIGHT + 60.0
            };

            draw_bead(rod_x, y, BLUE);
        }
    }
}

fn draw_market(market: &Market, sma: u64, signal: Signal) {
    let base_x = 50.0;
    let base_y = 450.0;
    let width = 600.0;
    let height = 120.0;

    draw_rectangle(base_x, base_y, width, height, BLACK);

    // Scale prices
    let min_p = *market.prices.iter().min().unwrap_or(&0).min(&sma);
    let max_p = *market.prices.iter().max().unwrap_or(&10000).max(&sma);
    let range = (max_p - min_p).max(1) as f32;

    if market.prices.len() < 2 {
        return;
    }

    let step_x = width / (market.prices.len().max(10) as f32);

    // Draw prices
    for i in 0..market.prices.len() - 1 {
        let p1 = market.prices[i];
        let p2 = market.prices[i + 1];

        let y1 = base_y + height - ((p1 - min_p) as f32 / range) * height;
        let y2 = base_y + height - ((p2 - min_p) as f32 / range) * height;

        let x1 = base_x + (i as f32) * step_x;
        let x2 = base_x + ((i + 1) as f32) * step_x;

        draw_line(x1, y1, x2, y2, 2.0, GREEN);
    }

    // Draw SMA
    let sma_y = base_y + height - ((sma - min_p) as f32 / range) * height;
    draw_line(base_x, sma_y, base_x + width, sma_y, 2.0, YELLOW);

    draw_text(
        &format!("SMA: {}", sma),
        base_x + 10.0,
        base_y + 20.0,
        20.0,
        YELLOW,
    );
    draw_text(
        &format!("Price: {}", market.current_price),
        base_x + 10.0,
        base_y + 40.0,
        20.0,
        GREEN,
    );

    // Draw Signal
    let (text, color) = match signal {
        Signal::Buy => ("BUY!", GREEN),
        Signal::Sell => ("SELL!", RED),
        Signal::Hold => ("HOLD", GRAY),
    };
    draw_text(text, base_x + width - 100.0, base_y + 40.0, 40.0, color);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Soroban HFT".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut market = Market::new();
    let mut trader = Trader::new(20); // 20 tick SMA

    // Seed market with history
    for _ in 0..100 {
        market.update();
        trader.process(&market);
    }

    loop {
        clear_background(LIGHTGRAY);

        market.update();
        trader.process(&market);

        // Keep market history manageable
        if market.prices.len() > 200 {
            market.prices.drain(0..50);
        }

        draw_soroban(&trader.soroban);
        draw_market(&market, trader.current_sma, trader.last_signal);

        draw_text("SOROBAN HFT", 50.0, 30.0, 30.0, BLACK);
        draw_text("Abacus Logic Active", 550.0, 30.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
