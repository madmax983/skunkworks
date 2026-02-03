use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Circle, Context},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::physics::System;

pub mod graph;
pub mod physics;

struct App {
    system: System,
    camera_pos: (f64, f64),
    zoom: f64,
    paused: bool,
    show_names: bool,
    tick_count: u64,
}

impl App {
    fn new(system: System) -> Self {
        Self {
            system,
            camera_pos: (0.0, 0.0),
            zoom: 1.0,
            paused: false,
            show_names: true,
            tick_count: 0,
        }
    }

    fn on_tick(&mut self) {
        if !self.paused {
            // Physics step
            // Use small steps for stability
            let dt = 0.05;
            self.system.update(dt);
            self.tick_count += 1;
        }
    }
}

fn main() -> Result<()> {
    // Load data
    println!("Loading crate universe...");
    let system = graph::load_workspace()?;
    let mut app = App::new(system);

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run Loop
    let res = run_app(&mut terminal, &mut app);

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('n') => app.show_names = !app.show_names,
                        // Pan
                        KeyCode::Left | KeyCode::Char('a') => app.camera_pos.0 -= 10.0 / app.zoom,
                        KeyCode::Right | KeyCode::Char('d') => app.camera_pos.0 += 10.0 / app.zoom,
                        KeyCode::Up | KeyCode::Char('w') => app.camera_pos.1 += 10.0 / app.zoom,
                        KeyCode::Down | KeyCode::Char('s') => app.camera_pos.1 -= 10.0 / app.zoom,
                        // Zoom
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        _ => {}
                    }
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let (cx, cy) = app.camera_pos;
    // Calculate bounds based on zoom
    // Aspect ratio correction?
    let aspect = chunks[0].width as f64 / chunks[0].height.max(1) as f64;
    // Base view width = 200.0
    let view_width = 200.0 / app.zoom;
    let view_height = view_width / aspect * 2.0; // Char height is ~2x width

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Crate Universe"),
        )
        .x_bounds([cx - view_width / 2.0, cx + view_width / 2.0])
        .y_bounds([cy - view_height / 2.0, cy + view_height / 2.0])
        .paint(|ctx| {
            draw_system(ctx, app);
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Pos: ({:.1}, {:.1}) | Zoom: {:.2} | Bodies: {} | Paused: {} (Space) | Quit: q",
        cx,
        cy,
        app.zoom,
        app.system.bodies.len(),
        app.paused
    );
    f.render_widget(
        Paragraph::new(status).style(Style::default().bg(Color::Blue)),
        chunks[1],
    );
}

fn draw_system(ctx: &mut Context, app: &App) {
    for body in &app.system.bodies {
        let color = if body.is_fixed {
            Color::Yellow
        } else if body.mass > 100.0 {
            Color::Red
        } else if body.mass > 50.0 {
            Color::Magenta
        } else {
            Color::Cyan
        };

        // Draw body
        // Radius based on mass, but scaled down for view
        // Canvas Circle radius is in coordinate units.
        // Body radius is sqrt(mass).
        // Let's use visual radius scaling
        let radius = (body.radius * 0.5).max(1.0);

        ctx.draw(&Circle {
            x: body.pos.x,
            y: body.pos.y,
            radius,
            color,
        });

        if app.show_names && (body.mass > 50.0 || app.zoom > 2.0) {
            ctx.print(
                body.pos.x + radius,
                body.pos.y,
                Span::raw(body.name.clone()),
            );
        }
    }
}
