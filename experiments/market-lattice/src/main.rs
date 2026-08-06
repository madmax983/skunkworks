//! # market-lattice
//!
//! A hybrid of `market-sim` and `miller-lattice`.
//!
//! ## Lineage
//! - **market-sim**: Provides the Continuous Double Auction particle system where trades happen when bids and asks collide.
//! - **miller-lattice**: Provides the 3D discrete hierarchical crystal structure of the directory tree.
//!
//! ## Concept
//! The structural crystal nodes of `miller-lattice` are injected directly into the continuous market grid of `market-sim`. The hierarchical file structures act as physical "walls" or constraints within the market, disrupting the flow of bids and asks. Structural branches force trades to navigate around the directory hierarchy, exploring how codebase architecture might constrain or channel financial pressure.
//!
//! ## Execution
//! ```bash
//! # Normal visualization
//! cargo run -p market-lattice --features macroquad_run
//!
//! # Headless CI mode
//! cargo run -p market-lattice --no-default-features -- --headless
//! ```
//!
use market_sim::{Grid, Particle};
use miller_lattice::Crystal;
use std::env;
use std::path::Path;

fn run_headless() {
    println!("Running in headless mode. Bypassing macroquad window.");
    let crystal = Crystal::build_from_path(Path::new(".")).unwrap();
    println!("Crystal built with {} atoms", crystal.atoms.len());
    let mut grid = Grid::new(50, 50);
    // Insert some structural walls
    for atom in crystal.atoms.iter().take(10) {
        let x = atom.position.x.unsigned_abs() as usize % 50;
        let y = atom.position.y.unsigned_abs() as usize % 50;
        grid.set(x, y, Particle::Wall);
    }
    grid.set(25, 49, Particle::Bid(1));
    grid.set(25, 0, Particle::Ask(2));
    grid.update();
    println!("Headless market update complete.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        run_headless();
        return;
    }

    #[cfg(feature = "macroquad_run")]
    macroquad_main::main();
}

#[cfg(feature = "macroquad_run")]
mod macroquad_main {
    use super::*;
    use macroquad::prelude::*;

    fn window_conf() -> Conf {
        Conf {
            window_title: "Market Lattice".to_owned(),
            ..Default::default()
        }
    }

    #[macroquad::main(window_conf)]
    pub async fn main() {
        let width = 100;
        let height = 100;
        let mut grid = Grid::new(width, height);

        let crystal = Crystal::build_from_path(Path::new(".")).unwrap();

        // Map crystal atoms to grid walls
        for atom in &crystal.atoms {
            let x = (((atom.position.x as f32 * 5.0) + (width as f32 / 2.0)) as usize)
                .clamp(0, width - 1);
            let y = (((atom.position.y as f32 * 5.0) + (height as f32 / 2.0)) as usize)
                .clamp(0, height - 1);
            grid.set(x, y, Particle::Wall);
        }

        let mut next_buyer_id = 1;
        let mut next_seller_id = 1;
        let mut total_trades = 0;

        loop {
            // Spawn participants randomly
            if rand::gen_range(0, 100) < 30 {
                let x = rand::gen_range(0, width);
                if matches!(grid.get(x, height - 1), Particle::Empty) {
                    grid.set(x, height - 1, Particle::Bid(next_buyer_id));
                    next_buyer_id += 1;
                }
            }
            if rand::gen_range(0, 100) < 30 {
                let x = rand::gen_range(0, width);
                if matches!(grid.get(x, 0), Particle::Empty) {
                    grid.set(x, 0, Particle::Ask(next_seller_id));
                    next_seller_id += 1;
                }
            }

            let trades = grid.update();
            total_trades += trades.len();

            clear_background(BLACK);

            let cell_w = screen_width() / width as f32;
            let cell_h = screen_height() / height as f32;

            for y in 0..height {
                for x in 0..width {
                    match grid.get(x, y) {
                        Particle::Bid(_) => {
                            draw_rectangle(
                                x as f32 * cell_w,
                                y as f32 * cell_h,
                                cell_w,
                                cell_h,
                                GREEN,
                            );
                        }
                        Particle::Ask(_) => {
                            draw_rectangle(
                                x as f32 * cell_w,
                                y as f32 * cell_h,
                                cell_w,
                                cell_h,
                                RED,
                            );
                        }
                        Particle::Trade { age } => {
                            let alpha = age as f32 / 10.0;
                            draw_rectangle(
                                x as f32 * cell_w,
                                y as f32 * cell_h,
                                cell_w,
                                cell_h,
                                Color::new(1.0, 1.0, 0.0, alpha),
                            );
                        }
                        Particle::Wall => {
                            draw_rectangle(
                                x as f32 * cell_w,
                                y as f32 * cell_h,
                                cell_w,
                                cell_h,
                                GRAY,
                            );
                        }
                        Particle::Empty => {}
                    }
                }
            }

            draw_text(
                format!("Trades executed: {}", total_trades),
                10.0,
                20.0,
                20.0,
                WHITE,
            );

            next_frame().await;
        }
    }
}
