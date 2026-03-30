// 🧬 Lineage Notes:
// This application crosses `neuro-sim` (Spiking Neural Network) with `market-sim` (Continuous Double Auction).
// The phenotype expressed here creates a "Spiking Market Liquidity", where market agents (Bids and Asks) are driven
// by the temporal pulse of Izhikevich neurons. Instead of a steady stream, liquidity arrives in rhythmic bursts
// simulating algorithmic trading clusters or market panics driven by integrate-and-fire dynamics.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use market_sim::{Grid, Particle as MarketParticle};
use neuro_sim::Network;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let mut market = Grid::new(40, 20);

    // Brain: We have 2 sensory neurons and 2 motor neurons
    let mut brain = Network::new();
    let n_bid = brain.add_neuron(); // Excites bids
    let n_ask = brain.add_neuron(); // Excites asks

    // Let's make them chattering/bursting to show off SNN traits
    brain.neurons[n_bid] = neuro_sim::Izhikevich::new_chattering();
    brain.neurons[n_ask] = neuro_sim::Izhikevich::new_chattering();

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    let mut rng = rand::thread_rng();

    // Some noise input
    let mut inputs = vec![0.0, 0.0];

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Neuro-Market "))
                .x_bounds([0.0, market.width as f64])
                .y_bounds([0.0, market.height as f64])
                .paint(|ctx| {
                    // Draw Market Grid
                    for y in 0..market.height {
                        for x in 0..market.width {
                            let cell = market.get(x, y);
                            match cell {
                                MarketParticle::Bid(_) => {
                                    ctx.print(x as f64, (market.height - 1 - y) as f64, Span::styled("B", Style::default().fg(Color::Green)));
                                }
                                MarketParticle::Ask(_) => {
                                    ctx.print(x as f64, (market.height - 1 - y) as f64, Span::styled("A", Style::default().fg(Color::Red)));
                                }
                                MarketParticle::Trade { age: _ } => {
                                    ctx.print(x as f64, (market.height - 1 - y) as f64, Span::styled("*", Style::default().fg(Color::Yellow)));
                                }
                                _ => {}
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let v_bid = brain.neurons[n_bid].v;
            let v_ask = brain.neurons[n_ask].v;

            let stats = Paragraph::new(vec![
                Line::from(format!(
                    "Bids/Asks: {}/{} | Trades: {} | [Space] Inject Current | [Q] Quit",
                    market.total_bids,
                    market.total_asks,
                    market.trade_count
                )),
                Line::from(format!(
                    "Neuron Bid V: {:.1} mV | Neuron Ask V: {:.1} mV",
                    v_bid, v_ask
                )),
            ])
            .block(Block::default().borders(Borders::ALL));

            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => {
                            // Inject random currents to stimulate market
                            inputs[0] += rng.gen_range(5.0..25.0);
                            inputs[1] += rng.gen_range(5.0..25.0);
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Decay inputs slightly
            inputs[0] *= 0.9;
            inputs[1] *= 0.9;

            // Step Brain
            brain.step(&inputs);

            // Did the bid neuron spike?
            if brain.is_spiking(n_bid) {
                // Spawn a burst of bids
                for _ in 0..5 {
                    let x = rng.gen_range(0..market.width);
                    let owner = rng.gen_range(1..100);
                    market.set(x, market.height - 1, MarketParticle::Bid(owner));
                }
            }

            // Did the ask neuron spike?
            if brain.is_spiking(n_ask) {
                // Spawn a burst of asks
                for _ in 0..5 {
                    let x = rng.gen_range(0..market.width);
                    let owner = rng.gen_range(1..100);
                    market.set(x, 0, MarketParticle::Ask(owner));
                }
            }

            // Step Market
            market.update();

            last_tick = Instant::now();
        }
    }
}
