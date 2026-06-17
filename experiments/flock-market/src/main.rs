use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use macroquad::prelude::*;
use market_sim::{Grid, Particle};
use std::env;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const NUM_BOIDS: usize = 200;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flock Market".to_owned(),
        ..Default::default()
    }
}

async fn run_sim(headless: bool) {
    let mut market = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    let mut ids = Vec::new();

    let params = FlockingParams {
        view_radius: 10.0,
        separation_radius: 2.0,
        max_speed: 1.0,
        max_force: 0.05,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    // Initialize boids
    for i in 0..NUM_BOIDS {
        positions.push(Vec2::new(
            macroquad::rand::gen_range(0.0, GRID_WIDTH as f64),
            macroquad::rand::gen_range(0.0, GRID_HEIGHT as f64),
        ));
        velocities.push(Vec2::new(
            macroquad::rand::gen_range(-1.0, 1.0),
            macroquad::rand::gen_range(-1.0, 1.0),
        ));
        ids.push(i as usize);
    }

    if headless {
        println!("flock-market running. Headless: {}", headless);
        // Step once to prove logic works without panicking
        let forces: Vec<Vec2> = (0..positions.len())
            .map(|i| compute_force(&positions, &velocities, i, &params))
            .collect();
        for (i, force) in forces.iter().enumerate() {
            velocities[i] += *force;
            velocities[i] = velocities[i].limit(params.max_speed);
            positions[i] += velocities[i];
        }
        return;
    }

    loop {
        clear_background(BLACK);

        // Calculate flocking forces
        let forces: Vec<Vec2> = (0..positions.len())
            .map(|i| compute_force(&positions, &velocities, i, &params))
            .collect();

        // Update positions & inject into market
        // Note: the market grid handles annihilation, so some boids conceptually execute trades.
        // For visualizer simplicity, we let the boids live continuously and act as constant liquidity providers.
        for (i, force) in forces.iter().enumerate() {
            velocities[i] += *force;
            velocities[i] = velocities[i].limit(params.max_speed);
            positions[i] += velocities[i];

            // Wrap around edges
            if positions[i].x < 0.0 {
                positions[i].x += GRID_WIDTH as f64;
            } else if positions[i].x >= GRID_WIDTH as f64 {
                positions[i].x -= GRID_WIDTH as f64;
            }
            if positions[i].y < 0.0 {
                positions[i].y += GRID_HEIGHT as f64;
            } else if positions[i].y >= GRID_HEIGHT as f64 {
                positions[i].y -= GRID_HEIGHT as f64;
            }

            let grid_x = positions[i].x as usize % GRID_WIDTH;
            let grid_y = positions[i].y as usize % GRID_HEIGHT;

            // If moving upwards (y decreasing), it's seeking a lower price?
            // In market-sim: y=0 is High Price, y=H is Low Price.
            // Bids move up from bottom (y increasing towards 0, i.e. going UP)
            // Asks move down from top (y increasing towards H, i.e. going DOWN)
            if velocities[i].y < 0.0 {
                // Moving UP (towards y=0)
                market.set(grid_x, grid_y, Particle::Bid(ids[i]));
            } else {
                // Moving DOWN (towards y=H)
                market.set(grid_x, grid_y, Particle::Ask(ids[i]));
            }
        }

        let trades = market.update();

        // Draw market grid and boids
        let cell_w = screen_width() / GRID_WIDTH as f32;
        let cell_h = screen_height() / GRID_HEIGHT as f32;

        // Draw Boids as arrows/triangles
        for pos in &positions {
            draw_circle(pos.x as f32 * cell_w, pos.y as f32 * cell_h, 2.0, GRAY);
        }

        // Highlight trades
        for trade in trades {
            // We don't have x, just price (y) and buyer/seller.
            // We'll draw a flash line at the price level.
            draw_line(
                0.0,
                trade.price as f32 * cell_h,
                screen_width(),
                trade.price as f32 * cell_h,
                4.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    if headless {
        // Run headless directly without initializing the window, we just block on the async future
        futures::executor::block_on(run_sim(headless));
        return;
    }

    macroquad::Window::from_config(window_conf(), run_sim(headless));
}
