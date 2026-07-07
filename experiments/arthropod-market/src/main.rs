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

async fn run() {
    let mut market = Grid::new(80, 60);

    let btn_buy = Button::new("Buy (Bid)", 10.0, 10.0, 150.0, 40.0)
        .with_colors(GREEN, LIME, DARKGREEN);
    let btn_sell = Button::new("Sell (Ask)", 10.0, 60.0, 150.0, 40.0)
        .with_colors(RED, ORANGE, MAROON);

    let mut buyer_id = 1;
    let mut seller_id = 1;

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        if btn_buy.draw() {
            // Spawn bid at bottom
            let x = rand::gen_range(0, 80);
            market.set(x, 59, Particle::Bid(buyer_id));
            buyer_id += 1;
        }

        if btn_sell.draw() {
            // Spawn ask at top
            let x = rand::gen_range(0, 80);
            market.set(x, 0, Particle::Ask(seller_id));
            seller_id += 1;
        }

        let _trades = market.update();

        let cell_w = 800.0 / 80.0;
        let cell_h = 600.0 / 60.0;

        for y in 0..60 {
            for x in 0..80 {
                let px = x as f32 * cell_w;
                let py = y as f32 * cell_h;

                match market.get(x, y) {
                    Particle::Empty => {},
                    Particle::Bid(_) => draw_rectangle(px, py, cell_w, cell_h, GREEN),
                    Particle::Ask(_) => draw_rectangle(px, py, cell_w, cell_h, RED),
                    Particle::Trade { age } => {
                        let alpha = age as f32 / market_sim::DEFAULT_TRADE_AGE as f32;
                        draw_rectangle(px, py, cell_w, cell_h, Color::new(1.0, 1.0, 0.0, alpha));
                    },
                    Particle::Wall => draw_rectangle(px, py, cell_w, cell_h, GRAY),
                }
            }
        }

        draw_text(&format!("Trades: {}", market.trade_count), 10.0, 130.0, 20.0, WHITE);
        draw_text(&format!("Bids: {}", market.total_bids), 10.0, 160.0, 20.0, WHITE);
        draw_text(&format!("Asks: {}", market.total_asks), 10.0, 190.0, 20.0, WHITE);
        draw_text(&format!("C.O.M: {:.2}", market.center_of_mass), 10.0, 220.0, 20.0, WHITE);

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Exiting immediately.");
        return;
    }

    macroquad::Window::from_config(window_conf(), run());
}
