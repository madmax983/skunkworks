pub mod agent;
pub mod market;
pub mod model;
pub mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::market::resolve_market;
use crate::model::{Agent, MarketState, Strategy};
use crate::ui::draw_ui;

struct App {
    state: MarketState,
    agents: Vec<Agent>,
    running: bool,
    tick_count: u64,
}

impl App {
    fn new() -> Self {
        let width = 32;
        let height = 32;
        let mut agents = Vec::new();
        let strategies = [
            Strategy::Greedy,
            Strategy::Saver,
            Strategy::Hoarder,
            Strategy::Panic,
        ];

        for i in 0..100 {
            let strategy = strategies[i % strategies.len()];
            // Randomize budget slightly
            let budget = 1000.0 + (i as f64 * 10.0);
            let demand = 5 + (i % 10);
            agents.push(Agent::new(i, strategy, budget, demand));
        }

        Self {
            state: MarketState::new(width, height),
            agents,
            running: true,
            tick_count: 0,
        }
    }

    fn update(&mut self) {
        self.tick_count += 1;
        // 1. Agents decide bids
        let mut all_bids = Vec::new();
        let current_price = self.state.current_price;

        for agent in &mut self.agents {
            agent.update_budget();
            let bids = agent.decide_bids(current_price);
            all_bids.extend(bids);
        }

        // 2. Market resolves
        resolve_market(&mut self.agents, &mut self.state, all_bids);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    // Run simulation tick every 100ms
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal
            .draw(|f| draw_ui(f, &app.state, &app.agents, app.tick_count))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && let KeyCode::Char('q') = key.code
        {
            app.running = false;
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
