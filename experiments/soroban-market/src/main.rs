mod abacus;
mod market;

use abacus::{Soroban, BeadMove, BeadType, Action};
use market::{Market, MarketEvent, AbacusId};
use macroquad::prelude::*;
use std::collections::VecDeque;

const COLUMN_WIDTH: f32 = 40.0;
const BEAD_HEIGHT: f32 = 20.0;
const BEAD_WIDTH: f32 = 30.0;
const FRAME_PADDING: f32 = 10.0;
const BEAM_Y_OFFSET: f32 = 100.0; // Relative to top of soroban
const HEAVEN_BEAD_Y_ACTIVE: f32 = BEAM_Y_OFFSET - BEAD_HEIGHT - 2.0;
const HEAVEN_BEAD_Y_INACTIVE: f32 = 20.0;
const EARTH_BEAD_Y_ACTIVE_BASE: f32 = BEAM_Y_OFFSET + 10.0;
const EARTH_BEAD_Y_INACTIVE_BASE: f32 = BEAM_Y_OFFSET + 60.0;

struct VisualColumn {
    heaven_y: f32,
    earth_ys: [f32; 4],
    heaven_active: bool,
    earth_active_count: u8,
}

impl VisualColumn {
    fn new(col: &abacus::Column) -> Self {
        let mut vc = Self {
            heaven_y: if col.heaven { HEAVEN_BEAD_Y_ACTIVE } else { HEAVEN_BEAD_Y_INACTIVE },
            earth_ys: [EARTH_BEAD_Y_INACTIVE_BASE; 4], // Default inactive positions
            heaven_active: col.heaven,
            earth_active_count: col.earth,
        };

        // Initialize earth positions based on count
        for i in 0..4 {
            vc.earth_ys[i] = if i < col.earth as usize {
                EARTH_BEAD_Y_ACTIVE_BASE + (i as f32 * BEAD_HEIGHT)
            } else {
                EARTH_BEAD_Y_INACTIVE_BASE + (i as f32 * BEAD_HEIGHT)
            };
        }
        vc
    }

    fn update(&mut self, dt: f32) {
        let speed = 500.0; // pixels per second

        // Target positions
        let target_h = if self.heaven_active { HEAVEN_BEAD_Y_ACTIVE } else { HEAVEN_BEAD_Y_INACTIVE };

        // Move Heaven
        if (self.heaven_y - target_h).abs() > 1.0 {
            let dir = (target_h - self.heaven_y).signum();
            self.heaven_y += dir * speed * dt;
            if (target_h - self.heaven_y).signum() != dir {
                self.heaven_y = target_h; // Snap
            }
        } else {
            self.heaven_y = target_h;
        }

        // Move Earth
        for i in 0..4 {
            let target_e = if i < self.earth_active_count as usize {
                EARTH_BEAD_Y_ACTIVE_BASE + (i as f32 * BEAD_HEIGHT)
            } else {
                EARTH_BEAD_Y_INACTIVE_BASE + (i as f32 * BEAD_HEIGHT)
            };

            if (self.earth_ys[i] - target_e).abs() > 1.0 {
                let dir = (target_e - self.earth_ys[i]).signum();
                self.earth_ys[i] += dir * speed * dt;
                if (target_e - self.earth_ys[i]).signum() != dir {
                    self.earth_ys[i] = target_e;
                }
            } else {
                self.earth_ys[i] = target_e;
            }
        }
    }
}

struct VisualSoroban {
    columns: Vec<VisualColumn>,
    move_queue: VecDeque<BeadMove>,
    move_timer: f32,
    move_interval: f32,
    x: f32,
    y: f32,
    label: String,
}

impl VisualSoroban {
    fn new(soroban: &Soroban, x: f32, y: f32, label: String) -> Self {
        Self {
            columns: soroban.columns.iter().map(VisualColumn::new).collect(),
            move_queue: VecDeque::new(),
            move_timer: 0.0,
            move_interval: 0.1, // 100ms per move step
            x,
            y,
            label,
        }
    }

    fn push_moves(&mut self, moves: Vec<BeadMove>) {
        for m in moves {
            self.move_queue.push_back(m);
        }
    }

    fn update(&mut self, dt: f32) {
        // Process queue
        if !self.move_queue.is_empty() {
            self.move_timer -= dt;
            if self.move_timer <= 0.0 {
                self.move_timer = self.move_interval;
                let m = self.move_queue.pop_front().unwrap();
                self.apply_move(m);
            }
        }

        // Update animations
        for col in &mut self.columns {
            col.update(dt);
        }
    }

    fn apply_move(&mut self, m: BeadMove) {
        if m.column_idx >= self.columns.len() { return; }
        let col = &mut self.columns[m.column_idx];

        match m.bead_type {
            BeadType::Heaven => {
                match m.action {
                    Action::Activate => col.heaven_active = true,
                    Action::Deactivate => col.heaven_active = false,
                }
            }
            BeadType::Earth => {
                match m.action {
                    Action::Activate => {
                        // Activate `amount` beads.
                        // Means increase count by amount.
                        // Beads `current` to `current + amount - 1` move up.
                        col.earth_active_count += m.amount;
                    }
                    Action::Deactivate => {
                        // Deactivate `amount` beads.
                        // Means decrease count.
                        if col.earth_active_count >= m.amount {
                            col.earth_active_count -= m.amount;
                        }
                    }
                }
            }
        }
    }

