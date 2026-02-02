#![allow(clippy::collapsible_if)]
use anyhow::{Context, Result};
use argh::FromArgs;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap, canvas::Canvas},
};
use std::{fs, io, time::Duration};
use tui_semantic::Snapshot;

/// Semantic Spy - A TUI inspector for tui-semantic snapshots
#[derive(FromArgs)]
struct Args {
    /// path to the snapshot JSON file
    #[argh(option, short = 's')]
    snapshot: String,
}

struct App {
    snapshot: Snapshot,
    list_state: ListState,
    should_quit: bool,
}

impl App {
    fn new(snapshot: Snapshot) -> Self {
        let mut list_state = ListState::default();
        if !snapshot.entities.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            snapshot,
            list_state,
            should_quit: false,
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            KeyCode::Up | KeyCode::Char('k') => self.previous(),
            _ => {}
        }
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.snapshot.entities.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.snapshot.entities.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let content = fs::read_to_string(&args.snapshot)
        .with_context(|| format!("Failed to read snapshot file: {}", args.snapshot))?;
    let snapshot: Snapshot =
        serde_json::from_str(&content).with_context(|| "Failed to parse snapshot JSON")?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(snapshot);
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key.code);
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    // List of entities
    let items: Vec<ListItem> = app
        .snapshot
        .entities
        .iter()
        .map(|e| {
            let id = e.id.as_deref().unwrap_or("?");
            let kind = &e.kind;
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<10}", kind), Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(id, Style::default().fg(Color::Yellow)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Entities "))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::White),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, left_chunks[0], &mut app.list_state);

    // Stats
    let stats = format!(
        "App: {}\nFrame: {}\nEntities: {}",
        app.snapshot.app,
        app.snapshot.frame.unwrap_or(0),
        app.snapshot.entities.len()
    );
    let stats_block =
        Paragraph::new(stats).block(Block::default().borders(Borders::ALL).title(" Stats "));
    f.render_widget(stats_block, left_chunks[1]);

    // Details of selected entity
    if let Some(selected_index) = app.list_state.selected() {
        if let Some(entity) = app.snapshot.entities.get(selected_index) {
            // Visualize props
            let props_json = serde_json::to_string_pretty(&entity.props).unwrap_or_default();
            let details_text = format!(
                "Kind: {}\nID: {}\nPos: {:?}\nVel: {:?}\nDisplay: {:?}\n\nProps:\n{}",
                entity.kind,
                entity.id.as_deref().unwrap_or("None"),
                entity.position,
                entity.velocity,
                entity.display,
                props_json
            );

            let details = Paragraph::new(details_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Entity Details "),
                )
                .wrap(Wrap { trim: true });
            f.render_widget(details, right_chunks[1]);

            // Draw on canvas
            // We need to define the bounds. If the snapshot has viewport, use it.
            let (width, height) = app.snapshot.viewport.unwrap_or((100, 50));

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Spatial View "),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    // Draw all entities as small dots
                    for (i, e) in app.snapshot.entities.iter().enumerate() {
                        if let Some(pos) = e.position {
                            let color = if i == selected_index {
                                Color::Red
                            } else {
                                Color::DarkGray
                            };

                            // Flip Y because Canvas (0,0) is bottom-left, but usually screens are top-left
                            // But wait, the memory said:
                            // "The ratatui Canvas widget uses a Cartesian coordinate system where the origin (0,0) is at the bottom-left, requiring Y-axis inversion when mapping from standard top-left image coordinates."
                            // Orbital decay likely uses top-left as (0,0) for TUI rendering.
                            // So we need to invert Y: y_draw = height - y_actual

                            let y_draw = height as f64 - pos.y;

                            ctx.print(
                                pos.x,
                                y_draw,
                                Span::styled(
                                    e.display.clone().unwrap_or_else(|| "o".to_string()),
                                    Style::default().fg(color),
                                ),
                            );
                        }
                    }

                    // Highlight selected even more?
                    if let Some(pos) = entity.position {
                        let y_draw = height as f64 - pos.y;
                        ctx.print(
                            pos.x,
                            y_draw,
                            Span::styled(
                                entity.display.clone().unwrap_or_else(|| "X".to_string()),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ),
                        );
                    }
                });

            f.render_widget(canvas, right_chunks[0]);
        }
    } else {
        let details = Paragraph::new("No entity selected")
            .block(Block::default().borders(Borders::ALL).title(" Details "));
        f.render_widget(details, right_chunks[1]);

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Spatial View "),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 50.0])
            .paint(|_| {});
        f.render_widget(canvas, right_chunks[0]);
    }
}
