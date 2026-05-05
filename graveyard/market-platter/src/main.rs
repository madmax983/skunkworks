//! 🧬 Splice: Cross market-sim × platter
//!
//! **Concept**: Market Heatmap Scan.
//!
//! **Lineage**:
//! - Parent A (market-sim): Provides the Continuous Double Auction (CDA) physics particle system.
//! - Parent B (platter): Provides the continuous 2D scalar field representation and decay logic.
//!
//! **Novel trait**: Market trades saturate a continuous scalar field.
//!
//! **Predicted Phenotype**: A fading visual heatmap of market activity.

use market_sim::{Grid, Particle};
use platter::Platter;
use rand::Rng;

fn main() {
    println!("🧬 market-platter: Market Heatmap Scan starting...\n");

    let width = 40;
    let height = 20;

    let mut market = Grid::new(width, height);
    let mut platter = Platter::new(width, height);

    let mut rng = rand::thread_rng();

    for step in 0..100 {
        // Randomly spawn Bids and Asks
        for _ in 0..3 {
            let x = rng.gen_range(0..width);
            market.set(x, height - 1, Particle::Bid(rng.gen()));
        }

        for _ in 0..3 {
            let x = rng.gen_range(0..width);
            market.set(x, 0, Particle::Ask(rng.gen()));
        }

        let _trades = market.update();

        // Scan market for Trade particles to saturate the platter
        let cells = market.cells.clone();
        for (idx, cell) in cells.iter().enumerate() {
            if let Particle::Trade { .. } = cell {
                let x = idx % width;
                let y = idx / width;
                platter.saturate(x, y, 1.0);
            }
        }

        platter.decay(0.9);

        // Simple CLI visualization
        println!("--- Step {} ---", step);
        for y in 0..height {
            for x in 0..width {
                let val = platter.get(x, y);
                let char = if val > 0.8 {
                    '#'
                } else if val > 0.5 {
                    '*'
                } else if val > 0.2 {
                    '.'
                } else {
                    ' '
                };
                print!("{}", char);
            }
            println!();
        }
        println!();
    }

    println!("Simulation complete.");
}
