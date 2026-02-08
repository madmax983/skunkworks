mod ghost;
mod decay;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::Serialize;
use std::io;
use quipu_serializer::quipu::{Quipu, Knot};
use quipu_serializer::ser::to_quipu;
use chrono::Utc;

#[derive(Serialize)]
struct GhostRecord {
    path: String,
    deleted_at: String,
    author: String,
}

struct App {
    ghosts: Vec<(ghost::Ghost, Quipu)>,
    scroll_x: usize, // Which cord (ghost) is selected
    scroll_y: u16, // Vertical scroll on the cord view
}

impl App {
    fn new() -> Result<Self> {
        // Scan current directory
        let raw_ghosts = ghost::scan_graveyard(".", 20)?;
        let mut ghosts = Vec::new();

        for g in raw_ghosts {
            let record = GhostRecord {
                path: g.path.clone(),
                deleted_at: g.deleted_at.format("%Y-%m-%d").to_string(),
                author: g.author.clone(),
            };

            // Serialize metadata to Quipu
            let quipu = to_quipu(&record).unwrap_or(Quipu { cords: vec![] });
            ghosts.push((g, quipu));
        }

        // Sort by deleted_at desc
        ghosts.sort_by(|a, b| b.0.deleted_at.cmp(&a.0.deleted_at));

        Ok(Self {
            ghosts,
            scroll_x: 0,
            scroll_y: 0,
        })
    }

    fn next(&mut self) {
        if !self.ghosts.is_empty() {
            self.scroll_x = (self.scroll_x + 1) % self.ghosts.len();
        }
    }

    fn previous(&mut self) {
        if !self.ghosts.is_empty() {
            if self.scroll_x == 0 {
                self.scroll_x = self.ghosts.len() - 1;
            } else {
                self.scroll_x -= 1;
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                KeyCode::Down => app.scroll_y = app.scroll_y.saturating_add(1),
                KeyCode::Up => app.scroll_y = app.scroll_y.saturating_sub(1),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let header = Paragraph::new("🧬 GHOST-KNOTS: The Quipu of Lost Code")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    if app.ghosts.is_empty() {
        f.render_widget(Paragraph::new("No ghosts found.").block(Block::default().borders(Borders::ALL)), chunks[1]);
        return;
    }

    // Main view: Display cords hanging down
    // We want to visualize multiple cords if they fit, but highlight the selected one.

    let main_area = chunks[1];
    let cords_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_area);

    // Left panel: The Hanging Garden of Knots
    render_quipu_garden(f, cords_area[0], app);

    // Right panel: Details of selected ghost
    render_ghost_details(f, cords_area[1], app);

    let footer = Paragraph::new("Arrows: Navigate | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_quipu_garden(f: &mut Frame, area: Rect, app: &App) {
    let (ghost, quipu) = &app.ghosts[app.scroll_x];

    // Calculate decay color
    let age_seconds = (Utc::now() - ghost.deleted_at).num_seconds();
    let max_age = 31536000.0; // 1 year
    let decay_factor = (age_seconds as f64 / max_age).clamp(0.0, 1.0);

    let base_color = if decay_factor > 0.8 {
        Color::DarkGray
    } else if decay_factor > 0.5 {
        Color::Gray
    } else {
        Color::Yellow
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Quipu for: {} ", ghost.path));

    let mut lines = Vec::new();

    // Find max depth (length of longest cord)
    let max_depth = quipu.cords.iter().map(|c| c.clusters.len()).max().unwrap_or(0);

    // Header line (Main Cord)
    let mut header_line = String::new();
    for _ in 0..quipu.cords.len() {
        header_line.push_str("══╤══ ");
    }
    lines.push(Line::from(Span::styled(header_line, Style::default().fg(base_color))));

    for depth in 0..max_depth {
        let mut line_spans = Vec::new();
        // Iterate cords

        for cord in &quipu.cords {
            // Get cluster at this depth (from top)
            // Depth 0 = Top = clusters[len-1]
            // Depth k = clusters[len-1-k]

            if depth >= cord.clusters.len() {
                line_spans.push(Span::raw("  |   "));
            } else {
                let cluster_idx = cord.clusters.len() - 1 - depth;
                let cluster = &cord.clusters[cluster_idx];

                let mut cluster_str = String::new();
                if cluster.is_empty() {
                    cluster_str.push_str("  |  ");
                } else {
                    for knot in cluster {
                        let symbol = match knot {
                             Knot::Simple => "●",
                             Knot::Long(_v) => "≡", // Simplification
                             Knot::FigureEight => "∞",
                        };
                         cluster_str.push_str(symbol);
                    }
                    // Pad
                     while cluster_str.chars().count() < 5 {
                         cluster_str.push(' ');
                     }
                }

                // Apply glitch if decayed
                if decay_factor > 0.3 && rand::random::<f64>() < decay_factor * 0.2 {
                    cluster_str = decay::apply_decay(&cluster_str, age_seconds).trim().to_string();
                }

                line_spans.push(Span::styled(format!("{:5} ", cluster_str), Style::default().fg(base_color)));
            }
        }
        lines.push(Line::from(line_spans));
    }

    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .scroll((app.scroll_y, 0)),
        area,
    );
}

fn render_ghost_details(f: &mut Frame, area: Rect, app: &App) {
    let (ghost, _) = &app.ghosts[app.scroll_x];

    let text = vec![
        Line::from(vec![Span::styled("Path: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(&ghost.path)]),
        Line::from(vec![Span::styled("Deleted: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(ghost.deleted_at.to_string())]),
        Line::from(vec![Span::styled("Author: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(&ghost.author)]),
        Line::from(vec![Span::styled("Commit: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(&ghost.commit_hash[..7])]),
        Line::from(vec![
            Span::styled("Content: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("\"{}...\"", ghost.content_snippet.replace('\n', " ")))
        ]),
        Line::from(""),
        Line::from(Span::styled("The Quipu knots encode the file path, deletion date, and author name as a sequence of numbers (ASCII values).", Style::default().fg(Color::DarkGray))),
    ];

    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Ghost Details "))
            .wrap(Wrap { trim: true }),
        area,
    );
}
