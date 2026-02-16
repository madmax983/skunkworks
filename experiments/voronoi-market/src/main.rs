use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Paragraph, Widget},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

// --- Physics & Market Types ---

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Bid, // Buyer (Green)
    Ask, // Seller (Red)
}

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

struct Trader {
    id: u64,
    pos: Vec2,
    vel: Vec2,
    side: Side,
    #[allow(dead_code)]
    volume: f32,
}

struct Market {
    width: f64,
    height: f64,
    traders: Vec<Trader>,
    next_id: u64,
    trade_count: u64,
    last_price: f64,
}

impl Market {
    fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            traders: Vec::new(),
            next_id: 0,
            trade_count: 0,
            last_price: height / 2.0,
        }
    }

    fn spawn_trader(&mut self) {
        let mut rng = rand::thread_rng();
        let side = if rng.gen_bool(0.5) { Side::Bid } else { Side::Ask };

        // Bids spawn at bottom (low price/high Y in screen coords? No, let's say Y=0 is top/high price)
        // Wait, usually Y=0 is top in terminal.
        // So Low Price = High Y (bottom). High Price = Low Y (top).
        // Bids want low price, so they spawn at bottom (High Y) and move up (Low Y).
        // Asks want high price, so they spawn at top (Low Y) and move down (High Y).

        let (y, vy) = match side {
            Side::Bid => (self.height - 1.0, -rng.gen_range(0.1..0.5)), // Start bottom, move up
            Side::Ask => (1.0, rng.gen_range(0.1..0.5)),                // Start top, move down
        };

        let x = rng.gen_range(0.0..self.width);
        let volume = rng.gen_range(1.0..10.0);

        self.traders.push(Trader {
            id: self.next_id,
            pos: Vec2 { x, y },
            vel: Vec2 {
                x: rng.gen_range(-0.2..0.2), // Slight horizontal drift
                y: vy,
            },
            side,
            volume,
        });
        self.next_id += 1;
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Spawn randomly
        if rng.gen_bool(0.15) {
            self.spawn_trader();
        }

        // Move Traders
        for trader in &mut self.traders {
            trader.pos.x += trader.vel.x;
            trader.pos.y += trader.vel.y;

            // Bounce X
            if trader.pos.x < 0.0 || trader.pos.x >= self.width {
                trader.vel.x *= -1.0;
                trader.pos.x = trader.pos.x.clamp(0.0, self.width - 0.1);
            }

            // Remove if out of bounds Y (drifting away from market)
            // But we handle this in cleanup
        }

        // Check Collisions / Trades
        let mut dead_ids = Vec::new();
        let mut trades = Vec::new();

        for i in 0..self.traders.len() {
            if dead_ids.contains(&self.traders[i].id) { continue; }

            for j in (i + 1)..self.traders.len() {
                if dead_ids.contains(&self.traders[j].id) { continue; }

                let t1 = &self.traders[i];
                let t2 = &self.traders[j];

                if t1.side != t2.side {
                    let dist = (t1.pos.x - t2.pos.x).hypot(t1.pos.y - t2.pos.y);

                    if dist < 4.0 { // Increased interaction radius
                        dead_ids.push(t1.id);
                        dead_ids.push(t2.id);
                        trades.push((t1.pos.y + t2.pos.y) / 2.0);
                    }
                }
            }
        }

        for price in trades {
            self.trade_count += 1;
            self.last_price = price;
        }

        let height = self.height;
        self.traders.retain(|t| !dead_ids.contains(&t.id)
            && t.pos.y >= 0.0
            && t.pos.y <= height
        );
    }
}

// --- TUI Logic ---

struct App {
    market: Market,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            market: Market::new(100.0, 50.0), // Initial size
            should_quit: false,
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.market.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.size());

    // Update Market Size to match View
    let area = chunks[0];
    if area.width as f64 != app.market.width || area.height as f64 != app.market.height {
        app.market.width = area.width as f64;
        app.market.height = area.height as f64;
    }

    let voronoi_widget = VoronoiWidget { market: &app.market };
    f.render_widget(voronoi_widget, area);

    // Status Bar
    let status_text = vec![Line::from(vec![
        " [Q] Quit ".yellow().bold(),
        format!(" | Traders: {} ", app.market.traders.len()).into(),
        format!(" | Trades: {} ", app.market.trade_count).into(),
        format!(" | Last Price: {:.1}", app.market.last_price).into(),
    ])];

    let status = Paragraph::new(status_text).style(Style::default().bg(Color::DarkGray));
    f.render_widget(status, chunks[1]);
}

struct VoronoiWidget<'a> {
    market: &'a Market,
}

impl<'a> Widget for VoronoiWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Voronoi Rendering
        // Iterate over all cells in the area
        for y in 0..area.height {
            for x in 0..area.width {
                let px = x as f64;
                let py = y as f64;

                let mut min_dist_sq = f64::MAX;
                let mut nearest_trader: Option<&Trader> = None;

                for trader in &self.market.traders {
                    let dx = px - trader.pos.x;
                    // Aspect ratio correction (chars are ~2x taller than wide)
                    let dy = (py - trader.pos.y) * 2.0;
                    let dist_sq = dx*dx + dy*dy;

                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        nearest_trader = Some(trader);
                    }
                }

                if let Some(trader) = nearest_trader {
                    let cell = buf.get_mut(area.x + x, area.y + y);
                    match trader.side {
                        Side::Bid => {
                            cell.set_bg(Color::Green);
                            cell.set_fg(Color::Black);
                        },
                        Side::Ask => {
                            cell.set_bg(Color::Red);
                            cell.set_fg(Color::Black);
                        },
                    }

                    // Draw center?
                    let tx = trader.pos.x.round() as u16;
                    let ty = trader.pos.y.round() as u16;

                    if x == tx && y == ty {
                        cell.set_char(match trader.side {
                            Side::Bid => '▲',
                            Side::Ask => '▼',
                        });
                        cell.set_fg(Color::White);
                    }
                }
            }
        }
    }
}
