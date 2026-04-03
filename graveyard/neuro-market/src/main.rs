use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use market_sim::{Grid, Particle};
use neuro_sim::{Izhikevich, Network};
use rand::Rng;

const SIM_WIDTH: usize = 60;
const SIM_HEIGHT: usize = 40;
const NUM_NEURONS: usize = 100;

fn setup_network() -> Network {
    let mut net = Network::new();
    let mut rng = rand::thread_rng();

    // Create a mix of excitatory and inhibitory neurons
    for _ in 0..NUM_NEURONS {
        let idx = net.add_neuron();
        if rng.gen_bool(0.8) {
            net.neurons[idx] = Izhikevich::new_regular_spiking();
        } else {
            net.neurons[idx] = Izhikevich::new_fast_spiking();
        }
    }

    // Connect them randomly
    for i in 0..NUM_NEURONS {
        for j in 0..NUM_NEURONS {
            if i != j && rng.gen_bool(0.1) {
                let weight = if rng.gen_bool(0.8) {
                    rng.gen_range(2.0..10.0) // Excitatory
                } else {
                    rng.gen_range(-10.0..-2.0) // Inhibitory
                };
                net.add_synapse(i, j, weight);
            }
        }
    }

    net
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut market = Grid::new(SIM_WIDTH, SIM_HEIGHT);
    let mut network = setup_network();
    let mut rng = rand::thread_rng();

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    let mut inputs = vec![0.0; NUM_NEURONS];

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();

            inputs.fill(0.0);
            for input in inputs.iter_mut().take(NUM_NEURONS) {
                if rng.gen_bool(0.05) {
                    *input = rng.gen_range(10.0..20.0);
                }
            }

            network.step(&inputs);

            for (i, &spiked) in network.spikes.iter().enumerate().take(NUM_NEURONS) {
                if spiked {
                    let x = rng.gen_range(0..SIM_WIDTH);
                    let is_bid = i % 2 == 0;

                    if is_bid {
                        if matches!(market.get(x, SIM_HEIGHT - 1), Particle::Empty) {
                            market.set(x, SIM_HEIGHT - 1, Particle::Bid(i));
                        }
                    } else {
                        if matches!(market.get(x, 0), Particle::Empty) {
                            market.set(x, 0, Particle::Ask(i));
                        }
                    }
                }
            }

            let _ = market.update();

            terminal.draw(|f| {
                let size = f.area();

                let market_rect = Rect {
                    x: (size.width.saturating_sub(SIM_WIDTH as u16 * 2)) / 2,
                    y: (size.height.saturating_sub(SIM_HEIGHT as u16)) / 2,
                    width: SIM_WIDTH as u16 * 2 + 2,
                    height: SIM_HEIGHT as u16 + 2,
                };

                let block = Block::default()
                    .title(" Spiking Market Liquidity (neuro-market) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan));

                f.render_widget(block, market_rect);

                let inner = Rect {
                    x: market_rect.x + 1,
                    y: market_rect.y + 1,
                    width: market_rect.width.saturating_sub(2),
                    height: market_rect.height.saturating_sub(2),
                };

                let buf = f.buffer_mut();

                for y in 0..SIM_HEIGHT {
                    for x in 0..SIM_WIDTH {
                        let render_x = inner.x + (x as u16 * 2);
                        let render_y = inner.y + y as u16;

                        if render_x < inner.x + inner.width && render_y < inner.y + inner.height {
                            let particle = market.get(x, y);
                            let symbol = match particle {
                                Particle::Bid(_) => "▲",
                                Particle::Ask(_) => "▼",
                                Particle::Trade { .. } => "💥",
                                Particle::Wall => "█",
                                Particle::Empty => " ",
                            };
                            let color = match particle {
                                Particle::Bid(_) => Color::Green,
                                Particle::Ask(_) => Color::Red,
                                Particle::Trade { .. } => Color::Yellow,
                                Particle::Wall => Color::Gray,
                                Particle::Empty => Color::Reset,
                            };

                            if let Some(cell) = buf.cell_mut((render_x, render_y)) {
                                cell.set_symbol(symbol).set_fg(color);
                            }
                        }
                    }
                }
            })?;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
