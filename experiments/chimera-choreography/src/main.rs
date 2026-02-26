mod dancer;
mod laban;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use dancer::Dancer;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct App {
    dancers: Vec<Dancer>,
    running: bool,
    view_scale: f64,
}

impl App {
    fn new() -> Self {
        let mut dancers = Vec::new();
        // Spawn 20 dancers
        for i in 0..20 {
            dancers.push(Dancer::new(i, 0.0, 0.0));
        }
        Self {
            dancers,
            running: true,
            view_scale: 200.0,
        }
    }

    fn update(&mut self, dt: f32) {
        for dancer in &mut self.dancers {
            dancer.update(dt);
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    // Game loop
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            handle_input(&mut app, event::read()?)?;
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f32();
            app.update(dt);
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn handle_input(app: &mut App, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => app.running = false,
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    app.view_scale = (app.view_scale - 10.0).max(10.0);
                }
                KeyCode::Char('-') => {
                    app.view_scale += 10.0;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3), // Info
        ])
        .split(f.area());

    draw_canvas(f, app, chunks[0]);
    draw_info(f, app, chunks[1]);
}

fn draw_canvas(f: &mut Frame, app: &App, area: Rect) {
    let r = app.view_scale;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chimera Choreography"),
        )
        .x_bounds([-r, r])
        .y_bounds([-r, r])
        .marker(symbols::Marker::Braille)
        .paint(|ctx| {
            for dancer in &app.dancers {
                // Determine color based on Flow and Weight
                // Free = Cyan, Bound = Magenta
                // Strong = Bold (Bright), Light = Dim (Dark)

                let color = if dancer.effort.is_free() {
                    if dancer.effort.is_strong() {
                        Color::Cyan
                    } else {
                        Color::Blue
                    }
                } else {
                    if dancer.effort.is_strong() {
                        Color::Magenta
                    } else {
                        Color::Red
                    }
                };

                // Draw dancer as a point or line indicating velocity
                ctx.print(
                    dancer.pos.x as f64,
                    dancer.pos.y as f64,
                    Span::styled("💃", Style::default().fg(color)),
                );

                // Velocity line
                let end = dancer.pos + dancer.vel * 0.2; // scale vector for visibility
                ctx.draw(&CanvasLine {
                    x1: dancer.pos.x as f64,
                    y1: dancer.pos.y as f64,
                    x2: end.x as f64,
                    y2: end.y as f64,
                    color,
                });
            }
        });

    f.render_widget(canvas, area);
}

fn draw_info(f: &mut Frame, _app: &App, area: Rect) {
    let info = Paragraph::new("Press 'q' to quit. +/- to zoom. Colors: Cyan=Free, Magenta=Bound. Bright=Strong, Dim=Light.")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, area);
}
