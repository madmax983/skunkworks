use crate::decay::apply_decay;
use crate::ghost::{fetch_ectoplasm, scan_graveyard, Ghost};
use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode};
use git2::Repository;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::time::Duration;

pub struct App {
    pub ghosts: Vec<Ghost>,
    pub list_state: ListState,
    pub content_cache: Option<(usize, String)>,
    pub repo_path: String,
    pub message: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let repo_path = ".";
        // Try to discover repo
        let ghosts = match scan_graveyard(repo_path, 100) {
            Ok(g) => g,
            Err(_) => Vec::new(), // Handle error gracefully or let it crash?
        };

        let mut state = ListState::default();
        if !ghosts.is_empty() {
            state.select(Some(0));
        }
        Ok(Self {
            ghosts,
            list_state: state,
            content_cache: None,
            repo_path: repo_path.to_string(),
            message: "Welcome to the Digital Graveyard. Select a ghost to commune.".to_string(),
        })
    }

    pub fn next(&mut self) {
        if self.ghosts.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.ghosts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.content_cache = None;
    }

    pub fn previous(&mut self) {
        if self.ghosts.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.ghosts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.content_cache = None;
    }

    pub fn resurrect(&mut self) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(ghost) = self.ghosts.get(i) {
                let repo = Repository::discover(&self.repo_path)?;
                let content = fetch_ectoplasm(&repo, ghost)?;
                let filename = std::path::Path::new(&ghost.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or("unknown");
                let out_path = format!("resurrected_{}", filename);
                std::fs::write(&out_path, content)?;
                self.message = format!("Resurrected {} to {}", ghost.path, out_path);
            }
        }
        Ok(())
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    loop {
        // Load content if needed
        if app.content_cache.is_none() {
            if let Some(i) = app.list_state.selected() {
                let content = generate_content(&app, i);
                app.content_cache = Some((i, content));
            }
        }

        terminal.draw(|f| ui(f, &mut app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('t') | KeyCode::Enter => {
                        if let Err(e) = app.resurrect() {
                            app.message = format!("Resurrection failed: {}", e);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.area());

    // List of ghosts
    let items: Vec<ListItem> = app
        .ghosts
        .iter()
        .map(|g| {
            let age_secs = (Utc::now() - g.deleted_at).num_seconds();
            let color = if age_secs < 86400 {
                Color::White
            } else if age_secs < 2592000 {
                // 30 days
                Color::Gray
            } else {
                Color::DarkGray
            };

            ListItem::new(format!("{} ({})", g.path, g.deleted_at.format("%Y-%m-%d")))
                .style(Style::default().fg(color))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Graveyard"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    // Content Pane
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Manifestation");
    f.render_widget(block, chunks[1]);

    let inner_area = chunks[1].inner(Margin {
        vertical: 1,
        horizontal: 1,
    });

    if let Some((_, content)) = &app.content_cache {
        // Render lines with CRT scanline effect
        let mut text_lines = Vec::new();
        for (idx, line_str) in content.lines().enumerate() {
            let bg = if idx % 2 == 0 {
                Color::Black
            } else {
                Color::Rgb(20, 20, 20)
            };
            let span = Span::styled(line_str, Style::default().fg(Color::Green).bg(bg));
            text_lines.push(Line::from(span));
        }

        let p = Paragraph::new(text_lines)
            .wrap(Wrap { trim: false })
            .scroll((0, 0)); // Could add scrolling later

        f.render_widget(p, inner_area);
    }

    // Message overlay (bottom left of screen)
    if !app.message.is_empty() {
        let area = f.area();
        let msg_area = ratatui::layout::Rect {
            x: 0,
            y: area.height - 1,
            width: area.width,
            height: 1,
        };
        f.render_widget(
            Paragraph::new(Span::styled(&app.message, Style::default().fg(Color::Red))),
            msg_area,
        );
    }
}

fn generate_content(app: &App, index: usize) -> String {
    if index >= app.ghosts.len() {
        return String::new();
    }
    let ghost = &app.ghosts[index];

    // Open repo temporarily (not efficient but safe)
    let repo = match Repository::discover(&app.repo_path) {
        Ok(r) => r,
        Err(_) => return "Error: Cannot open repository.".to_string(),
    };

    match fetch_ectoplasm(&repo, ghost) {
        Ok(raw) => {
            let age = (Utc::now() - ghost.deleted_at).num_seconds();
            apply_decay(&raw, age)
        }
        Err(e) => format!("Error summoning ghost: {}", e),
    }
}
