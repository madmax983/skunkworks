use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod ants;
mod market;

use ants::AntColony;
use market::{Grid, Particle, Terrain};

struct App {
    grid: Grid,
    ants: AntColony,
    should_quit: bool,
    volatility: f32,

    // Render buffers
    bids_buf: Vec<(f64, f64)>,
    asks_buf: Vec<(f64, f64)>,
    bridge_buf: Vec<(f64, f64)>,
    ant_buf: Vec<(f64, f64)>,
}

impl App {
    fn new() -> Self {
        let width = 100;
        let height = 60;
        let mut grid = Grid::new(width, height);

        // Initialize Gap (Spread) in the middle
        for y in 25..35 {
            for x in 0..width {
                grid.set_terrain(x, y, Terrain::Gap);
            }
        }

        // Initialize Particles (Bids bottom, Asks top)
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Spawn Bids
        for _ in 0..200 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(40..height);
            grid.set_particle(x, y, Particle::Bid(0));
        }

        // Spawn Asks
        for _ in 0..200 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..20);
            grid.set_particle(x, y, Particle::Ask(0));
        }

        let ants = AntColony::new(width, height, 100);

        Self {
            grid,
            ants,
            should_quit: false,
            volatility: 0.05,
            bids_buf: Vec::with_capacity(1000),
            asks_buf: Vec::with_capacity(1000),
            bridge_buf: Vec::with_capacity(1000),
            ant_buf: Vec::with_capacity(1000),
        }
    }

    fn update(&mut self) {
        // 1. Update Ants (build bridges)
        // Pass grid to ants so they can modify terrain
        self.ants.update(&mut self.grid, self.volatility);

        // 2. Update Market (move particles)
        // Market respects terrain set by ants
        let _trades = self.grid.update();

        // 3. Spawn new particles to keep market alive
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            let x = rng.gen_range(0..self.grid.width);
            self.grid
                .set_particle(x, self.grid.height - 1, Particle::Bid(0));
        }
        if rng.gen_bool(0.1) {
            let x = rng.gen_range(0..self.grid.width);
            self.grid.set_particle(x, 0, Particle::Ask(0));
        }

        // 4. Volatility dynamics
        // Randomly fluctuate
        if rng.gen_bool(0.01) {
            self.volatility = (self.volatility + rng.gen_range(-0.01..0.01)).clamp(0.01, 0.2);
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal);
    drop(tui); // Restore terminal
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
                        KeyCode::Char('r') => app = App::new(),
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

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    // Prepare buffers
    app.bids_buf.clear();
    app.asks_buf.clear();
    app.bridge_buf.clear();
    app.ant_buf.clear();

    for y in 0..app.grid.height {
        for x in 0..app.grid.width {
            let render_y = (app.grid.height - 1 - y) as f64;
            let render_x = x as f64;

            // Terrain
            match app.grid.get_terrain(x, y) {
                Terrain::Bridge => {
                    app.bridge_buf.push((render_x, render_y));
                }
                _ => {}
            }

            // Particles
            match app.grid.get_particle(x, y) {
                Particle::Bid(_) => {
                    app.bids_buf.push((render_x, render_y));
                }
                Particle::Ask(_) => {
                    app.asks_buf.push((render_x, render_y));
                }
                Particle::Trade { .. } => {
                    // Flash?
                    app.bridge_buf.push((render_x, render_y)); // Use bridge color for trades?
                }
                _ => {}
            }
        }
    }

    // Ants overlay
    for ant in &app.ants.ants {
        let render_y = (app.grid.height - 1 - ant.y) as f64;
        let render_x = ant.x as f64;
        app.ant_buf.push((render_x, render_y));
    }

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Liquidity Bridge "),
        )
        .x_bounds([0.0, app.grid.width as f64])
        .y_bounds([0.0, app.grid.height as f64])
        .marker(Marker::Block)
        .paint(|ctx| {
            // Draw Bridge (Terrain)
            ctx.draw(&Points {
                coords: &app.bridge_buf,
                color: Color::Yellow,
            });

            // Draw Bids
            ctx.draw(&Points {
                coords: &app.bids_buf,
                color: Color::Green,
            });

            // Draw Asks
            ctx.draw(&Points {
                coords: &app.asks_buf,
                color: Color::Red,
            });

            // Draw Ants
            ctx.draw(&Points {
                coords: &app.ant_buf,
                color: Color::White,
            });
        });

    f.render_widget(canvas, chunks[0]);

    let status = Paragraph::new(format!(
        "Volatility: {:.2} | Ants: {} | Trades: {} | 'q': Quit, 'r': Reset",
        app.volatility,
        app.ants.ants.len(),
        app.grid.trade_count
    ))
    .style(Style::default().fg(Color::Cyan));

    f.render_widget(status, chunks[1]);
}
