mod synth;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use market_sim::{Grid, Particle};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    widgets::{
        Block, Borders, Paragraph, Sparkline,
        canvas::{Canvas, Points},
    },
};
use std::time::Duration;
use synth::SynthState;
use tui_shared::Tui;

struct App {
    grid: Grid,
    synth: SynthState,
    should_quit: bool,
    market_price: f32, // The "External" price driving the market maker

    // Rendering buffers (Zero-allocation loop)
    bids_buf: Vec<(f64, f64)>,
    asks_buf: Vec<(f64, f64)>,
    trades_buf: Vec<(f64, f64)>,
    price_line_buf: Vec<(f64, f64)>,
}

impl App {
    fn new() -> Self {
        Self {
            grid: Grid::new(60, 40),
            synth: SynthState::new(),
            should_quit: false,
            market_price: 20.0, // Start in middle (height 40, middle 20)
            bids_buf: Vec::with_capacity(1000),
            asks_buf: Vec::with_capacity(1000),
            trades_buf: Vec::with_capacity(100),
            price_line_buf: Vec::with_capacity(60),
        }
    }

    fn update(&mut self) {
        // Market Maker Logic
        let mut rng = rand::thread_rng();

        // Random walk the "True Price"
        if rng.gen_bool(0.1) {
            self.market_price += rng.gen_range(-1.0..1.0);
            self.market_price = self.market_price.clamp(5.0, 35.0);
        }

        // Spawn Orders
        // Bids below price
        if rng.gen_bool(0.5) {
            let offset = rng.gen_range(1.0..15.0);
            let bid_y = (self.market_price + offset) as usize;
            let x = rng.gen_range(0..self.grid.width);
            if bid_y < self.grid.height {
                self.grid.set(x, bid_y, Particle::Bid(0));
            }
        }

        // Asks above price
        if rng.gen_bool(0.5) {
            let offset = rng.gen_range(1.0..15.0);
            let ask_y = (self.market_price - offset) as isize;
            let x = rng.gen_range(0..self.grid.width);
            if ask_y >= 0 {
                self.grid.set(x, ask_y as usize, Particle::Ask(0));
            }
        }

        // Whales
        if rng.gen_bool(0.02) {
            // Drop a block
            let is_bid = rng.gen_bool(0.5);
            let cx = rng.gen_range(10..self.grid.width - 10);
            // Bid whale at bottom, Ask whale at top
            let cy = if is_bid { self.grid.height - 8 } else { 5 };
            let p = if is_bid {
                Particle::Bid(0)
            } else {
                Particle::Ask(0)
            };
            for dy in 0..4 {
                for dx in 0..6 {
                    self.grid.set(cx + dx, cy + dy, p);
                }
            }
        }

        self.grid.update();
        self.synth
            .update(self.grid.center_of_mass, self.grid.trade_count);
    }
}

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),    // Market Grid
                    Constraint::Length(8), // Waveform
                    Constraint::Length(1), // Status
                ])
                .split(f.area());

            // Prepare Points for Rendering
            app.bids_buf.clear();
            app.asks_buf.clear();
            app.trades_buf.clear();
            app.price_line_buf.clear();

            for y in 0..app.grid.height {
                for x in 0..app.grid.width {
                    match app.grid.get(x, y) {
                        Particle::Bid(_) => {
                            app.bids_buf
                                .push((x as f64, (app.grid.height - y - 1) as f64));
                        }
                        Particle::Ask(_) => {
                            app.asks_buf
                                .push((x as f64, (app.grid.height - y - 1) as f64));
                        }
                        Particle::Trade { .. } => {
                            app.trades_buf
                                .push((x as f64, (app.grid.height - y - 1) as f64));
                        }
                        Particle::Empty => {}
                    }
                }
            }

            // True Price Line
            let price_y = (app.grid.height as f32 - app.market_price - 1.0) as f64;
            for x in 0..app.grid.width {
                if x % 2 == 0 {
                    app.price_line_buf.push((x as f64, price_y));
                }
            }

            // Render Grid
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Market Liquidity Flow (Green=Bid, Red=Ask) "),
                )
                .x_bounds([0.0, app.grid.width as f64])
                .y_bounds([0.0, app.grid.height as f64])
                .marker(Marker::Block)
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: &app.bids_buf,
                        color: Color::Green,
                    });
                    ctx.draw(&Points {
                        coords: &app.asks_buf,
                        color: Color::Red,
                    });
                    ctx.draw(&Points {
                        coords: &app.trades_buf,
                        color: Color::White,
                    });
                    ctx.draw(&Points {
                        coords: &app.price_line_buf,
                        color: Color::Blue,
                    });
                });
            f.render_widget(canvas, chunks[0]);

            // Render Waveform
            let waveform_width = chunks[1].width as usize;
            if waveform_width > 0 {
                let data = app.synth.get_waveform(waveform_width);
                let spark_data: Vec<u64> = data.iter().map(|v| ((v + 1.0) * 10.0) as u64).collect();

                let sparkline = Sparkline::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Market Synth Output "),
                    )
                    .data(&spark_data)
                    .style(Style::default().fg(Color::Cyan));
                f.render_widget(sparkline, chunks[1]);
            }

            // Status
            let status = Paragraph::new(format!(
                "Freq: {:.1}Hz | Noise: {:.2} | Trades: {} | Orders: {}",
                app.synth.price_frequency,
                app.synth.trade_intensity,
                app.grid.trade_count,
                app.grid.total_bids + app.grid.total_asks
            ))
            .style(Style::default().bg(Color::DarkGray));
            f.render_widget(status, chunks[2]);
        })?;

        // Handle Events
        if event::poll(Duration::from_millis(30))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('r') => {
                            // Reset grid but keep buffers
                            app.grid = Grid::new(60, 40);
                            app.synth = SynthState::new();
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }

        app.update();
    }

    Ok(())
}
