use macroquad::prelude::*;
use market_sim::{Grid, Particle};
use ::rand::Rng;

mod mechanism;
use mechanism::{Differential, Integrator, Shaft};

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;
const SCALE: f32 = 0.5; // Scaling factor for force
const DAMPING: f32 = 0.1;

struct MarketMachine {
    grid: Grid,

    // Components
    diff: Differential, // Sums inputs from Bulls/Bears
    int_velocity: Integrator, // Integrates Net Force -> Velocity
    int_price: Integrator,    // Integrates Velocity -> Price

    // State
    price: f32,
    velocity: f32,

    // Visual State
    time_shaft: Shaft,
    history: Vec<(f32, f32)>, // (time, price)
    t: f32,
    paused: bool,
}

impl MarketMachine {
    fn new() -> Self {
        Self {
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            diff: Differential::new(),
            int_velocity: Integrator::new(),
            int_price: Integrator::new(),
            price: 20.0, // Center of 40-high grid
            velocity: 0.0,
            time_shaft: Shaft::default(),
            history: Vec::new(),
            t: 0.0,
            paused: false,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.paused { return; }

        let mut rng = ::rand::thread_rng();

        // 1. Spawn Particles (Market Maker Logic) based on Mechanical Price
        // Price is roughly mapped to Grid Height (0..40).
        // If Price is 20, we spawn Bids around 20-offset, Asks around 20+offset.

        let center_y = (GRID_HEIGHT as f32 - 1.0 - self.price).clamp(0.0, GRID_HEIGHT as f32 - 1.0);

        // Bulls (Bids)
        if rng.gen_bool(0.3) {
            let offset = rng.gen_range(1.0..10.0);
            let y = (center_y + offset) as usize;
            // Bids spawn at bottom (Low Price, High Y) and move Up (Low Y).
            if y < GRID_HEIGHT {
                let x = rng.gen_range(0..GRID_WIDTH);
                self.grid.set(x, y, Particle::Bid(0));
            }
        }

        // Bears (Asks)
        if rng.gen_bool(0.3) {
            let offset = rng.gen_range(1.0..10.0);
            let y = (center_y - offset) as isize;
            // Asks spawn at top (High Price, Low Y) and move Down (High Y).
            if y >= 0 {
                let x = rng.gen_range(0..GRID_WIDTH);
                self.grid.set(x, y as usize, Particle::Ask(0));
            }
        }

        // 2. Update Grid Physics
        self.grid.update();

        // 3. Measure Forces
        let bull_force = self.grid.total_bids as f32 * SCALE;
        let bear_force = self.grid.total_asks as f32 * SCALE;

        // 4. Update Mechanical Computer
        self.time_shaft.rotate(dt);

        // Net Force = Bids - Asks
        let net_force = bull_force - bear_force;
        let damp_force = -self.velocity * DAMPING;
        let total_force = net_force + damp_force;

        // Differential Visual Update
        self.diff.update(bull_force * dt, -bear_force * dt);

        // Integrator 1: Acceleration -> Velocity
        // Carriage = Force
        self.int_velocity.carriage_pos = total_force;
        let dv = self.int_velocity.update(dt);
        self.velocity += dv;

        // Integrator 2: Velocity -> Position
        // Carriage = Velocity
        self.int_price.carriage_pos = self.velocity;
        let dp = self.int_price.update(dt);
        self.price += dp;

        // Clamp Price
        if self.price < 0.0 { self.price = 0.0; self.velocity = 0.0; }
        if self.price > 40.0 { self.price = 40.0; self.velocity = 0.0; }

        // History
        self.t += dt;
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
        self.history.push((self.t, self.price));
    }

    fn draw(&self) {
        let w = screen_width();
        let h = screen_height();

        // Split screen: Left = Machine, Right = Grid
        let machine_w = w * 0.6;
        let grid_w = w - machine_w;

        // Draw Machine
        draw_text("MECHANICAL MARKET", 20.0, 30.0, 30.0, WHITE);

        // Draw Integrator 1 (Velocity)
        let cx = machine_w / 2.0;
        let cy = h / 3.0;
        self.draw_integrator(vec2(cx - 100.0, cy), &self.int_velocity, "Acc -> Vel");

        // Draw Integrator 2 (Price)
        self.draw_integrator(vec2(cx + 100.0, cy), &self.int_price, "Vel -> Price");

        // Draw Differential (Force)
        self.draw_differential(vec2(cx, cy - 150.0), &self.diff, "Bids - Asks");

        // Draw Plot
        self.draw_plot(vec2(20.0, h - 150.0), vec2(machine_w - 40.0, 130.0));

        // Draw Grid
        let cell_w = grid_w / GRID_WIDTH as f32;
        let cell_h = h / GRID_HEIGHT as f32;
        let start_x = machine_w;

        draw_rectangle(start_x, 0.0, grid_w, h, Color::new(0.1, 0.1, 0.1, 1.0));

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let p = self.grid.get(x, y);
                let px = start_x + x as f32 * cell_w;
                let py = y as f32 * cell_h;

                match p {
                    Particle::Bid(_) => draw_rectangle(px, py, cell_w, cell_h, GREEN),
                    Particle::Ask(_) => draw_rectangle(px, py, cell_w, cell_h, RED),
                    Particle::Trade { .. } => draw_rectangle(px, py, cell_w, cell_h, WHITE),
                    _ => {}
                }
            }
        }

