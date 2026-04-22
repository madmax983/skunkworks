use ::rand::seq::SliceRandom;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

const GRID_W: usize = 64;
const GRID_H: usize = 64;
const TARGET_UTILIZATION: f32 = 0.70;
const PRICE_SENSITIVITY: f32 = 0.02;
const BASE_INCOME_PER_BLOCK: f32 = 1.0;
const INITIAL_WEALTH: f32 = 50.0;
const FRAGMENTATION_TIME: u32 = 20;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Free,
    Allocated(usize), // Owner PID
    Fragmented(u32),  // Timer until free
}

struct Agent {
    id: usize,
    color: Color,
    wealth: f32,
    holdings: Vec<(usize, usize)>,
    efficiency: f32,
}

struct Market {
    grid: [[CellState; GRID_H]; GRID_W],
    agents: Vec<Agent>,
    price: f32,
    history: Vec<f32>,
    next_pid: usize,
    utilization_history: Vec<f32>,
}

impl Market {
    fn new() -> Self {
        Self {
            grid: [[CellState::Free; GRID_H]; GRID_W],
            agents: Vec::new(),
            price: 1.0,
            history: Vec::new(),
            next_pid: 0,
            utilization_history: Vec::new(),
        }
    }

    fn spawn_agent(&mut self) {
        let mut rng = thread_rng();
        // Try to find a free spot
        for _ in 0..50 {
            let x = rng.gen_range(0..GRID_W);
            let y = rng.gen_range(0..GRID_H);
            if self.grid[x][y] == CellState::Free {
                let id = self.next_pid;
                self.next_pid += 1;

                let color = Color::new(
                    rng.gen_range(0.4..1.0),
                    rng.gen_range(0.4..1.0),
                    rng.gen_range(0.4..1.0),
                    1.0,
                );

                let mut agent = Agent {
                    id,
                    color,
                    wealth: INITIAL_WEALTH,
                    holdings: Vec::new(),
                    efficiency: rng.gen_range(0.8..1.5),
                };

                // Initial purchase
                agent.holdings.push((x, y));
                self.grid[x][y] = CellState::Allocated(id);
                self.agents.push(agent);
                break;
            }
        }
    }

    fn update(&mut self) {
        let total_cells = (GRID_W * GRID_H) as f32;
        let mut allocated_count = 0;

        // 1. Grid Maintenance (Fragmentation Decay) & Counting
        for x in 0..GRID_W {
            for y in 0..GRID_H {
                match self.grid[x][y] {
                    CellState::Allocated(_) => allocated_count += 1,
                    CellState::Fragmented(mut timer) => {
                        if timer > 0 {
                            timer -= 1;
                            self.grid[x][y] = CellState::Fragmented(timer);
                        } else {
                            self.grid[x][y] = CellState::Free;
                        }
                    }
                    CellState::Free => {}
                }
            }
        }

        // 2. Market Macro-Economics
        let utilization = allocated_count as f32 / total_cells;
        self.utilization_history.push(utilization);
        if self.utilization_history.len() > 300 {
            self.utilization_history.remove(0);
        }

        // Price Dynamics
        let delta = utilization - TARGET_UTILIZATION;
        self.price *= 1.0 + (delta * PRICE_SENSITIVITY);
        if self.price < 0.1 {
            self.price = 0.1;
        }

        self.history.push(self.price);
        if self.history.len() > 300 {
            self.history.remove(0);
        }

        // 3. Agent Decisions
        // We split borrows here to avoid borrow checker issues.
        // `grid` and `agents` are disjoint fields.
        let grid = &mut self.grid;
        let current_price = self.price;
        let mut dead_agent_ids = Vec::new();

        let mut rng = thread_rng();

        for agent in self.agents.iter_mut() {
            let holdings_count = agent.holdings.len() as f32;

            // Income
            let income = holdings_count * BASE_INCOME_PER_BLOCK * agent.efficiency;
            agent.wealth += income;

            // Rent
            let rent = holdings_count * current_price;
            agent.wealth -= rent;

            if agent.wealth < 0.0 {
                dead_agent_ids.push(agent.id);
                continue; // Agent is dead, skip actions
            }

            // EXPANSION
            // If wealthy, try to buy adjacent
            let expansion_threshold = rent * 2.0 + 50.0;
            if agent.wealth > expansion_threshold {
                if let Some(&(hx, hy)) = agent.holdings.choose(&mut rng) {
                    let neighbors: [(usize, usize); 4] = [
                        (hx.wrapping_sub(1), hy),
                        (hx + 1, hy),
                        (hx, hy.wrapping_sub(1)),
                        (hx, hy + 1),
                    ];

                    for &(nx, ny) in neighbors.iter() {
                        if nx < GRID_W && ny < GRID_H && grid[nx][ny] == CellState::Free {
                            let cost = current_price * 1.5; // Buying fee
                            if agent.wealth > cost + expansion_threshold {
                                agent.wealth -= cost;
                                grid[nx][ny] = CellState::Allocated(agent.id);
                                agent.holdings.push((nx, ny));
                                break; // Buy one per tick max
                            }
                        }
                    }
                }
            }

            // CONTRACTION
            // If poor, sell random holding
            let panic_threshold = rent * 1.5;
            if agent.wealth < panic_threshold && agent.holdings.len() > 1 {
                let remove_idx = rng.gen_range(0..agent.holdings.len());
                let (rx, ry) = agent.holdings.remove(remove_idx);
                // "Selling" just releases it to Free immediately (or Fragmented?)
                // Let's say selling is orderly, so it becomes Free.
                grid[rx][ry] = CellState::Free;
                // No refund, just stop paying rent.
            }
        }

        // 4. Cleanup Dead Agents
        if !dead_agent_ids.is_empty() {
            // Remove agents
            // We need to move agents out or retain.
            // Since we need to access `grid` to mark fragmentation based on the dead agent's holdings,
            // we first find the holdings of dead agents.

            // Actually, we can just iterate again or do it in the loop?
            // In the loop above, we didn't remove holdings because we were iterating.

            let mut i = 0;
            while i < self.agents.len() {
                if self.agents[i].wealth < 0.0 {
                    let agent = self.agents.remove(i);
                    // Mark grid
                    for (x, y) in agent.holdings {
                        self.grid[x][y] = CellState::Fragmented(FRAGMENTATION_TIME);
                    }
                } else {
                    i += 1;
                }
            }
        }

        // 5. Spawning
        if self.agents.len() < 10 || (self.agents.len() < 100 && rng.gen_bool(0.05)) {
            self.spawn_agent();
        }
    }
}

