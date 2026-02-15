use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use poincare_disk::{mobius_add, Point};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

// --- Constants ---
const BOUNDARY_RADIUS: f64 = 1.0;
const SPAWN_EDGE_MARGIN: f64 = 0.9;
const MOVEMENT_SPEED: f64 = 0.02;
const COLLISION_THRESHOLD: f64 = 0.05; // Euclidean distance on disk
const TRADE_AGE_MAX: u8 = 10;
const TRADER_COUNT: usize = 100;

// --- Data Structures ---

#[derive(Clone, Copy, PartialEq, Debug)]
enum TraderType {
    Bid, // Buyer (Green, moves Up)
    Ask, // Seller (Red, moves Down)
}

#[derive(Clone, Copy, Debug)]
struct Trader {
    pos: Point,
    kind: TraderType,
}

#[derive(Clone, Copy, Debug)]
struct TradeEvent {
    pos: Point,
    price: f64,
    age: u8,
}

struct App {
    traders: Vec<Trader>,
    trades: Vec<TradeEvent>,
    rng: rand::rngs::ThreadRng,
    last_tick: Instant,
    total_volume: usize,
    price_history: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            traders: Vec::new(),
            trades: Vec::new(),
            rng: rand::thread_rng(),
            last_tick: Instant::now(),
            total_volume: 0,
            price_history: Vec::new(),
        };
        // Initial spawn
        for _ in 0..TRADER_COUNT {
            app.spawn_trader();
        }
        app
    }

    fn spawn_trader(&mut self) {
        // 50/50 chance of Bid or Ask
        let kind = if self.rng.gen_bool(0.5) {
            TraderType::Bid
        } else {
            TraderType::Ask
        };

        // Spawn logic:
        // Bids spawn at bottom (Im approx -0.9)
        // Asks spawn at top (Im approx 0.9)
        // X (Re) is random [-0.8, 0.8]

        // We use slightly randomized spawn points to create a "cloud"
        let re = self.rng.gen_range(-0.8..0.8);
        let im_base = match kind {
            TraderType::Bid => -SPAWN_EDGE_MARGIN,
            TraderType::Ask => SPAWN_EDGE_MARGIN,
        };
        // Add some jitter to Y
        let im = im_base + self.rng.gen_range(-0.05..0.05);

        let pos = Point::new(re, im);

        // Ensure strictly inside disk (should be safe with these constants, but clamp just in case)
        let norm = pos.norm();
        let safe_pos = if norm >= 0.99 {
            pos * (0.98 / norm)
        } else {
            pos
        };

        self.traders.push(Trader {
            pos: safe_pos,
            kind,
        });
    }

    fn update(&mut self) {
        let mut to_remove = Vec::new();
        let mut new_trades = Vec::new();

        // 1. Move Traders
        for (i, trader) in self.traders.iter_mut().enumerate() {
            let step = match trader.kind {
                TraderType::Bid => Point::new(0.0, MOVEMENT_SPEED),  // Move "Up"
                TraderType::Ask => Point::new(0.0, -MOVEMENT_SPEED), // Move "Down"
            };

            // Hyperbolic translation: pos = mobius_add(step, pos)
            // This adds 'step' in the local reference frame of 'pos'
            trader.pos = mobius_add(step, trader.pos);

            // Check boundary exit (liquidity drying up / order cancelled)
            // If they reach the OTHER side, they expire (unfilled)
            match trader.kind {
                TraderType::Bid => {
                    if trader.pos.im > 0.95 {
                        to_remove.push(i);
                    }
                }
                TraderType::Ask => {
                    if trader.pos.im < -0.95 {
                        to_remove.push(i);
                    }
                }
            }
        }

        // 2. Collision Detection (Naive O(N^2) but N=100 is fine)
        // We check Bids against Asks
        let mut matched_indices = std::collections::HashSet::new();

        for i in 0..self.traders.len() {
            if matched_indices.contains(&i) || to_remove.contains(&i) {
                continue;
            }

            for j in (i + 1)..self.traders.len() {
                if matched_indices.contains(&j) || to_remove.contains(&j) {
                    continue;
                }

                let t1 = self.traders[i];
                let t2 = self.traders[j];

                // Only trade if different types
                if t1.kind != t2.kind {
                    // Check visual distance (Euclidean on disk)
                    let dist_sq = (t1.pos - t2.pos).norm_sqr();
                    if dist_sq < COLLISION_THRESHOLD * COLLISION_THRESHOLD {
                        // Trade!
                        matched_indices.insert(i);
                        matched_indices.insert(j);
                        to_remove.push(i);
                        to_remove.push(j);

                        // Calculate Price (based on Y position normalized)
                        // Map Y from [-1, 1] to [0, 100] approximately
                        // Collision pos is average
                        let collision_pos = (t1.pos + t2.pos) / 2.0;
                        let price = (collision_pos.im + 1.0) * 50.0;

                        new_trades.push(TradeEvent {
                            pos: collision_pos,
                            price,
                            age: TRADE_AGE_MAX,
                        });

                        self.total_volume += 1;
                        self.price_history.push(price);
                        if self.price_history.len() > 100 {
                            self.price_history.remove(0);
                        }

                        // Break inner loop since 'i' is now removed
                        break;
                    }
                }
            }
        }

        // Remove matched/expired traders (in reverse order to keep indices valid... wait, indices are tricky)
        // Better: filter and rebuild
        // Or just mark as dead
        // Simple rebuild:
        let mut next_traders = Vec::new();
        for (i, trader) in self.traders.iter().enumerate() {
            if !to_remove.contains(&i) {
                next_traders.push(*trader);
            }
        }
        self.traders = next_traders;

        // Respawn to maintain population
        while self.traders.len() < TRADER_COUNT {
            self.spawn_trader();
        }

        // Add new trades
        self.trades.extend(new_trades);

        // Age trades
        self.trades.retain_mut(|t| {
            if t.age > 0 {
                t.age -= 1;
                true
            } else {
                false
            }
        });
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // Update
        if app.last_tick.elapsed() >= Duration::from_millis(50) {
            app.update();
            app.last_tick = Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    // Info Bar
    let avg_price = if app.price_history.is_empty() {
        50.0
    } else {
        app.price_history.iter().sum::<f64>() / app.price_history.len() as f64
    };

    let info_text = vec![
        Line::from(vec![
            Span::styled("Hyperbolic Market", Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled(format!("Vol: {}", app.total_volume), Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::styled(format!("Price: {:.2}", avg_price), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::raw("Green: Bids (Moving Up) | Red: Asks (Moving Down) | Flash: Trade | Q: Quit"),
        ]),
    ];

    f.render_widget(
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL)),
        info_area,
    );

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Poincaré Disk"))
        .x_bounds([-1.1, 1.1])
        .y_bounds([-1.1, 1.1])
        .paint(|ctx| {
            // Draw Boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: BOUNDARY_RADIUS,
                color: Color::White,
            });

            // Draw Geodesic Grid (Optional, simple axes)
            ctx.draw(&CanvasLine {
                x1: -1.0, y1: 0.0, x2: 1.0, y2: 0.0, color: Color::DarkGray
            });
            ctx.draw(&CanvasLine {
                x1: 0.0, y1: -1.0, x2: 0.0, y2: 1.0, color: Color::DarkGray
            });

            // Draw Traders
            for trader in &app.traders {
                let color = match trader.kind {
                    TraderType::Bid => Color::Green,
                    TraderType::Ask => Color::Red,
                };
                // Radius expands near boundary to show perspective?
                // No, kept constant for clarity, or small dot.
                ctx.draw(&Circle {
                    x: trader.pos.re,
                    y: trader.pos.im,
                    radius: 0.015,
                    color,
                });
            }

            // Draw Trades
            for trade in &app.trades {
                // Flash decays
                let base_color = if trade.price > avg_price { Color::LightGreen } else { Color::LightRed };
                let color = if trade.age > 5 { Color::Yellow } else { base_color };
                ctx.draw(&Circle {
                    x: trade.pos.re,
                    y: trade.pos.im,
                    radius: 0.03, // Slightly larger explosion
                    color,
                });
            }
        });

    f.render_widget(canvas, canvas_area);
}
