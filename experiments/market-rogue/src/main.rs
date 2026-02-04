mod crawler;
mod mechanics;
mod simulation;

use anyhow::Result;
use crawler::{crawl, Node};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use mechanics::{Position, Trader};
use rand::{Rng, SeedableRng, rngs::StdRng};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
    Frame,
};
use simulation::{Grid, Particle};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct App {
    grid: Grid,
    trader: Trader, // Replaced Player
    commits: Vec<Node>,
    current_commit_idx: usize,
    market_price: f32,
    volatility: f64,
    should_quit: bool,
    rng: StdRng,
    message: String,
    game_over: bool,

    // Rendering buffers
    bids_buf: Vec<(f64, f64)>,
    asks_buf: Vec<(f64, f64)>,
    trades_buf: Vec<(f64, f64)>,
}

impl App {
    fn new() -> Result<Self> {
        let commits = crawl(".", 100).unwrap_or_else(|_| vec![]);
        let mut app = Self {
            grid: Grid::new(60, 40),
            trader: Trader::new(2.0, 20.0), // Initialize Trader
            commits,
            current_commit_idx: 0,
            market_price: 20.0,
            volatility: 0.1,
            should_quit: false,
            rng: StdRng::from_entropy(),
            message: "Welcome to Market Rogue. Arrow keys to move. 'S' to Short. Avoid Liquidation.".to_string(),
            game_over: false,
            bids_buf: Vec::with_capacity(1000),
            asks_buf: Vec::with_capacity(1000),
            trades_buf: Vec::with_capacity(100),
        };
        app.load_level();
        Ok(app)
    }

    fn load_level(&mut self) {
        if self.commits.is_empty() {
            self.message = "No commits found. Is this a git repo?".to_string();
            return;
        }

        let commit = &self.commits[self.current_commit_idx];

        // Seed RNG from commit hash to make level deterministic-ish (but we want gameplay to be dynamic, so let's just use hash for params)
        let hash_bytes = commit.hash.as_bytes();
        let seed_val = hash_bytes.iter().map(|&b| b as u64).sum::<u64>();
        self.rng = StdRng::seed_from_u64(seed_val);

        // Grid setup
        self.grid = Grid::new(60, 40);

        // Player Reset
        self.trader.x = 2.0;
        self.trader.y = (self.grid.height as f64) / 2.0;
        // Keep balance and position across levels? Or reset?
        // Let's keep them! It's a roguelike run.

        // Level Params
        // Price determined by first byte
        self.market_price = (hash_bytes[0] as f32 % (self.grid.height as f32 - 10.0)) + 5.0;

        // Volatility determined by message length
        let msg_len = commit.message.len().max(1);
        self.volatility = (10.0 / msg_len as f64).clamp(0.05, 0.5);

        self.game_over = false;
        self.message = format!("Level {}: {}", self.current_commit_idx + 1, commit.message);
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // 1. Spawn Orders based on Volatility
        // Bids below price
        if self.rng.gen_bool(self.volatility) {
            let offset = self.rng.gen_range(1.0..15.0);
            let bid_y = (self.market_price + offset) as usize;
            let x = self.rng.gen_range(5..self.grid.width); // Don't spawn on spawn point
            if bid_y < self.grid.height {
                self.grid.set(x, bid_y, Particle::Bid);
            }
        }
        // Asks above price
        if self.rng.gen_bool(self.volatility) {
            let offset = self.rng.gen_range(1.0..15.0);
            let ask_y = (self.market_price - offset) as isize;
            let x = self.rng.gen_range(5..self.grid.width);
            if ask_y >= 0 {
                self.grid.set(x, ask_y as usize, Particle::Ask);
            }
        }

        // 2. Update Grid (Physics)
        self.grid.update();

        // 3. Collision Logic
        let px = self.trader.x.round() as usize;
        let py = self.trader.y.round() as usize;

        // Boundary Check
        if py >= self.grid.height || self.trader.y < 0.0 {
            self.message = "LIQUIDATED! (Hit Boundary). Press 'R' to retry.".to_string();
            self.game_over = true;
            // Penalty for liquidation?
            self.trader.balance -= 1000.0;
            return;
        }

        // Particle Interaction
        let particle = self.grid.get(px, py);
        if self.trader.interact(particle) {
             self.grid.set(px, py, Particle::Empty); // Consume
        }

        // Win Condition (Reach right side)
        if px >= self.grid.width - 2 {
            if self.current_commit_idx + 1 < self.commits.len() {
                self.current_commit_idx += 1;
                self.load_level();
            } else {
                self.message = format!("MARKET CONQUERED! Final Balance: ${:.2}", self.trader.balance);
                self.game_over = true;
            }
        }
    }

