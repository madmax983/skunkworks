//! Thermodynamic Market Liquidity.
//!
//! # Lineage
//! - **Parent A:** `platter` (Topological Scalar Field Morphogenesis).
//! - **Parent B:** `market-sim` (Continuous Double Auction dynamics).
//! - **Novel Trait:** The discrete financial order book grid successfully maps into the continuous scalar heat field. Bids and asks generate low heat, while trades burst with massive kinetic thermal energy.
//!
//! # Execution
//! Run the simulation locally:
//! ```bash
//! cargo run -p platter-market
//! ```
//! Run headless (CI bypass):
//! ```bash
//! cargo run -p platter-market -- --headless
//! ```

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::text::{Line, Span};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, stdout};
use std::time::{Duration, Instant};

use market_sim::{Grid, Particle};
use platter::Platter;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    if headless {
        let mut sim = MarketSim::new(60, 30);
        for _ in 0..100 {
            sim.update();
        }
        println!("Headless execution successful.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sim = MarketSim::new(100, 40);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(10),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let header = Paragraph::new(format!(
                "Thermodynamic Market Liquidity | Volume: {} | Bids: {} | Asks: {} | Press 'q' to quit",
                sim.market.trade_count, sim.market.total_bids, sim.market.total_asks
            ))
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("platter-market"));
            f.render_widget(header, chunks[0]);

            let mut lines = Vec::new();
            for y in 0..sim.height {
                let mut spans = Vec::new();
                for x in 0..sim.width {
                    let particle = sim.market.get(x, y);
                    let heat = sim.platter.get(x, y);

                    let symbol = match particle {
                        Particle::Bid(_) => "B",
                        Particle::Ask(_) => "A",
                        Particle::Trade{ .. } => "T",
                        Particle::Empty => " ",
                        Particle::Wall => "#",
                    };

                    let color = if heat > 2.0 {
                        Color::Red
                    } else if heat > 1.0 {
                        Color::Yellow
                    } else if heat > 0.5 {
                        Color::Green
                    } else if heat > 0.1 {
                        Color::Blue
                    } else {
                        Color::DarkGray
                    };

                    spans.push(Span::styled(symbol, Style::default().fg(color)));
                }
                lines.push(Line::from(spans));
            }
            let canvas = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
            f.render_widget(canvas, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sim.update();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

struct MarketSim {
    width: usize,
    height: usize,
    market: Grid,
    platter: Platter,
}

impl MarketSim {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            market: Grid::new(width, height),
            platter: Platter::new(width, height),
        }
    }

    fn update(&mut self) {
        let trades = self.market.update();

        // Decay heat
        self.platter.decay(0.95);

        // Map bids and asks to low heat
        for y in 0..self.height {
            for x in 0..self.width {
                match self.market.get(x, y) {
                    Particle::Bid(_) => self.platter.accumulate(x, y, 0.1),
                    Particle::Ask(_) => self.platter.accumulate(x, y, 0.1),
                    _ => {}
                }
            }
        }

        // Trades generate massive burst of heat
        for _trade in trades {
            let rx = _trade.x;
            let ry = _trade.y;
            self.platter.accumulate(rx, ry, 5.0);

            // spread it out
            let radius = 2;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = rx as isize + dx;
                    let ny = ry as isize + dy;
                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        let dist = (dx * dx + dy * dy) as f64;
                        if dist > 0.0 {
                            self.platter
                                .accumulate(nx as usize, ny as usize, 2.0 / dist);
                        }
                    }
                }
            }
        }
    }
}
