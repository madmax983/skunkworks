//! # quipu-poincare
//!
//! ## Concept
//! Hyperbolic Knot Morphogenesis.
//!
//! ## Parents
//! - `quipu`: Generates discrete structural knots representing ancient data cords.
//! - `poincare-disk`: Provides the mathematical primitives for hyperbolic non-Euclidean space mapping.
//!
//! ## Novel Trait
//! Projecting the discrete, structural data knots of a Quipu cord into the continuous 2D Poincaré disk. As the values and depths of the knots increase, they are constrained and squashed towards the hyperbolic boundary.
//!
//! ## Phenotype
//! An organic visualization where integer data values are compressed towards the edge of a non-Euclidean disk, bridging discrete state persistence with infinite boundary limits.
//!
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use poincare_disk::{Mobius, Point};
use quipu::{Cord, Quipu};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

struct App {
    quipu: Quipu,
    view_center: Point,
    zoom: f64,
}

impl App {
    fn new() -> Self {
        let mut quipu = Quipu::new();
        quipu.add_cord(Cord::from(10));
        quipu.add_cord(Cord::from(200));
        quipu.add_cord(Cord::from(3000));
        quipu.add_cord(Cord::from(42));

        Self {
            quipu,
            view_center: Point::new(0.0, 0.0),
            zoom: 1.0,
        }
    }

    fn on_tick(&mut self) {
        // Slowly rotate or translate the view if desired
    }
}

fn map_to_poincare(cord_idx: usize, knot_idx: usize, value: usize) -> Point {
    let r_euclid = value as f64 * 2.0 + knot_idx as f64;
    let theta = (cord_idx as f64) * 0.5 + (knot_idx as f64) * 0.1;

    // Hyperbolic mapping
    let r_disk = (r_euclid / 20.0).tanh(); // Compress to disk bounds

    Point::new(r_disk * theta.cos(), r_disk * theta.sin())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();
    let block = Block::default()
        .title("Quipu Poincare - Hyperbolic Knot Morphogenesis")
        .borders(Borders::ALL);
    f.render_widget(block, size);

    let center_x = size.width / 2;
    let center_y = size.height / 2;
    let radius = (std::cmp::min(size.width, size.height * 2) / 2 - 2) as f64;

    for (c_idx, cord) in app.quipu.cords.iter().enumerate() {
        let value = cord.value();
        for (k_idx, cluster) in cord.clusters.iter().enumerate() {
            for (sub_k_idx, knot) in cluster.iter().enumerate() {
                let _knot_val = knot.value();
                let p = map_to_poincare(c_idx, k_idx + sub_k_idx, value as usize);
                let t = Mobius::translation(app.view_center);
                let transformed = t.apply(p);

                let screen_x = center_x as f64 + transformed.re * radius * app.zoom;
                let screen_y = center_y as f64 + transformed.im * (radius / 2.0) * app.zoom;

                if screen_x >= 1.0
                    && screen_x < size.width as f64 - 1.0
                    && screen_y >= 1.0
                    && screen_y < size.height as f64 - 1.0
                {
                    let symbol = knot.symbol();
                    let color = Color::Yellow;

                    let widget = Paragraph::new(Span::styled(symbol, Style::default().fg(color)));
                    f.render_widget(widget, Rect::new(screen_x as u16, screen_y as u16, 1, 1));
                }
            }
        }
    }
}

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting immediately.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
