use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use rayon::prelude::*;

mod order_book;
use order_book::*;

const GRID_W: usize = 64;
const GRID_H: usize = 64;
// const TARGET_UTILIZATION: f32 = 0.70;
// const PRICE_SENSITIVITY: f32 = 0.02; // Deprecated: Emergent pricing
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
    // Strategy parameters
    urgency: f32, // How much they are willing to bid above market
}

struct Market {
    grid: [[CellState; GRID_H]; GRID_W],
    agents: Vec<Agent>,
    last_price: f32,
    order_book: OrderBook,
    history: Vec<f32>,
    next_pid: usize,
    utilization_history: Vec<f32>,
    last_tx_count: usize,
}

impl Market {
    fn new() -> Self {
        Self {
            grid: [[CellState::Free; GRID_H]; GRID_W],
            agents: Vec::new(),
            last_price: 1.0,
            order_book: OrderBook::new(),
            history: Vec::new(),
            next_pid: 0,
            utilization_history: Vec::new(),
            last_tx_count: 0,
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
                    urgency: rng.gen_range(0.9..1.5),
                };

                // Initial purchase (Gift/Genesis block)
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
        let mut free_cells = Vec::new();
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
                            free_cells.push((x, y));
                        }
                    }
                    CellState::Free => {
                        free_cells.push((x, y));
                    }
                }
            }
        }

        // 2. Stats
        let utilization = allocated_count as f32 / total_cells;
        self.utilization_history.push(utilization);
        if self.utilization_history.len() > 300 {
            self.utilization_history.remove(0);
        }

        // 3. Clear Order Book
        self.order_book.clear();

        // 4. System (Seller) places Asks
        // Supply Curve: Price increases with Utilization
        // If grid is full, price is infinite.
        // Base price = 1.0
        // Price = Base * (1 / (1 - Utilization)^2)
        // Or simpler: Linear? Exponential?
        let system_ask_price = if utilization < 0.99 {
            1.0 / (1.0 - utilization).powi(2)
        } else {
            100.0 // Cap
        };

        // The system offers all free cells at this price (or a range?)
        // Let's say the system places ONE large ask order for all free cells.
        // Agent ID for system is usize::MAX
        if !free_cells.is_empty() {
            self.order_book.add_order(usize::MAX, system_ask_price, free_cells.len(), OrderType::Ask);
        }

        // 5. Agents (Buyers) place Bids
        // Parallel decision making?
        // Collecting orders first.
        let last_price = self.last_price;
        let orders: Vec<(usize, f32, usize, OrderType)> = self.agents.par_iter().filter_map(|agent| {
            let _rng = thread_rng();

            // Income calculation is done later in serial update, but decision depends on current wealth
            let estimated_wealth = agent.wealth; // + income?

            if estimated_wealth < 0.0 {
                return None;
            }

            // Demand Strategy
            // If efficiency is high, expand.
            // Bid price depends on Urgency.
            let bid_price = last_price * agent.urgency;

            // Check budget
            if estimated_wealth > bid_price * 2.0 {
                // Place a bid for 1 block
                 Some((agent.id, bid_price, 1, OrderType::Bid))
            } else {
                None
            }
        }).collect();

        for (id, price, qty, otype) in orders {
            self.order_book.add_order(id, price, qty, otype);
        }

        // 6. Match Orders
        let transactions = self.order_book.match_orders();
        self.last_tx_count = transactions.len();

        // 7. Process Transactions
        if !transactions.is_empty() {
            let mut price_sum = 0.0;
            for tx in &transactions {
                price_sum += tx.price;

                // Seller Logic
                if tx.seller_id == usize::MAX {
                    // System sold a block
                    // Find a free block to give
                    // We need to pop from free_cells.
                    // But wait, free_cells was collected before match.
                    // We need to be careful not to double allocate if we had multiple asks (but we only had one big ask).

                    for _ in 0..tx.quantity {
                         if let Some((fx, fy)) = free_cells.pop() {
                             self.grid[fx][fy] = CellState::Allocated(tx.buyer_id);
                             // Update Buyer
                             if let Some(agent) = self.agents.iter_mut().find(|a| a.id == tx.buyer_id) {
                                 agent.holdings.push((fx, fy));
                                 agent.wealth -= tx.price;
                             }
                         }
                    }
                } else {
                    // Agent to Agent trade? (Not implemented yet, but scaffold is here)
                }
            }
            self.last_price = price_sum / transactions.len() as f32;
        } else {
            // Price decay if no trades? Or stick?
            // Let's decay slightly to encourage trading if stuck high
            self.last_price *= 0.99;
            if self.last_price < 0.1 { self.last_price = 0.1; }
        }

        self.history.push(self.last_price);
        if self.history.len() > 300 {
            self.history.remove(0);
        }

        // 8. Agent Updates (Income, Rent, Death)
        let mut dead_agent_ids = Vec::new();
        let current_price = self.last_price;

        for agent in self.agents.iter_mut() {
            let holdings_count = agent.holdings.len() as f32;

            // Income: Agents generate value by holding memory (simulation of work)
            let income = holdings_count * BASE_INCOME_PER_BLOCK * agent.efficiency;
            agent.wealth += income;

            // Rent: Agents must pay upkeep to the system?
            // In a pure purchase market, maybe no rent?
            // But if there is no rent, hoarding is free.
            // Let's implement a Property Tax / Maintenance Cost.
            let tax_rate = 0.05 * current_price;
            let tax = holdings_count * tax_rate;
            agent.wealth -= tax;

            // Panic Sell (Liquidation)
            if agent.wealth < 0.0 {
                dead_agent_ids.push(agent.id);
            } else if agent.wealth < tax * 5.0 && agent.holdings.len() > 1 {
                // Sell holding back to system (free it)
                // In a real market, they would place an Ask.
                // For simplicity, they "abandon" it.
                if let Some((rx, ry)) = agent.holdings.pop() {
                     self.grid[rx][ry] = CellState::Free; // Immediately free
                }
            }
        }

        // 9. Cleanup Dead Agents
        if !dead_agent_ids.is_empty() {
             let mut i = 0;
            while i < self.agents.len() {
                if dead_agent_ids.contains(&self.agents[i].id) {
                    let agent = self.agents.remove(i);
                    for (x, y) in agent.holdings {
                        self.grid[x][y] = CellState::Fragmented(FRAGMENTATION_TIME);
                    }
                } else {
                    i += 1;
                }
            }
        }

        // 10. Spawning
        if self.agents.len() < 10 || (self.agents.len() < 100 && thread_rng().gen_bool(0.05)) {
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
            &format!("Price: {:.2}", market.last_price),
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
        draw_text(
            &format!("TX/Tick: {}", market.last_tx_count),
            10.0,
            ui_y + 90.0,
            30.0,
            text_color,
        );
        if let Some(util) = market.utilization_history.last() {
            draw_text(
                &format!("Util: {:.1}%", util * 100.0),
                10.0,
                ui_y + 120.0,
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

        // Ensure agent has enough wealth to bid
        market.agents[0].wealth = 100.0;
        let initial_holdings = market.agents[0].holdings.len();

        market.update();

        // Check if market updated
        assert!(market.history.len() > 0);

        // Check if transaction happened (agent bought something)
        // Agent logic: if wealthy, bid.
        // Initial holdings: 1.
        // After update, if bought, holdings: 2.
        if market.agents[0].holdings.len() > initial_holdings {
            assert!(market.last_tx_count > 0);
        }
    }
}
