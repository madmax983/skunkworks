use std::{io, thread, time::Duration};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use rand::prelude::*;

use heap_market::{market::Market, model::*, ui::draw_ui};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut market = Market::new();
    let mut rng = rand::thread_rng();

    // Init Heap
    let total_blocks = 256;
    let mut blocks: Vec<MemoryBlock> = (0..total_blocks)
        .map(|i| MemoryBlock {
            id: i,
            size: 1,
            owner: None, // None = Free / System owned
        })
        .collect();

    // Init Agents
    let mut agents: Vec<Agent> = (1..21)
        .map(|i| Agent {
            id: AgentId(i),
            budget: rng.gen_range(500..5000),
            strategy: match i {
                1..=5 => Strategy::Saver,
                6..=15 => Strategy::Spender,
                _ => Strategy::Hoarder,
            },
        })
        .collect();

    let mut tick_counter = 0;
    let system_reserve_price = 50;

    loop {
        terminal.draw(|f| draw_ui(f, &market, &blocks))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        tick_counter += 1;

        // --- SIMULATION LOGIC ---

        let last_price = market.clearing_price().unwrap_or(100);

        // 1. System sells free blocks (Supply)
        // If there are free blocks, the system puts a few asks
        let free_blocks = blocks.iter().filter(|b| b.owner.is_none()).count();
        if free_blocks > 0 {
            market.add_order(Order {
                agent_id: AgentId(0), // System
                order_type: OrderType::Ask,
                price: system_reserve_price,
                quantity: std::cmp::min(5, free_blocks),
            });
        }

        // 2. Agents place orders
        for agent in &mut agents {
            // Determine current holdings
            let holdings = blocks.iter().filter(|b| b.owner == Some(agent.id)).count();

            // Simple logic:
            // Target holdings: Saver=5, Spender=20, Hoarder=50
            let target = match agent.strategy {
                Strategy::Saver => 5,
                Strategy::Spender => 20,
                Strategy::Hoarder => 50,
                Strategy::Whale => 200,
                Strategy::PanicSeller => 0,
            };

            // Noise
            let variance: i64 = rng.gen_range(-10..=10);
            let bid_price = (last_price as i64 + variance).max(1) as u64;

            if holdings < target && agent.budget > bid_price {
                // Buy
                market.add_order(Order {
                    agent_id: agent.id,
                    order_type: OrderType::Bid,
                    price: bid_price,
                    quantity: 1,
                });
            } else if holdings > target {
                // Sell
                market.add_order(Order {
                    agent_id: agent.id,
                    order_type: OrderType::Ask,
                    price: bid_price.max(1), // Sell slightly cheaper? Or same.
                    quantity: 1,
                });
            }
        }

        // Whale Event
        if tick_counter == 200 {
            agents.push(Agent {
                id: AgentId(99),
                budget: 1_000_000, // Rich
                strategy: Strategy::Whale,
            });
        }

        // Crash Event (Whale leaves or panic)
        if tick_counter == 400 {
            if let Some(whale) = agents.iter_mut().find(|a| a.id == AgentId(99)) {
                whale.strategy = Strategy::PanicSeller; // Dump everything
            }
        }


        // 3. Market Match
        let transactions = market.match_orders();

        // 4. Settlement
        for txn in transactions {
            // Find seller's block (or free block if seller is System(0))
            let block_idx = if txn.seller_id == AgentId(0) {
                 blocks.iter().position(|b| b.owner.is_none())
            } else {
                 blocks.iter().position(|b| b.owner == Some(txn.seller_id))
            };

            if let Some(idx) = block_idx {
                // Update Owner
                blocks[idx].owner = Some(txn.buyer_id);

                // Update Budgets
                // Buyer pays
                if let Some(buyer) = agents.iter_mut().find(|a| a.id == txn.buyer_id) {
                    buyer.budget = buyer.budget.saturating_sub(txn.price);
                }

                // Seller gets paid (if not System)
                if txn.seller_id != AgentId(0) {
                    if let Some(seller) = agents.iter_mut().find(|a| a.id == txn.seller_id) {
                         seller.budget += txn.price;
                    }
                }

                // If PanicSeller sold, maybe reset owner to None (burn) or System?
                // Currently just transfers to Buyer.
                // PanicSeller dumps to market, so Buyer takes it. Correct.
            }
        }

        // Decay/Rent? (Optional, skipping for now)

        // Slow down slightly for visual pacing
        thread::sleep(Duration::from_millis(10));
    }
}
