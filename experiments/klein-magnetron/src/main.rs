mod fs;
mod model;
mod renderer;
mod topology;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use glam::Vec3;
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

use fs::scan_and_populate;
use model::Platter;
use renderer::{draw_platter, Camera};

struct App {
    platter: Platter,
    selected_idx: usize,

    // Camera
    camera_angle: f32,
    camera_height: f32,
    camera_radius: f32,

    should_quit: bool,
    time_scale: f32,
}

impl App {
    fn new() -> Result<Self> {
        let mut platter = Platter::new();
        let current_path = std::env::current_dir()?;
        scan_and_populate(&current_path, &mut platter)?;

        Ok(Self {
            platter,
            selected_idx: 0,
            camera_angle: 0.0,
            camera_height: 1.0,
            camera_radius: 6.0,
            should_quit: false,
            time_scale: 1.0,
        })
    }

    fn on_tick(&mut self, dt: f32) {
        self.platter.update(dt * self.time_scale);
        // Auto-rotate camera slightly?
        // self.camera_angle += dt * 0.1;
    }

    fn move_selection(&mut self, delta: i32) {
        if self.platter.sectors.is_empty() {
            return;
        }
        let len = self.platter.sectors.len();
        self.selected_idx = (self.selected_idx as i32 + delta).rem_euclid(len as i32) as usize;
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Left => app.camera_angle -= 0.1,
                    KeyCode::Right => app.camera_angle += 0.1,
                    KeyCode::Char('w') => app.camera_height += 0.5,
                    KeyCode::Char('s') => app.camera_height -= 0.5,
                    KeyCode::Char('a') => app.move_selection(-1),
                    KeyCode::Char('d') => app.move_selection(1),
                    KeyCode::Char(' ') => {
                        if let Some(sector) = app.platter.sectors.get_mut(app.selected_idx) {
                            sector.scrub();
                        }
                    }
                    KeyCode::Char('+') => app.time_scale *= 2.0,
                    KeyCode::Char('-') => app.time_scale *= 0.5,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f32();
            app.on_tick(dt);
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Main
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(format!(
        "KLEIN-MAGNETRON | Time: {:.1}x | Polarity: {}",
        app.time_scale,
        if app.platter.radiation_polarity {
            "NORMAL"
        } else {
            "INVERTED"
        }
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"))
    .style(Style::default().fg(if app.platter.radiation_polarity {
        Color::Green
    } else {
        Color::Magenta
    }));
    f.render_widget(title, chunks[0]);

    // Content
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    // Canvas
    let canvas_area = content_chunks[0];
    let aspect = (canvas_area.width as f32 * 0.5) / canvas_area.height as f32;

    let camera_pos = Vec3::new(
        app.camera_radius * app.camera_angle.cos(),
        app.camera_height,
        app.camera_radius * app.camera_angle.sin(),
    );
    let camera = Camera::new(camera_pos, Vec3::ZERO);

    let platter_ref = &app.platter;
    let selected_idx = app.selected_idx;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Topological Platter"),
        )
        .x_bounds([-2.0, 2.0])
        .y_bounds([-1.5, 1.5])
        .paint(move |ctx| {
            draw_platter(ctx, &camera, aspect, platter_ref, selected_idx);
        });
    f.render_widget(canvas, content_chunks[0]);

    // Inspector
    let mut lines = vec![];
    if let Some(sector) = app.platter.sectors.get(app.selected_idx) {
        lines.push(Line::from(vec![
            Span::raw("File: "),
            Span::styled(&sector.label, Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("Signal: "),
            Span::styled(
                format!("{:.1}%", sector.magnetization * 100.0),
                Style::default().fg(if sector.magnetization > 0.8 {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("Coercivity: "),
            Span::styled(
                format!("{:.1}%", sector.coercivity * 100.0),
                Style::default().fg(Color::Blue),
            ),
        ]));
        lines.push(Line::from(""));

        // Hex Dump
        for chunk in sector.data.chunks(8).take(4) {
            let hex: String = chunk.iter().map(|b| format!("{:02X} ", b)).collect();
            let ascii: String = chunk
                .iter()
                .map(|b| {
                    if *b >= 32 && *b <= 126 {
                        *b as char
                    } else {
                        '.'
                    }
                })
                .collect();
            lines.push(Line::from(format!("{} | {}", hex, ascii)));
        }
    } else {
        lines.push(Line::from("No Data"));
    }

    let inspector =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Inspector"));
    f.render_widget(inspector, content_chunks[1]);

    // Help
    let help = Paragraph::new(
        "Arrows: Rotate Cam (L/R), Select (A/D) | W/S: Cam Height | Space: Scrub | +/-: Time",
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
