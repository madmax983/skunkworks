use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod git;
mod harmonograph;

use git::{get_commit_history, Commit};
use harmonograph::{generate_points, HarmonographParams};

struct App {
    commits: Vec<Commit>,
    current_index: usize,
    points: Vec<(f64, f64)>,
    params: HarmonographParams,
    // Animation
    draw_progress: usize,
    auto_play: bool,
}

impl App {
    fn new() -> Result<Self> {
        // Try to get commits. If failed, use dummy.
        let commits = match get_commit_history() {
            Ok(c) if !c.is_empty() => c,
            _ => vec![Commit {
                hash: "0000000000000000000000000000000000000000".to_string(),
                author: "Nova".to_string(),
                message: "No git history found (or error). Enjoy this default curve.".to_string(),
            }],
        };

        let mut app = Self {
            commits,
            current_index: 0,
            points: Vec::new(),
            params: HarmonographParams {
                f1: 1.0,
                f2: 1.0,
                f3: 1.0,
                f4: 1.0,
                p1: 0.0,
                p2: 0.0,
                p3: 0.0,
                p4: 0.0,
                d1: 0.0,
                d2: 0.0,
                d3: 0.0,
                d4: 0.0,
            },
            draw_progress: 0,
            auto_play: false,
        };
        app.update_selection();
        Ok(app)
    }

    fn update_selection(&mut self) {
        if self.current_index >= self.commits.len() {
            self.current_index = 0;
        }
        let commit = &self.commits[self.current_index];
        self.params = HarmonographParams::from_hash(&commit.hash);
        // Generate more points for higher resolution
        self.points = generate_points(&self.params, 2000);
        self.draw_progress = 0;
    }

    fn next(&mut self) {
        if self.current_index + 1 < self.commits.len() {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
        self.update_selection();
    }

    fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.commits.len() - 1;
        }
        self.update_selection();
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    // Tick rate
    let tick_rate = Duration::from_millis(16); // 60 FPS for smooth drawing
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Right | KeyCode::Char('l') => app.next(),
                        KeyCode::Left | KeyCode::Char('h') => app.prev(),
                        KeyCode::Char(' ') => app.auto_play = !app.auto_play,
                        KeyCode::Char('r') => app.draw_progress = 0,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Animate drawing
            if app.draw_progress < app.points.len() {
                app.draw_progress += 20; // Draw speed
                if app.draw_progress > app.points.len() {
                    app.draw_progress = app.points.len();
                }
            } else if app.auto_play {
                // Wait a bit then next?
                // For simplicity, just next immediately when done if auto_play
                // But that's too fast.
                // Let's rely on user for now, or add delay logic.
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Left: Canvas
    let canvas_block = Block::default()
        .borders(Borders::ALL)
        .title(" Git Harmonograph ");

    // Determine bounds based on points (auto-center)
    // Or fixed bounds [-200, 200]? Amplitude is 100 * 2 = 200 max.
    // So [-200, 200] is safe.

    let canvas = Canvas::default()
        .block(canvas_block)
        .x_bounds([-220.0, 220.0])
        .y_bounds([-220.0, 220.0])
        .marker(symbols::Marker::Braille)
        .paint(|ctx| {
            let color = get_author_color(&app.commits[app.current_index].author);

            // Draw up to draw_progress
            let visible_points = &app.points[0..app.draw_progress];

            ctx.draw(&Points {
                coords: visible_points,
                color,
            });
        });

    f.render_widget(canvas, chunks[0]);

    // Right: Info
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Details
            Constraint::Length(3), // Controls
        ])
        .split(chunks[1]);

    let title = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            "Git Harmonograph",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw(format!(
            "Commit {}/{}",
            app.current_index + 1,
            app.commits.len()
        ))]),
    ])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, info_chunks[0]);

    let commit = &app.commits[app.current_index];
    let author_color = get_author_color(&commit.author);

    let details_text = vec![
        Line::from(Span::styled("Hash:", Style::default().fg(Color::Gray))),
        Line::from(Span::raw(&commit.hash)),
        Line::from(""),
        Line::from(Span::styled("Author:", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(
            &commit.author,
            Style::default().fg(author_color),
        )),
        Line::from(""),
        Line::from(Span::styled("Message:", Style::default().fg(Color::Gray))),
        Line::from(Span::raw(&commit.message)),
        Line::from(""),
        Line::from(Span::styled("Harmonics:", Style::default().fg(Color::Gray))),
        Line::from(format!("F1: {:.2} Hz", app.params.f1)),
        Line::from(format!("F2: {:.2} Hz", app.params.f2)),
        Line::from(format!("Damping: {:.4}", app.params.d1)),
    ];

    let details = Paragraph::new(details_text)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" Metadata "));
    f.render_widget(details, info_chunks[1]);

    let controls = Paragraph::new("←/→: Nav | Space: Auto | R: Re-draw | Q: Quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls, info_chunks[2]);
}

fn get_author_color(author: &str) -> Color {
    let mut hasher = DefaultHasher::new();
    author.hash(&mut hasher);
    let h = hasher.finish();
    match h % 6 {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        5 => Color::Cyan,
        _ => Color::White,
    }
}
