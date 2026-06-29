use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use market_sim::{Grid as MarketGrid, Particle};
use neuro_sim::Network;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct NeuroMarketApp {
    market: MarketGrid,
    net: Network,
    width: usize,
    height: usize,
    iteration: u64,
}

impl NeuroMarketApp {
    fn new(width: usize, height: usize) -> Self {
        let mut net = Network::new();
        let mut rng = rand::thread_rng();

        // 10 Neurons
        let num_neurons = 10;
        for _ in 0..num_neurons {
            net.add_neuron();
        }

        // Randomly connect them
        for i in 0..num_neurons {
            let num_targets = rng.gen_range(2..5);
            for _ in 0..num_targets {
                let target = rng.gen_range(0..num_neurons);
                if i != target {
                    let weight = rng.gen_range(-5.0..15.0);
                    net.add_synapse(i, target, weight);
                }
            }
        }

        Self {
            market: MarketGrid::new(width, height),
            net,
            width,
            height,
            iteration: 0,
        }
    }

    fn tick(&mut self) {
        self.iteration += 1;
        let mut rng = rand::thread_rng();

        // Step neural network
        // Provide some random input to keep things lively
        let inputs: Vec<f32> = (0..self.net.neurons.len())
            .map(|_| if rng.gen_bool(0.1) { 25.0 } else { 0.0 })
            .collect();
        self.net.step(&inputs);

        // Map spikes to market actions
        for (i, &spiked) in self.net.spikes.iter().enumerate() {
            if spiked {
                let x = rng.gen_range(0..self.width);
                // First half of neurons buy (Bids), second half sell (Asks)
                if i < self.net.neurons.len() / 2 {
                    self.market
                        .set(x, self.height - 1, Particle::Bid(rng.gen_range(1..100)));
                } else {
                    self.market.set(x, 0, Particle::Ask(rng.gen_range(1..100)));
                }
            }
        }

        self.market.update();
    }
}

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode: exiting early to prevent CI timeouts.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 40;
    let height = 20;
    let mut app = NeuroMarketApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(f.area());

            // Left panel: The Market Simulation
            let market_block = Block::default()
                .borders(Borders::ALL)
                .title(" Market Sim (Bids & Asks) ");
            f.render_widget(market_block, chunks[0]);

            let inner_market = Rect::new(
                chunks[0].x + 1,
                chunks[0].y + 1,
                chunks[0].width.saturating_sub(2),
                chunks[0].height.saturating_sub(2),
            );

            for y in 0..app.height {
                for x in 0..app.width {
                    let screen_x = inner_market.x + x as u16;
                    let screen_y = inner_market.y + y as u16;

                    if screen_x < inner_market.right() && screen_y < inner_market.bottom() {
                        let particle = app.market.get(x, y);
                        match particle {
                            Particle::Bid(_) => {
                                let widget = Paragraph::new(Span::styled(
                                    "▲",
                                    Style::default().fg(Color::Green),
                                ));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Ask(_) => {
                                let widget = Paragraph::new(Span::styled(
                                    "▼",
                                    Style::default().fg(Color::Red),
                                ));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Trade { age } => {
                                let color = if age > 5 {
                                    Color::Yellow
                                } else {
                                    Color::DarkGray
                                };
                                let widget =
                                    Paragraph::new(Span::styled("✸", Style::default().fg(color)));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Empty => {
                                let widget = Paragraph::new(Span::styled(
                                    ".",
                                    Style::default().fg(Color::Rgb(30, 30, 30)),
                                ));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Wall => {}
                        }
                    }
                }
            }

            // Right panel: Neural Network State
            let neuro_block = Block::default()
                .borders(Borders::ALL)
                .title(" Cognitive Traders (SNN) ");
            f.render_widget(neuro_block.clone(), chunks[1]);

            let inner_neuro = Rect::new(
                chunks[1].x + 1,
                chunks[1].y + 1,
                chunks[1].width.saturating_sub(2),
                chunks[1].height.saturating_sub(2),
            );

            let mut neuro_text = vec![];
            for (i, spiked) in app.net.spikes.iter().enumerate() {
                let trader_type = if i < app.net.neurons.len() / 2 {
                    "Buyer (Bid)"
                } else {
                    "Seller (Ask)"
                };
                let status = if *spiked {
                    Span::styled("SPIKED (Trading!)", Style::default().fg(Color::Yellow))
                } else {
                    Span::styled("Resting", Style::default().fg(Color::DarkGray))
                };

                let v = app.net.neurons[i].v;

                neuro_text.push(Line::from(vec![
                    Span::raw(format!("Neuron {} [{}]: ", i, trader_type)),
                    status,
                    Span::raw(format!(" (v: {:.1})", v)),
                ]));
            }

            neuro_text.push(Line::from(""));
            neuro_text.push(Line::from(format!("Iteration: {}", app.iteration)));

            let neuro_widget = Paragraph::new(neuro_text);
            f.render_widget(neuro_widget, inner_neuro);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}