    fn draw(&self) {
        draw_text(&self.label, self.x, self.y - 10.0, 20.0, WHITE);

        let width = self.columns.len() as f32 * COLUMN_WIDTH + FRAME_PADDING * 2.0;
        let height = 200.0;

        // Frame
        draw_rectangle_lines(self.x, self.y, width, height, 2.0, BROWN);
        // Beam
        draw_line(self.x, self.y + BEAM_Y_OFFSET, self.x + width, self.y + BEAM_Y_OFFSET, 4.0, BROWN);

        for (i, col) in self.columns.iter().enumerate() {
            let cx = self.x + FRAME_PADDING + i as f32 * COLUMN_WIDTH + COLUMN_WIDTH / 2.0;

            // Rod
            draw_line(cx, self.y + 10.0, cx, self.y + height - 10.0, 2.0, GRAY);

            // Heaven Bead
            let hy = self.y + col.heaven_y;
            draw_ellipse(cx, hy, BEAD_WIDTH / 2.0, BEAD_HEIGHT / 2.0, 0.0, RED);

            // Earth Beads
            for j in 0..4 {
                let ey = self.y + col.earth_ys[j];
                draw_ellipse(cx, ey, BEAD_WIDTH / 2.0, BEAD_HEIGHT / 2.0, 0.0, YELLOW);
            }
        }
    }
}

// Custom ellipse drawer since macroquad doesn't have draw_ellipse?
// Wait, macroquad has `draw_ellipse`?
// No, it has `draw_circle`, `draw_poly`.
// I can implement `draw_ellipse` using `draw_poly` or just use scaled circles.
fn draw_ellipse(x: f32, y: f32, rx: f32, ry: f32, _rotation: f32, color: Color) {
    // Basic approximation with circle for now if needed, or scaled.
    // Actually, let's just draw rectangles or circles.
    // Standard abacus beads are diamond/bicone shape.
    // Let's use `draw_poly` with 4 sides (Diamond).
    draw_poly(x, y, 4, rx, 0.0, color);
    // rx is radius. ry is ignored if I use regular poly.
    // If I want real ellipse, I need `draw_texture_ex` with scale or generating mesh.
    // Diamond shape is fine for ancient feel.
}

#[macroquad::main("Soroban Market")]
async fn main() {
    let mut market = Market::new();

    // Create visualizers
    // Screen size roughly 800x600 default?

    let mut v_bid = VisualSoroban::new(&market.best_bid, 50.0, 50.0, "Best Bid".to_string());
    let mut v_ask = VisualSoroban::new(&market.best_ask, 400.0, 50.0, "Best Ask".to_string());
    let mut v_last = VisualSoroban::new(&market.last_price, 50.0, 300.0, "Last Price".to_string());
    let mut v_volume = VisualSoroban::new(&market.volume, 400.0, 300.0, "Volume".to_string());

    loop {
        let dt = get_frame_time();

        // Update market logic
        // We only update market if queues are empty to avoid desync?
        // Or just let it run.
        let events = market.update();

        for event in events {
            match event {
                MarketEvent::AbacusUpdate(id, moves) => {
                    match id {
                        AbacusId::BestBid => v_bid.push_moves(moves),
                        AbacusId::BestAsk => v_ask.push_moves(moves),
                        AbacusId::LastPrice => v_last.push_moves(moves),
                        AbacusId::Volume => v_volume.push_moves(moves),
                    }
                }
                MarketEvent::Trade(price, size) => {
                    // Flash effect?
                    // Maybe update LastPrice manually if it wasn't an AbacusUpdate?
                    // But our market.update calls set_value for LastPrice.
                    // If set_value is used, moves are empty.
                    // We need to sync visual state if set_value was used!
                    // VisualSoroban doesn't handle set_value (instant jump).
                    // We should detect if logical state diverged?
                    // Or Market should emit "Reset" event?
                    // For now, let's just rely on AbacusUpdate for Volume.
                    // For Prices, we are not animating them yet (instant set).
                    // So we need to sync them manually.
                }
                _ => {}
            }
        }

        // Sync instant updates (Prices)
        // Check if v_bid matches market.best_bid
        // This is a hack because we don't have events for set_value.
        // We can just re-init columns if no moves are playing.
        if v_bid.move_queue.is_empty() {
             // Re-sync logical state
             for (i, col) in market.best_bid.columns.iter().enumerate() {
                 v_bid.columns[i].heaven_active = col.heaven;
                 v_bid.columns[i].earth_active_count = col.earth;
             }
        }
        if v_ask.move_queue.is_empty() {
             for (i, col) in market.best_ask.columns.iter().enumerate() {
                 v_ask.columns[i].heaven_active = col.heaven;
                 v_ask.columns[i].earth_active_count = col.earth;
             }
        }
        if v_last.move_queue.is_empty() {
             for (i, col) in market.last_price.columns.iter().enumerate() {
                 v_last.columns[i].heaven_active = col.heaven;
                 v_last.columns[i].earth_active_count = col.earth;
             }
        }


        // Update visuals
        v_bid.update(dt);
        v_ask.update(dt);
        v_last.update(dt);
        v_volume.update(dt);

        clear_background(BLACK);

        v_bid.draw();
        v_ask.draw();
        v_last.draw();
        v_volume.draw();

        // Draw simple info
        draw_text("Soroban HFT Market", 10.0, 20.0, 30.0, LIGHTGRAY);

        next_frame().await
    }
}