#[macroquad::main("Malloc Market 📉")]
async fn main() {
    let mut market = Market::new();

    // Initial spawn
    for _ in 0..20 {
        market.spawn_agent();
    }

    loop {
        if is_key_pressed(KeyCode::R) {
            market = Market::new();
            for _ in 0..20 {
                market.spawn_agent();
            }
        }

        market.update();

        clear_background(BLACK);

        // Rendering setup
        let screen_w = screen_width();
        let screen_h = screen_height();

        let ui_height = 150.0;
        let grid_view_h = screen_h - ui_height;

        let cell_w = screen_w / GRID_W as f32;
        let cell_h = grid_view_h / GRID_H as f32;

        // Draw Grid
        for x in 0..GRID_W {
            for y in 0..GRID_H {
                let rect_x = x as f32 * cell_w;
                let rect_y = y as f32 * cell_h;

                let color = match market.grid[x][y] {
                    CellState::Free => BLACK,
                    CellState::Allocated(pid) => {
                        if let Some(agent) = market.agents.iter().find(|a| a.id == pid) {
                            agent.color
                        } else {
                            // Zombie cell? Should not happen.
                            DARKGRAY
                        }
                    }
                    CellState::Fragmented(timer) => {
                        let shade = (timer as f32 / FRAGMENTATION_TIME as f32) * 0.5;
                        Color::new(shade, shade, shade, 1.0)
                    }
                };

                if color != BLACK {
                    draw_rectangle(rect_x, rect_y, cell_w, cell_h, color);
                }
            }
        }

        // Draw UI
        let ui_y = grid_view_h;
        draw_rectangle(
            0.0,
            ui_y,
            screen_w,
            ui_height,
            Color::new(0.1, 0.1, 0.1, 1.0),
        );

        // Stats Text
        let text_color = WHITE;
        draw_text(
            &format!("Price: {:.2}", market.price),
            10.0,
            ui_y + 30.0,
            30.0,
            text_color,
        );
        draw_text(
            &format!("Agents: {}", market.agents.len()),
            10.0,
            ui_y + 60.0,
            30.0,
            text_color,
        );
        if let Some(util) = market.utilization_history.last() {
            draw_text(
                &format!("Util: {:.1}%", util * 100.0),
                10.0,
                ui_y + 90.0,
                30.0,
                text_color,
            );
        }

        // Charts
        let chart_x = 200.0;
        let chart_y = ui_y + 10.0;
        let chart_w = screen_w - chart_x - 10.0;
        let chart_h = ui_height - 20.0;

        draw_rectangle(chart_x, chart_y, chart_w, chart_h, BLACK);

        // Draw Price History (Green)
        if market.history.len() > 1 {
            let max_val = market
                .history
                .iter()
                .cloned()
                .fold(0.0 / 0.0, f32::max)
                .max(2.0);

            for i in 0..market.history.len() - 1 {
                let v1 = market.history[i];
                let v2 = market.history[i + 1];

                let x1 = chart_x + (i as f32 / market.history.len() as f32) * chart_w;
                let x2 = chart_x + ((i + 1) as f32 / market.history.len() as f32) * chart_w;

                let y1 = chart_y + chart_h - (v1 / max_val) * chart_h;
                let y2 = chart_y + chart_h - (v2 / max_val) * chart_h;

                draw_line(x1, y1, x2, y2, 2.0, GREEN);
            }
        }

        // Draw Utilization History (Blue)
        if market.utilization_history.len() > 1 {
            for i in 0..market.utilization_history.len() - 1 {
                let v1 = market.utilization_history[i];
                let v2 = market.utilization_history[i + 1];

                let x1 = chart_x + (i as f32 / market.utilization_history.len() as f32) * chart_w;
                let x2 =
                    chart_x + ((i + 1) as f32 / market.utilization_history.len() as f32) * chart_w;

                // Utilization is 0.0 to 1.0
                let y1 = chart_y + chart_h - v1 * chart_h;
                let y2 = chart_y + chart_h - v2 * chart_h;

                draw_line(x1, y1, x2, y2, 1.0, BLUE);
            }
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_update() {
        let mut market = Market::new();
        market.spawn_agent();

        let initial_wealth = market.agents[0].wealth;

        market.update();

        // Agent might survive or die, but initially it should survive.
        if !market.agents.is_empty() {
            let agent = &market.agents[0];
            assert_ne!(agent.wealth, initial_wealth);
        }
    }
}