        // Draw Mechanical Price Line on Grid
        let price_y = (GRID_HEIGHT as f32 - 1.0 - self.price) * cell_h;
        draw_line(start_x, price_y, w, price_y, 2.0, YELLOW);

        // UI
        draw_text(&format!("Price: {:.2}", self.price), 20.0, 60.0, 20.0, YELLOW);
        draw_text(&format!("Bids: {}", self.grid.total_bids), 20.0, 80.0, 20.0, GREEN);
        draw_text(&format!("Asks: {}", self.grid.total_asks), 20.0, 100.0, 20.0, RED);
    }

    // Helper drawing functions copied/adapted from mechanical-integrator
    fn draw_integrator(&self, pos: Vec2, integrator: &Integrator, label: &str) {
         let r = 40.0;
         draw_circle(pos.x, pos.y, r, DARKGRAY);
         draw_circle_lines(pos.x, pos.y, r, 2.0, WHITE);

         // Carriage
         let cw = (integrator.carriage_pos * 10.0).clamp(-r, r);
         draw_line(pos.x - r, pos.y, pos.x + r, pos.y, 1.0, GRAY);
         draw_rectangle(pos.x + cw - 5.0, pos.y - 10.0, 10.0, 20.0, RED);

         draw_text(label, pos.x - 30.0, pos.y + r + 20.0, 15.0, WHITE);
    }

    fn draw_differential(&self, pos: Vec2, _diff: &Differential, label: &str) {
        let w = 80.0;
        let h = 40.0;
        draw_rectangle(pos.x - w/2.0, pos.y - h/2.0, w, h, GOLD);
        draw_rectangle_lines(pos.x - w/2.0, pos.y - h/2.0, w, h, 2.0, WHITE);
        draw_text(label, pos.x - 40.0, pos.y + h/2.0 + 20.0, 15.0, WHITE);
    }

    fn draw_plot(&self, pos: Vec2, size: Vec2) {
        draw_rectangle(pos.x, pos.y, size.x, size.y, BLACK);
        draw_rectangle_lines(pos.x, pos.y, size.x, size.y, 2.0, WHITE);

        if self.history.len() < 2 { return; }

        let min_t = self.history.first().unwrap().0;
        let max_t = self.history.last().unwrap().0;
        let range_t = (max_t - min_t).max(1.0);

        let map_x = |t: f32| pos.x + ((t - min_t) / range_t) * size.x;
        let map_y = |p: f32| pos.y + size.y - (p / 40.0) * size.y;

        let mut prev = self.history[0];
        for &curr in self.history.iter().skip(1) {
            draw_line(map_x(prev.0), map_y(prev.1), map_x(curr.0), map_y(curr.1), 2.0, YELLOW);
            prev = curr;
        }
    }
}

#[macroquad::main("Mechanical Market")]
async fn main() {
    let mut machine = MarketMachine::new();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        // Steps
        for _ in 0..4 {
            machine.update(0.016 / 4.0);
        }

        machine.draw();

        next_frame().await
    }
}