    fn move_player(&mut self, dx: f64, dy: f64) {
        if self.game_over { return; }
        self.trader.x = (self.trader.x + dx).clamp(0.0, self.grid.width as f64 - 1.0);
        self.trader.y = (self.trader.y + dy).clamp(0.0, self.grid.height as f64 - 1.0);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char('r') => app.load_level(),
                        KeyCode::Char('s') => app.trader.toggle_position(), // Toggle Long/Short
                        KeyCode::Up => app.move_player(0.0, -1.0),
                        KeyCode::Down => app.move_player(0.0, 1.0),
                        KeyCode::Left => app.move_player(-1.0, 0.0),
                        KeyCode::Right => app.move_player(1.0, 0.0),
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

fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Info
            Constraint::Min(0),    // Game
            Constraint::Length(1), // Help
        ])
        .split(f.area());

    // Info
    let commit_info = if !app.commits.is_empty() {
        let c = &app.commits[app.current_commit_idx];
        format!("{} | {}", c.short_hash, c.author)
    } else {
        "No Repo".to_string()
    };

    // Position Style
    let pos_style = match app.trader.position {
        Position::Long => Style::default().fg(Color::Green).bold(),
        Position::Short => Style::default().fg(Color::Cyan).bold(),
    };
    let pos_text = match app.trader.position {
        Position::Long => "LONG",
        Position::Short => "SHORT",
    };

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(" MARKET ROGUE ", Style::default().fg(Color::Yellow).bold()),
            Span::raw(format!("Level {}/{} | ", app.current_commit_idx + 1, app.commits.len())),
            Span::raw("Balance: "),
            Span::styled(format!("${:.0}", app.trader.balance), if app.trader.balance >= 10000.0 { Style::default().fg(Color::Green) } else { Style::default().fg(Color::Red) }),
            Span::raw(" | Pos: "),
            Span::styled(pos_text, pos_style),
        ]),
        Line::from(vec![
            Span::raw(format!("Price: {:.1} | Volatility: {:.2} | ", app.market_price, app.volatility)),
            Span::styled(&app.message, Style::default().fg(if app.game_over { Color::Red } else { Color::Green })),
        ]),
        Line::from(commit_info),
    ])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[0]);

    // Game Canvas
    app.bids_buf.clear();
    app.asks_buf.clear();
    app.trades_buf.clear();

    for y in 0..app.grid.height {
        for x in 0..app.grid.width {
            match app.grid.get(x, y) {
                Particle::Bid => {
                    app.bids_buf.push((x as f64, (app.grid.height - 1 - y) as f64));
                }
                Particle::Ask => {
                    app.asks_buf.push((x as f64, (app.grid.height - 1 - y) as f64));
                }
                Particle::Trade { .. } => {
                    app.trades_buf.push((x as f64, (app.grid.height - 1 - y) as f64));
                }
                Particle::Empty => {}
            }
        }
    }

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Exchange Floor "))
        .x_bounds([0.0, app.grid.width as f64])
        .y_bounds([0.0, app.grid.height as f64])
        .marker(Marker::Block)
        .paint(|ctx| {
            // Draw Particles
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

            // Draw Trader
            let py_render = app.grid.height as f64 - 1.0 - app.trader.y;
            let trader_color = match app.trader.position {
                Position::Long => Color::Yellow,
                Position::Short => Color::Cyan,
            };
            ctx.draw(&Points {
                coords: &[(app.trader.x, py_render)],
                color: trader_color,
            });
            // Draw Exit Line
            let exit_x = app.grid.width as f64 - 2.0;
            ctx.print(exit_x, app.grid.height as f64 / 2.0, "EXIT");
        });
    f.render_widget(canvas, chunks[1]);

    // Help
    let help = Paragraph::new("ARROWS: Move | S: Toggle Short/Long | Q: Quit | R: Restart Level")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[2]);
}
