use market_sim::{Grid as MarketGrid, Particle};
use gray_scott::GrayScott;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let headless = args.iter().any(|arg| arg == "--headless");

    let width = 100;
    let height = 100;

    let mut market = MarketGrid::new(width, height);
    let mut gray = GrayScott::new(width, height);
    let mut rng = rand::thread_rng();

    if headless {
        println!("Running market-gray in headless mode...");
        for _ in 0..10 {
            // Spawn random bids and asks
            if rng.gen_bool(0.3) {
                let x = rng.gen_range(0..width);
                market.set(x, height - 1, Particle::Bid(rng.gen()));
            }
            if rng.gen_bool(0.3) {
                let x = rng.gen_range(0..width);
                market.set(x, 0, Particle::Ask(rng.gen()));
            }

            let trades = market.update();

            for trade in trades {
                // Seed the V chemical using verified API
                gray.add_chemical(trade.x, trade.y, 0.5);
            }

            // Update using verified API parameters (feed, kill, dt)
            gray.update(0.055, 0.062, 1.0);
        }
        println!("Headless simulation complete.");
        return;
    }

    panic!("UI mode not implemented for market-gray. Please run with --headless");
}
