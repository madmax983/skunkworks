mod git;
mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git::{get_commit_history, Commit};
use physics::Universe;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct App {
    commits: Vec<Commit>,
    current_index: usize,
    universe: Universe,
}

impl App {
    fn new() -> Result<Self> {
        let commits = match get_commit_history() {
            Ok(c) if !c.is_empty() => c,
            _ => vec![Commit {
                hash: "0000000000000000000000000000000000000000".to_string(),
                author: "Nova".to_string(),
                message: "No git history found.".to_string(),
            }],
        };

        let mut app = Self {
            commits,
            current_index: 0,
            universe: Universe::new(200.0, 100.0),
        };
        app.update_magnet();
        Ok(app)
    }

    fn update_magnet(&mut self) {
        let commit = &self.commits[self.current_index];

        // Hash to coords
        let get_val = |start: usize, count: usize| -> f64 {
            if start + count > commit.hash.len() {
                return 0.5;
            }
            let slice = &commit.hash[start..start + count];
            let val = u32::from_str_radix(slice, 16).unwrap_or(0);
            val as f64
        };

        // x [0.0, 200.0]
        let x = (get_val(0, 3) / 4096.0) * 200.0;
        // y [0.0, 100.0]
        let y = (get_val(3, 3) / 4096.0) * 100.0;

        #[allow(clippy::manual_is_multiple_of)]
        let polarity = (get_val(6, 1) as u32) % 2 == 0; // True if even

        self.universe.add_magnet(x, y, polarity);

        // Keep only last 3 magnets to avoid chaotic noise overwhelming structure
        if self.universe.magnets.len() > 3 {
            self.universe.magnets.remove(0);
        }
    }

    fn next(&mut self) {
        if self.current_index + 1 < self.commits.len() {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
        self.update_magnet();
    }

    fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.commits.len() - 1;
        }
        self.update_magnet();
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    // Tick rate
    let tick_rate = Duration::from_millis(33); // 30 FPS for fluid
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
                        KeyCode::Char('r') => {
                            app.universe = Universe::new(200.0, 100.0);
                            app.update_magnet();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.universe.update(0.05); // using update based on ferrous-fluid physics.rs
            last_tick = Instant::now();
        }
    }

    tui.exit()?;
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
        .title(" Magnetic Codebase Fingerprint ");

    let canvas = Canvas::default()
        .block(canvas_block)
        .x_bounds([0.0, app.universe.width])
        .y_bounds([0.0, app.universe.height])
        .paint(|ctx| {
            // Draw Particles
            for p in &app.universe.particles {
                ctx.print(
                    p.pos.x,
                    p.pos.y,
                    ratatui::text::Span::styled("·", Style::default().fg(Color::Cyan)),
                );
            }

            // Draw Magnets
            for mag in &app.universe.magnets {
                let color = if mag.polarity {
                    Color::Red
                } else {
                    Color::Blue
                };
                let label = if mag.polarity { "N" } else { "S" };
                ctx.print(
                    mag.pos.x,
                    mag.pos.y,
                    ratatui::text::Span::styled(label, Style::default().fg(color).bg(Color::White)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Right: Info
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Details
            Constraint::Length(4), // Controls
        ])
        .split(chunks[1]);

    let title = Paragraph::new(vec![
        ratatui::text::Line::from(vec![ratatui::text::Span::styled(
            "Git Fluid",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        ratatui::text::Line::from(vec![ratatui::text::Span::raw(format!(
            "Commit {}/{} | Magnets: {}",
            app.current_index + 1,
            app.commits.len(),
            app.universe.magnets.len()
        ))]),
    ])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, info_chunks[0]);

    let commit = &app.commits[app.current_index];
    let details_text = vec![
        ratatui::text::Line::from(ratatui::text::Span::styled(
            "Hash:",
            Style::default().fg(Color::Gray),
        )),
        ratatui::text::Line::from(ratatui::text::Span::raw(&commit.hash)),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(ratatui::text::Span::styled(
            "Author:",
            Style::default().fg(Color::Gray),
        )),
        ratatui::text::Line::from(ratatui::text::Span::styled(
            &commit.author,
            Style::default().fg(Color::Green),
        )),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(ratatui::text::Span::styled(
            "Message:",
            Style::default().fg(Color::Gray),
        )),
        ratatui::text::Line::from(ratatui::text::Span::raw(&commit.message)),
    ];

    let details = Paragraph::new(details_text)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" Metadata "));
    f.render_widget(details, info_chunks[1]);

    let controls = Paragraph::new("←/→: Nav | R: Reset Fluid | Q: Quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls, info_chunks[2]);
}
