mod git;
mod physics;
mod spectral;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use physics::Universe;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

use git::{get_commit_history, Commit};

struct App {
    universe: Universe,
    commits: Vec<Commit>,
    current_index: usize,
    resonance_dist: f64,
}

impl App {
    fn new() -> Self {
        let commits = match get_commit_history() {
            Ok(c) if !c.is_empty() => c,
            _ => vec![Commit {
                hash: "0000000000000000000000000000000000000000".to_string(),
                author: "Nova".to_string(),
                message: "No git history found (or error).".to_string(),
            }],
        };

        Self {
            universe: Universe::new(100.0, 50.0), // TUI size roughly
            commits,
            current_index: 0,
            resonance_dist: 10.0,
        }
    }

    fn next_commit(&mut self) {
        if self.current_index + 1 < self.commits.len() {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
    }

    fn prev_commit(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.commits.len() - 1;
        }
    }

    fn update(&mut self) {
        let commit = &self.commits[self.current_index];
        // Calculate resonance_dist based on the first two characters of the hash
        let hash_prefix = &commit.hash[0..2];
        let hash_val = u8::from_str_radix(hash_prefix, 16).unwrap_or(128) as f64;

        // Map 0-255 to roughly 2.0 to 20.0
        self.resonance_dist = 2.0 + (hash_val / 255.0) * 18.0;

        self.universe.update(0.016, self.resonance_dist);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B::Error: std::marker::Send + std::marker::Sync + 'static,
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[1]);

            // Header
            let title = Paragraph::new(" Git Hologram: Codebase Spectral Signatures ")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Left: Spatial (Particles)
            let canvas_spatial = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Holographic Swarm "))
                .x_bounds([0.0, app.universe.width])
                .y_bounds([0.0, app.universe.height])
                .marker(ratatui::symbols::Marker::Braille)
                .paint(|ctx| {
                    let mut points = Vec::new();
                    for p in &app.universe.particles {
                        // Flip Y for TUI
                        points.push((p.pos.x, app.universe.height - p.pos.y));
                    }
                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Blue,
                    });
                });
            f.render_widget(canvas_spatial, main_chunks[0]);

            // Right: Commit Info
            let commit = &app.commits[app.current_index];
            let info_text = vec![
                Line::from(Span::styled("Commit:", Style::default().fg(Color::Gray))),
                Line::from(Span::raw(format!("{}/{}", app.current_index + 1, app.commits.len()))),
                Line::from(""),
                Line::from(Span::styled("Hash:", Style::default().fg(Color::Gray))),
                Line::from(Span::styled(&commit.hash, Style::default().fg(Color::Yellow))),
                Line::from(""),
                Line::from(Span::styled("Author:", Style::default().fg(Color::Gray))),
                Line::from(Span::raw(&commit.author)),
                Line::from(""),
                Line::from(Span::styled("Message:", Style::default().fg(Color::Gray))),
                Line::from(Span::raw(&commit.message)),
                Line::from(""),
                Line::from(Span::styled("Resonance Filter:", Style::default().fg(Color::Magenta))),
                Line::from(Span::raw(format!("{:.2} Hz", app.resonance_dist))),
            ];
            let info_panel = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL).title(" Spectral Metadata "));
            f.render_widget(info_panel, main_chunks[1]);

            // Footer
            let footer = Paragraph::new(" [←/→] Navigate Commits | [Q] Quit ")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left | KeyCode::Char('h') => app.prev_commit(),
                        KeyCode::Right | KeyCode::Char('l') => app.next_commit(),
                        _ => {}
                    }
                }
            }
        }

        app.update();
    }
}