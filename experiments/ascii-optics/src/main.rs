mod model;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use model::{Ray, Simulation, Vec2};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct App {
    sim: Simulation,
    cursor_x: usize,
    cursor_y: usize,
    tools: Vec<char>,
    current_tool_idx: usize,
    source_pos: Vec2,
    source_dir: Vec2,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let width = 80;
        let height = 40;
        let mut sim = Simulation::new(width, height);

        // Add some default mirrors
        sim.grid.set(10, 10, '/');
        sim.grid.set(20, 10, '\\');
        sim.grid.set(20, 20, '/');
        sim.grid.set(10, 20, '\\');

        Self {
            sim,
            cursor_x: 10,
            cursor_y: 10,
            tools: vec!['/', '\\', '|', '-', '#', ' '],
            current_tool_idx: 0,
            source_pos: Vec2::new(5.5, 15.5),
            source_dir: Vec2::new(1.0, -0.5),
            should_quit: false,
        }
    }

    fn current_tool(&self) -> char {
        self.tools[self.current_tool_idx]
    }

    fn on_tick(&mut self) {
        // Update rays
        self.sim.clear_rays();
        self.sim.add_ray(Ray::new(
            self.source_pos.x,
            self.source_pos.y,
            self.source_dir.x,
            self.source_dir.y,
            Color::Yellow,
        ));
        self.sim.step();
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Left => app.cursor_x = app.cursor_x.saturating_sub(1),
                        KeyCode::Right => app.cursor_x = (app.cursor_x + 1).min(app.sim.grid.width - 1),
                        KeyCode::Up => app.cursor_y = app.cursor_y.saturating_sub(1),
                        KeyCode::Down => app.cursor_y = (app.cursor_y + 1).min(app.sim.grid.height - 1),

                        KeyCode::Tab => {
                            app.current_tool_idx = (app.current_tool_idx + 1) % app.tools.len();
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            let tool = app.current_tool();
                            app.sim.grid.set(app.cursor_x, app.cursor_y, tool);
                        }

                        // Move Source
                        KeyCode::Char('w') => app.source_pos.y -= 0.5,
                        KeyCode::Char('s') => app.source_pos.y += 0.5,
                        KeyCode::Char('a') => app.source_pos.x -= 0.5,
                        KeyCode::Char('d') => app.source_pos.x += 0.5,

                        // Rotate Source
                        KeyCode::Char('z') => {
                            // Rotate left
                            let cos = (-0.1f64).cos();
                            let sin = (-0.1f64).sin();
                            let nx = app.source_dir.x * cos - app.source_dir.y * sin;
                            let ny = app.source_dir.x * sin + app.source_dir.y * cos;
                            app.source_dir = Vec2::new(nx, ny).normalize();
                        }
                        KeyCode::Char('x') => {
                            // Rotate right
                            let cos = (0.1f64).cos();
                            let sin = (0.1f64).sin();
                            let nx = app.source_dir.x * cos - app.source_dir.y * sin;
                            let ny = app.source_dir.x * sin + app.source_dir.y * cos;
                            app.source_dir = Vec2::new(nx, ny).normalize();
                        }

                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    // Calculate viewport based on grid size
    // We map grid (0..width, 0..height) to canvas bounds
    // Note: Canvas Y is up, Grid Y is down.
    // We need to invert Y when drawing.

    let w = app.sim.grid.width as f64;
    let h = app.sim.grid.height as f64;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" ASCII Optics "))
        .marker(Marker::Braille)
        .x_bounds([0.0, w])
        .y_bounds([0.0, h])
        .paint(|ctx| {
            // Draw Grid Elements
            // Since we can't draw text easily on Canvas in Ratatui < 0.29 without Labels (which are limited)
            // wait, we can't draw text on Canvas easily?
            // "Lesson: Ratatui's Canvas widget can render arbitrary text at coordinates" - Literary Boids.
            // Let's check how they do it.
            // Actually, Canvas only supports Shapes (Line, Point, Rectangle, Circle).
            // Maybe they mean drawing points that form text?
            // Or maybe using `ctx.print(x, y, "text")`?
            // Checking docs... yes `Context::print` exists.

            for y in 0..app.sim.grid.height {
                for x in 0..app.sim.grid.width {
                    let c = app.sim.grid.get(x, y);
                    if c != ' ' {
                        // Invert Y: 0 is top in grid, h is top in canvas?
                        // No, canvas 0 is bottom.
                        // So grid y=0 -> canvas y=h-1
                        // grid y=10 -> canvas y=h-1-10
                        let cy = h - 1.0 - y as f64;
                        ctx.print(x as f64, cy, c.to_string());
                    }
                }
            }

            // Draw Cursor
            let cx = app.cursor_x as f64;
            let cy = h - 1.0 - app.cursor_y as f64;
            ctx.print(cx, cy, "█".to_string().fg(Color::Blue));

            // Draw Rays
            for ray in &app.sim.rays {
                if ray.path.len() < 2 { continue; }

                for i in 0..ray.path.len() - 1 {
                    let p1 = ray.path[i];
                    let p2 = ray.path[i+1];

                    // Invert Y
                    // Actually our ray positions are in grid coordinates [0, w].
                    // grid(0,0) is top-left.
                    // canvas(0,0) is bottom-left.
                    // So y_canvas = height - y_grid.

                    ctx.draw(&CanvasLine {
                        x1: p1.x,
                        y1: h - p1.y,
                        x2: p2.x,
                        y2: h - p2.y,
                        color: ray.color,
                    });
                }
            }

            // Draw Source
             ctx.print(app.source_pos.x, h - app.source_pos.y, "S".to_string().fg(Color::Yellow).bold());
        });

    f.render_widget(canvas, canvas_area);

    // Info
    let tool_spans: Vec<Span> = app.tools.iter().enumerate().map(|(i, t)| {
        let label = if *t == ' ' { "Clr".to_string() } else { t.to_string() };
        let s = format!(" {} ", label);
        if i == app.current_tool_idx {
            Span::styled(s, Style::default().fg(Color::Black).bg(Color::Cyan))
        } else {
            Span::raw(s)
        }
    }).collect();

    let mut info_text = vec![
        Span::raw("Tool: [Tab] "),
    ];
    info_text.extend(tool_spans);
    info_text.extend(vec![
        Span::raw(" | Place: [Space] | Move Src: [WASD] | Rot Src: [Z/X] | Quit: [Q]"),
    ]);

    let info = Paragraph::new(Line::from(info_text))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, info_area);
}
