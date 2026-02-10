use market_sim::{Grid, Particle};
use macroquad::prelude::*;
use macroquad::color::hsl_to_rgb;
use ::rand::Rng;

// Constants
const HEAP_SIZE: usize = 32;
const MARKET_HEIGHT: usize = 20;
const CELL_SIZE: f32 = 20.0;
const MARGIN: f32 = 20.0;

#[derive(Debug, Clone)]
struct Agent {
    id: usize,
    budget: f32,
    desired_pages: Vec<usize>,
    owned_pages: Vec<usize>,
    color: Color,
}

impl Agent {
    fn new(id: usize, budget: f32) -> Self {
        // Generate distinct color using golden angle
        let hue = (id as f32 * 0.618033988749895) % 1.0;
        let color = hsl_to_rgb(hue, 0.8, 0.6);

        Self {
            id,
            budget,
            desired_pages: Vec::new(),
            owned_pages: Vec::new(),
            color,
        }
    }
}

struct HeapMarket {
    grid: Grid,
    ownership: Vec<Option<usize>>, // None = System owned. Agent IDs start at 1.
    agents: Vec<Agent>,
    time: u64,
    total_trades: usize,
}

impl HeapMarket {
    fn new() -> Self {
        Self {
            grid: Grid::new(HEAP_SIZE, MARKET_HEIGHT),
            ownership: vec![None; HEAP_SIZE],
            agents: Vec::new(),
            time: 0,
            total_trades: 0,
        }
    }

    fn add_agent(&mut self, budget: f32) -> usize {
        let id = self.agents.len() + 1; // IDs start at 1
        let agent = Agent::new(id, budget);
        self.agents.push(agent);
        id
    }

    fn tick(&mut self) {
        self.time += 1;
        let mut rng = ::rand::thread_rng();

        // 1. Agents: Income, Decay, Bidding
        for agent in &mut self.agents {
            agent.budget += 1.0; // Basic Income
            // Decay based on owned pages (Rent)
            agent.budget -= agent.owned_pages.len() as f32 * 0.5;

            // Bidding logic
            if agent.budget > 5.0 {
                // Bid for desired pages
                for &page_id in &agent.desired_pages {
                    if !agent.owned_pages.contains(&page_id) {
                         if matches!(self.grid.get(page_id, MARKET_HEIGHT - 1), Particle::Empty) {
                             self.grid.set(page_id, MARKET_HEIGHT - 1, Particle::Bid(agent.id));
                        }
                    }
                }

                // Randomly desire new pages if rich
                if agent.budget > 100.0 && rng.gen_bool(0.05) {
                    let p = rng.gen_range(0..HEAP_SIZE);
                    if !agent.desired_pages.contains(&p) {
                        agent.desired_pages.push(p);
                    }
                }
            }

            // Bankruptcy / Eviction
            if agent.budget < 0.0 {
                // Drop all pages
                 for &p in &agent.owned_pages {
                     self.ownership[p] = None; // Reset to System
                 }
                 agent.owned_pages.clear();
                 // Reset budget
                 agent.budget = 10.0;
            }
        }

        // 2. Owners (Asks)
        for col in 0..HEAP_SIZE {
            let owner_id = self.ownership[col].unwrap_or(0); // 0 = System

            // Only place Ask if top is empty
             if matches!(self.grid.get(col, 0), Particle::Empty) {
                 self.grid.set(col, 0, Particle::Ask(owner_id));
             }
        }

        // 3. Update Market Simulation
        let events = self.grid.update();
        self.total_trades += events.len();

        // 4. Process Trades
        let mut budget_changes = vec![0.0; self.agents.len()];
        let mut page_adds = vec![Vec::new(); self.agents.len()];
        let mut page_removes = vec![Vec::new(); self.agents.len()];
        let mut ownership_updates = Vec::new();

        for event in events {
            let page_id = event.x;
            let new_owner = event.buyer;
            let old_owner = event.seller;
            let price = event.price;

            if new_owner > 0 {
                ownership_updates.push((page_id, Some(new_owner)));
            }

            if new_owner > 0 {
                let idx = new_owner - 1;
                if idx < self.agents.len() {
                    budget_changes[idx] -= price;
                    page_adds[idx].push(page_id);
                }
            }

            if old_owner > 0 {
                let idx = old_owner - 1;
                if idx < self.agents.len() {
                    budget_changes[idx] += price;
                    page_removes[idx].push(page_id);
                }
            }
        }

        // Apply Ownership Updates
        for (page, owner) in ownership_updates {
            self.ownership[page] = owner;
        }

        // Apply changes
        for (i, agent) in self.agents.iter_mut().enumerate() {
            agent.budget += budget_changes[i];

            for p in &page_adds[i] {
                if !agent.owned_pages.contains(p) {
                    agent.owned_pages.push(*p);
                }
            }

            for p in &page_removes[i] {
                if let Some(pos) = agent.owned_pages.iter().position(|x| x == p) {
                    agent.owned_pages.swap_remove(pos);
                }
            }
        }
    }
}

