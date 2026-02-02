pub mod agent;
pub mod market;
pub mod model;
pub mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::market::{resolve_auction, Bid};
use crate::model::{AgentState, Core, MarketState, Strategy, ThreadAgent};
use crate::ui::draw_ui;

struct App {
    state: MarketState,
    agents: Vec<ThreadAgent>,
    cores: Vec<Core>,
    running: bool,
    next_agent_id: usize,
}

impl App {
    fn new() -> Self {
        let mut cores = Vec::new();
        for i in 0..4 {
            cores.push(Core {
                id: i,
                current_agent_id: None,
                utilization: 0.0,
            });
        }

        Self {
            state: MarketState::new(),
            agents: Vec::new(),
            cores,
            running: true,
            next_agent_id: 0,
        }
    }

    fn spawn_agents(&mut self) {
        let mut rng = rand::thread_rng();
        // Spawn 1-5 new agents every tick
        let count = rng.gen_range(1..5);

        let strategies = [
            Strategy::HighFreq,
            Strategy::Sniper,
            Strategy::Desperate,
            Strategy::Value,
        ];

        for _ in 0..count {
            let strategy = strategies[rng.gen_range(0..strategies.len())];
            let work = rng.gen_range(10.0..50.0);
            // Deadline must be at least work + buffer
            let buffer = rng.gen_range(1.1..3.0);
            let deadline = self.state.current_tick + (work * buffer) as u64;

            // Budget roughly proportional to work * expected price (1.0)
            let budget = work * rng.gen_range(1.0..2.0) * 1.5;

            self.agents.push(ThreadAgent::new(
                self.next_agent_id,
                strategy,
                budget,
                work,
                deadline,
            ));
            self.next_agent_id += 1;
        }
    }

    fn update(&mut self) {
        self.state.current_tick += 1;

        // 1. Spawn new agents periodically
        if self.state.current_tick % 5 == 0 {
            self.spawn_agents();
        }

        // 2. Collect bids
        let mut bids = Vec::new();
        for agent in &self.agents {
            if agent.state != AgentState::Finished && agent.state != AgentState::Killed {
                if let Some(price) =
                    agent.decide_bid(self.state.current_price, self.state.current_tick)
                {
                    bids.push(Bid {
                        agent_id: agent.id,
                        price,
                    });
                }
            }
        }

        // 3. Resolve Auction
        resolve_auction(&mut self.agents, &mut self.cores, &mut self.state, bids);

        // 4. Update Agent States (Check deadlines, finish work)
        let current_tick = self.state.current_tick;
        for agent in &mut self.agents {
            if agent.state == AgentState::Finished || agent.state == AgentState::Killed {
                continue;
            }

            if agent.work_remaining <= 0.0 {
                agent.state = AgentState::Finished;
                self.state.finished_count += 1;
            } else if agent.deadline <= current_tick {
                agent.state = AgentState::Killed;
                self.state.killed_count += 1;
            } else if agent.credits <= 0.0 {
                // Bankrupt but maybe still alive? Let's kill them for now to save simulation slots.
                agent.state = AgentState::Killed;
                self.state.killed_count += 1;
            }
        }

        // 5. Cleanup old agents to keep memory low
        // Only remove finished/killed agents if we have too many
        if self.agents.len() > 1000 {
            self.agents.retain(|a| match a.state {
                AgentState::Finished | AgentState::Killed => false,
                _ => true,
            });
        }

        self.state.active_thread_count = self
            .agents
            .iter()
            .filter(|a| a.state == AgentState::Active || a.state == AgentState::Running)
            .count();
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    // Seed initial agents
    app.spawn_agents();
    for _ in 0..10 {
        app.spawn_agents();
    }

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal
            .draw(|f| draw_ui(f, &app.state, &app.agents, &app.cores))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
