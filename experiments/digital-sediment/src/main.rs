mod git;
mod entropy;
mod recovery;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::{error::Error, io, time::{Duration, Instant}};

use git::{CommitInfo, RepoHandler};
use entropy::EntropyEngine;
use recovery::RecoveryEngine;

struct App {
    repo: RepoHandler,
    commits: Vec<CommitInfo>,
    list_state: ListState,
    file_content: String,
    recovery_mode: bool,
    decay_enabled: bool,
    filepath: String,
}

impl App {
    fn new() -> Result<Self> {
        let repo = RepoHandler::new(".")?;
        let commits = repo.list_commits(50)?;
        let mut list_state = ListState::default();
        if !commits.is_empty() {
            list_state.select(Some(0));
        }

        // Initial file load
        let filepath = "README.md".to_string(); // Default file to view
        let file_content = if !commits.is_empty() {
            repo.get_file_content(&commits[0].id, &filepath).unwrap_or_default()
        } else {
            String::new()
        };

        Ok(Self {
            repo,
            commits,
            list_state,
            file_content,
            recovery_mode: false,
            decay_enabled: true,
            filepath,
        })
    }

    fn on_tick(&mut self) {
        // Optional: dynamic decay updates?
    }

    fn next(&mut self) {
        if self.commits.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.commits.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_content();
    }

    fn previous(&mut self) {
        if self.commits.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.commits.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_content();
    }

    fn update_content(&mut self) {
        if let Some(i) = self.list_state.selected() {
            let commit_id = &self.commits[i].id;
            // Try to find README.md or src/main.rs or src/lib.rs
            let content = self.repo.get_file_content(commit_id, &self.filepath).unwrap_or_else(|_| "File not found".to_string());
            self.file_content = content;
        }
    }

    fn toggle_recovery(&mut self) {
        self.recovery_mode = !self.recovery_mode;
    }

    fn toggle_decay(&mut self) {
        self.decay_enabled = !self.decay_enabled;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let app = App::new();

    if let Err(e) = app {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        eprintln!("Error initializing app: {}", e);
        return Err(e.into());
    }

    let mut app = app.unwrap();

    // Run loop
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('r') => app.toggle_recovery(),
                    KeyCode::Char('d') => app.toggle_decay(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    // Left Pane: Sediment Layers (Commits)
    let items: Vec<ListItem> = app
        .commits
        .iter()
        .enumerate()
        .map(|(i, commit)| {
            // Calculate visual decay for the list item itself?
            // Maybe color gradient. Newer (top) is bright, older (bottom) is dim.
            // But list is rendered by index.
            // Let's just use white for now.
            let style = if i == app.list_state.selected().unwrap_or(0) {
                 Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                 Style::default().fg(Color::Gray)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", &commit.id[..7]), Style::default().fg(Color::Cyan)),
                Span::raw(&commit.message),
            ])).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Sediment Layers "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    // Right Pane: Excavation Site (File Content)
    // Determine decay factor based on depth
    let depth = app.list_state.selected().unwrap_or(0);
    let max_depth = app.commits.len().max(1) as f64;
    let decay_factor = if app.decay_enabled {
        (depth as f64 / max_depth).min(1.0)
    } else {
        0.0
    };

    let content_text = if app.recovery_mode {
        // Run recovery
        // First corrupt it (simulation of what we found)
        let corrupted = EntropyEngine::corrupt(&app.file_content, decay_factor);
        // Then try to recover
        let tokens = RecoveryEngine::recover(&corrupted);

        // Let's iterate tokens and build Lines.
        let mut lines = Vec::new();
        let mut current_spans = Vec::new();

        for (text, recovered) in tokens {
             let parts: Vec<&str> = text.split('\n').collect();
             for (j, part) in parts.iter().enumerate() {
                 if j > 0 {
                     lines.push(Line::from(current_spans.clone()));
                     current_spans.clear();
                 }
                 if !part.is_empty() {
                     if recovered {
                         current_spans.push(Span::styled(part.to_string(), Style::default().fg(Color::Green)));
                     } else {
                         current_spans.push(Span::styled(part.to_string(), Style::default().fg(Color::DarkGray)));
                     }
                 }
             }
        }
        if !current_spans.is_empty() {
            lines.push(Line::from(current_spans));
        }
        lines
    } else {
        let corrupted = EntropyEngine::corrupt(&app.file_content, decay_factor);
        corrupted.lines().map(|l| Line::from(l.to_string())).collect()
    };

    let paragraph = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL).title(format!(" Excavation Site (Decay: {:.2}) ", decay_factor)))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, chunks[1]);
}