#[macroquad::main("Heap Market")]
async fn main() {
    let mut market = HeapMarket::new();
    let mut rng = ::rand::thread_rng();

    // Initial Population
    for _ in 0..5 {
        let id = market.add_agent(100.0);
        // Assign random desired pages
        for _ in 0..5 {
            market.agents[id-1].desired_pages.push(rng.gen_range(0..HEAP_SIZE));
        }
    }

    let mut paused = false;

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if !paused {
            market.tick();
        }

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // Draw Market Grid
        for y in 0..MARKET_HEIGHT {
            for x in 0..HEAP_SIZE {
                let particle = market.grid.get(x, y);
                let color = match particle {
                    Particle::Empty => BLACK,
                    Particle::Bid(owner) => {
                         if let Some(agent) = market.agents.iter().find(|a| a.id == owner) {
                             agent.color
                         } else {
                             GREEN
                         }
                    },
                    Particle::Ask(owner) => {
                         if owner == 0 {
                             RED
                         } else if let Some(agent) = market.agents.iter().find(|a| a.id == owner) {
                             // Darker version of agent color?
                             Color::new(agent.color.r * 0.5, agent.color.g * 0.5, agent.color.b * 0.5, 1.0)
                         } else {
                             RED
                         }
                    },
                    Particle::Trade { age } => {
                        let alpha = age as f32 / 5.0;
                        Color::new(1.0, 1.0, 1.0, alpha)
                    }
                };

                draw_rectangle(
                    MARGIN + x as f32 * CELL_SIZE,
                    MARGIN + y as f32 * CELL_SIZE,
                    CELL_SIZE - 1.0,
                    CELL_SIZE - 1.0,
                    color
                );
            }
        }

        // Draw Heap State (Bottom)
        let heap_y = MARGIN + MARKET_HEIGHT as f32 * CELL_SIZE + 20.0;
        draw_text("HEAP STATE (Owned Pages)", MARGIN, heap_y - 5.0, 20.0, WHITE);

        for x in 0..HEAP_SIZE {
            let owner = market.ownership[x];
            let color = match owner {
                Some(id) => {
                     if let Some(agent) = market.agents.iter().find(|a| a.id == id) {
                         agent.color
                     } else {
                         GRAY
                     }
                },
                None => DARKGRAY,
            };

            draw_rectangle(
                MARGIN + x as f32 * CELL_SIZE,
                heap_y,
                CELL_SIZE - 1.0,
                CELL_SIZE * 2.0,
                color
            );
             draw_rectangle_lines(
                MARGIN + x as f32 * CELL_SIZE,
                heap_y,
                CELL_SIZE - 1.0,
                CELL_SIZE * 2.0,
                2.0,
                BLACK
            );
        }

        // HUD
        let hud_y = heap_y + CELL_SIZE * 3.0;
        draw_text(&format!("Time: {}", market.time), MARGIN, hud_y, 20.0, WHITE);
        draw_text(&format!("Total Trades: {}", market.total_trades), MARGIN, hud_y + 20.0, 20.0, WHITE);
        draw_text("Press SPACE to Pause", MARGIN, hud_y + 40.0, 20.0, WHITE);

        // Agent List
        let mut agent_y = hud_y + 70.0;
        for agent in &market.agents {
            draw_rectangle(MARGIN, agent_y - 15.0, 15.0, 15.0, agent.color);
            draw_text(
                &format!("Agent {}: ${:.1} | Owned: {}", agent.id, agent.budget, agent.owned_pages.len()),
                MARGIN + 20.0,
                agent_y,
                20.0,
                WHITE
            );
            agent_y += 20.0;
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_auction() {
        let mut market = HeapMarket::new();
        let agent_id = market.add_agent(100.0);

        // Agent wants page 0
        market.agents[0].desired_pages.push(0);

        // Run ticks until trade happens.
        for _ in 0..20 {
            market.tick();
            if market.ownership[0] == Some(agent_id) {
                break;
            }
        }

        assert_eq!(market.ownership[0], Some(agent_id));
    }
}
