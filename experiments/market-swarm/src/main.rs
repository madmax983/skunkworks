mod market;
mod synth;
mod flock;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        Block, Borders, Paragraph, Sparkline,
        canvas::{Canvas, Points},
    },
    Terminal,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use market::{Grid, Particle};
use synth::SynthState;
use flock::Flock;

struct App {
    grid: Grid,
    synth: SynthState,
    flock: Flock,
    should_quit: bool,
    market_price: f32,

    // Rendering buffers
    bids_buf: Vec<(f64, f64)>,
    asks_buf: Vec<(f64, f64)>,
    trades_buf: Vec<(f64, f64)>,
    price_line_buf: Vec<(f64, f64)>,
}

impl App {
    fn new() -> Self {
        let width = 60;
        let height = 40;
        Self {
            grid: Grid::new(width, height),
            synth: SynthState::new(),
            flock: Flock::new(width as f64, height as f64, 50),
            should_quit: false,
            market_price: 20.0,
            bids_buf: Vec::with_capacity(1000),
            asks_buf: Vec::with_capacity(1000),
            trades_buf: Vec::with_capacity(100),
            price_line_buf: Vec::with_capacity(60),
        }
    }

    fn update(&mut self) {
        // Market Maker Logic (from market-flow)
        let mut rng = rand::thread_rng();

        if rng.gen_bool(0.1) {
            self.market_price += rng.gen_range(-1.0..1.0);
            self.market_price = self.market_price.clamp(5.0, 35.0);
        }

        if rng.gen_bool(0.5) {
            let offset = rng.gen_range(1.0..15.0);
            let bid_y = (self.market_price + offset) as usize;
            let x = rng.gen_range(0..self.grid.width);
            if bid_y < self.grid.height {
                self.grid.set(x, bid_y, Particle::Bid);
            }
        }

        if rng.gen_bool(0.5) {
            let offset = rng.gen_range(1.0..15.0);
            let ask_y = (self.market_price - offset) as isize;
            let x = rng.gen_range(0..self.grid.width);
            if ask_y >= 0 {
                self.grid.set(x, ask_y as usize, Particle::Ask);
            }
        }

        if rng.gen_bool(0.02) {
            let is_bid = rng.gen_bool(0.5);
            let cx = rng.gen_range(10..self.grid.width - 10);
            let cy = if is_bid { self.grid.height - 8 } else { 5 };
            let p = if is_bid { Particle::Bid } else { Particle::Ask };
            for dy in 0..4 {
                for dx in 0..6 {
                    self.grid.set(cx + dx, cy + dy, p);
                }
            }
        }

        self.grid.update();
        self.synth.update(self.grid.center_of_mass, self.grid.trade_count);

        // Update Flock
        // Price Y in TUI coordinates = (height - price - 1)?
        // market-flow renders: (height - y - 1).
        // market_price is roughly center_of_mass (which is Y index).
        // So we pass center_of_mass directly as target Y.

        // Wait, market-flow render logic:
        // match particle { Bid => push(x, height - y - 1) }
        // So Grid Y=0 is Top. Render Y=0 is Bottom.
        // Boids use Render Y (0..height).
        // If Grid Price is `center_of_mass` (Y index), then Render Price is `height - center_of_mass`.

        let target_render_y = self.grid.height as f64 - self.grid.center_of_mass as f64;
        self.flock.update(target_render_y);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal);
    drop(tui);
    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('r') => {
                            app = App::new();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(8),
            Constraint::Length(1),
        ])
        .split(f.area());

    // Prepare Grid Points
    app.bids_buf.clear();
    app.asks_buf.clear();
    app.trades_buf.clear();
    app.price_line_buf.clear();

    for y in 0..app.grid.height {
        for x in 0..app.grid.width {
            match app.grid.get(x, y) {
                Particle::Bid => {
                    app.bids_buf.push((x as f64, (app.grid.height - y - 1) as f64));
                }
                Particle::Ask => {
                    app.asks_buf.push((x as f64, (app.grid.height - y - 1) as f64));
                }
                Particle::Trade { .. } => {
                    app.trades_buf.push((x as f64, (app.grid.height - y - 1) as f64));
                }
                Particle::Empty => {}
            }
        }
    }

    // Render Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Market Swarm "))
        .x_bounds([0.0, app.grid.width as f64])
        .y_bounds([0.0, app.grid.height as f64])
        .marker(Marker::Block)
        .paint(|ctx| {
            // Draw Grid
            ctx.draw(&Points {
                coords: &app.bids_buf,
                color: Color::DarkGray, // Dimmer background
            });
            ctx.draw(&Points {
                coords: &app.asks_buf,
                color: Color::DarkGray,
            });
            ctx.draw(&Points {
                coords: &app.trades_buf,
                color: Color::White,
            });

            // Draw Price Line
            // let price_y = app.grid.height as f64 - app.grid.center_of_mass as f64;
            // Draw simple line
            // ctx.print(0.0, price_y, Span::raw("---------- Price ----------"));

            // Draw Boids
            for boid in &app.flock.boids {
                let char_str = boid.dna.char_representation.to_string();
                let color = if boid.flash_timer > 0 {
                    Color::White
                } else {
                    boid.dna.color
                };

                ctx.print(
                    boid.position.x,
                    boid.position.y,
                    Span::styled(char_str, Style::default().fg(color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Waveform
    let waveform_width = chunks[1].width as usize;
    if waveform_width > 0 {
        let data = app.synth.get_waveform(waveform_width);
        let spark_data: Vec<u64> = data.iter().map(|v| ((v + 1.0) * 10.0) as u64).collect();

        let sparkline = Sparkline::default()
            .block(Block::default().borders(Borders::ALL).title(" Market Synth "))
            .data(&spark_data)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(sparkline, chunks[1]);
    }

    // Status
    let sync_idx = app.flock.synchronization_index();
    let status = Paragraph::new(format!(
        "Price: {:.2} | Sync: {:.3} | Bulls/Bears Swarming | 'q': Quit",
        app.market_price,
        sync_idx
    ))
    .style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(status, chunks[2]);
}
