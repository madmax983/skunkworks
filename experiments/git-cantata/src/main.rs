use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git_cantata::audio::AudioEngine;
use git_cantata::git::{get_repo_history, CommitData};
use git_cantata::vis::VisualState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::{
    env,
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct App {
    commits: Vec<CommitData>,
    current_idx: usize,
    audio: AudioEngine,
    vis: VisualState,
    paused: bool,
    last_commit_change: Instant,
    commit_duration: Duration,
    list_state: ListState,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let commits = get_repo_history(path, 100)?; // Limit 100
        let audio = AudioEngine::new();
        let vis = VisualState::new(200.0, 100.0); // Canvas logical size

        Ok(Self {
            commits,
            current_idx: 0,
            audio,
            vis,
            paused: false,
            last_commit_change: Instant::now(),
            commit_duration: Duration::from_secs(4),
            list_state: ListState::default(),
        })
    }

    fn on_tick(&mut self, dt: f64) {
        self.vis.update(dt);

        if !self.paused {
            if self.last_commit_change.elapsed() >= self.commit_duration {
                self.next_commit();
            }
        }
    }

    fn next_commit(&mut self) {
        if self.commits.is_empty() {
            return;
        }

        self.current_idx = (self.current_idx + 1) % self.commits.len();
        self.last_commit_change = Instant::now();
        self.play_current();
    }

    fn prev_commit(&mut self) {
        if self.commits.is_empty() {
            return;
        }

        if self.current_idx == 0 {
            self.current_idx = self.commits.len() - 1;
        } else {
            self.current_idx -= 1;
        }
        self.last_commit_change = Instant::now();
        self.play_current();
    }

    fn play_current(&mut self) {
        if let Some(commit) = self.commits.get(self.current_idx) {
            self.audio.play_commit(commit);
            self.vis.spawn_commit(commit);
            self.list_state.select(Some(self.current_idx));
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    let mut tui = Tui::init()?;
    let mut app = App::new(path)?;

    // Start playing first commit immediately
    app.play_current();

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
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
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Right | KeyCode::Char('n') => app.next_commit(),
                        KeyCode::Left | KeyCode::Char('p') => app.prev_commit(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            app.on_tick(dt);
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(12)])
        .split(f.area());

    // Top: Visualizer
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" The Song of the Commit 🎶 "),
        )
        .x_bounds([0.0, app.vis.width])
        .y_bounds([0.0, app.vis.height])
        .paint(|ctx| {
            app.vis.draw(ctx);
        });
    f.render_widget(canvas, chunks[0]);

    // Bottom: Commit Info & List
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Commit List
    let items: Vec<ListItem> = app
        .commits
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let style = if i == app.current_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(ratatui::style::Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(
                format!("{} - {}", &c.hash[..7], c.author),
                style,
            ))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" History "))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, bottom_chunks[0]);

    // Current Commit Details
    if let Some(commit) = app.commits.get(app.current_idx) {
        let text = vec![
            Line::from(vec![
                Span::styled("Hash: ", Style::default().fg(Color::Gray)),
                Span::raw(&commit.hash),
            ]),
            Line::from(vec![
                Span::styled("Author: ", Style::default().fg(Color::Gray)),
                Span::raw(&commit.author),
            ]),
            Line::from(vec![
                Span::styled("Date: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{}", commit.timestamp)),
            ]),
            Line::from(vec![
                Span::styled("Files: ", Style::default().fg(Color::Gray)),
                Span::raw(commit.changes.len().to_string()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Message: ", Style::default().fg(Color::Gray)),
                Span::raw(&commit.message),
            ]),
        ];

        let p = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Detail "))
            .wrap(Wrap { trim: true });
        f.render_widget(p, bottom_chunks[1]);
    }
}
