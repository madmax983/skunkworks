use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use market_sim::{Grid as MarketGrid, Particle};
use platter::Platter;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct MarketPlatterApp {
    market: MarketGrid,
    platter: Platter,
    width: usize,
    height: usize,
}

impl MarketPlatterApp {
    fn new(width: usize, height: usize) -> Self {
        let mut market = MarketGrid::new(width, height);
        // Initial market state
        for _ in 0..10 {
            market.set(
                rand::random::<usize>() % width,
                rand::random::<usize>() % (height / 2),
                Particle::Ask(rand::random()),
            );
            market.set(
                rand::random::<usize>() % width,
                height - 1 - rand::random::<usize>() % (height / 2),
                Particle::Bid(rand::random()),
            );
        }

        Self {
            market,
            platter: Platter::new(width, height),
            width,
            height,
        }
    }

    fn tick(&mut self) {
        // Step market simulation
        let _events = self.market.update();

        // Feed new market orders randomly
        if rand::random::<f32>() < 0.2 {
            self.market.set(
                rand::random::<usize>() % self.width,
                0,
                Particle::Ask(rand::random()),
            );
        }
        if rand::random::<f32>() < 0.2 {
            self.market.set(
                rand::random::<usize>() % self.width,
                self.height - 1,
                Particle::Bid(rand::random()),
            );
        }

        // Map Market to Platter Heat
        for y in 0..self.height {
            for x in 0..self.width {
                match self.market.get(x, y) {
                    Particle::Ask(_) => self.platter.accumulate(x, y, 0.1),
                    Particle::Bid(_) => self.platter.accumulate(x, y, 0.1),
                    Particle::Trade { .. } => self.platter.accumulate(x, y, 2.0),
                    _ => {}
                }
            }
        }

        // Decay heat
        self.platter.decay(0.95);
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let title = " 🧬 Splice: market-sim × platter (Thermodynamic Market Liquidity) ";
        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let heat = self.platter.get(x, y);
                        if heat > 0.1 {
                            let color = if heat > 1.5 {
                                Color::Red
                            } else if heat > 0.8 {
                                Color::LightRed
                            } else if heat > 0.4 {
                                Color::Yellow
                            } else {
                                Color::DarkGray
                            };
                            let render_x = (x as f64 / self.width as f64) * 100.0;
                            let render_y =
                                ((self.height - 1 - y) as f64 / self.height as f64) * 100.0;
                            ctx.print(
                                render_x,
                                render_y,
                                ratatui::text::Span::styled("█", Style::default().fg(color)),
                            );
                        }

                        // Overlay market particles
                        match self.market.get(x, y) {
                            Particle::Bid(_) => {
                                let render_x = (x as f64 / self.width as f64) * 100.0;
                                let render_y =
                                    ((self.height - 1 - y) as f64 / self.height as f64) * 100.0;
                                ctx.print(
                                    render_x,
                                    render_y,
                                    ratatui::text::Span::styled(
                                        "▲",
                                        Style::default().fg(Color::Green),
                                    ),
                                );
                            }
                            Particle::Ask(_) => {
                                let render_x = (x as f64 / self.width as f64) * 100.0;
                                let render_y =
                                    ((self.height - 1 - y) as f64 / self.height as f64) * 100.0;
                                ctx.print(
                                    render_x,
                                    render_y,
                                    ratatui::text::Span::styled(
                                        "▼",
                                        Style::default().fg(Color::Red),
                                    ),
                                );
                            }
                            Particle::Trade { .. } => {
                                let render_x = (x as f64 / self.width as f64) * 100.0;
                                let render_y =
                                    ((self.height - 1 - y) as f64 / self.height as f64) * 100.0;
                                ctx.print(
                                    render_x,
                                    render_y,
                                    ratatui::text::Span::styled(
                                        "💥",
                                        Style::default().fg(Color::Yellow),
                                    ),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        frame.render_widget(canvas, chunks[0]);

        let info = Paragraph::new(
            "Market liquidity directly translates into a thermodynamic scalar field. Trades create intense heat bursts.",
        )
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("🧬 market-platter running in headless mode for 10 ticks.");
        let mut app = MarketPlatterApp::new(100, 100);
        for _ in 0..10 {
            app.tick();
        }
        println!("Finished headless run.");
        return Ok(());
    }

    let mut tui = Tui::init()?;

    let mut app = MarketPlatterApp::new(100, 50);
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
                {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
