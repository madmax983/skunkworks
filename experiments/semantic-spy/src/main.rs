//! # Semantic Spy
//!
//! `semantic-spy` is a TUI visualizer for **Semantic Bridge** snapshots.
//!
//! It reads a JSON snapshot (conforming to the `tui_semantic` protocol) from either
//! a file or `stdin` and renders an interactive interface to explore it.
//!
//! ## Usage
//!
//! ```bash
//! # From file
//! cargo run --bin semantic-spy snapshot.json
//!
//! # From pipe
//! other-app --semantic | cargo run --bin semantic-spy
//! ```

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::{
    env,
    io::{self, Read},
};
use tui_shared::semantic::Snapshot;

/// Application state for the TUI.
///
/// Holds the loaded snapshot and the UI state (e.g., currently selected entity).
struct App {
    /// The semantic snapshot being visualized.
    snapshot: Snapshot,
    /// State of the entity list widget (selection, scroll).
    list_state: ListState,
}

impl App {
    /// Creates a new `App` with the given snapshot.
    ///
    /// Selects the first entity by default if any exist.
    fn new(snapshot: Snapshot) -> Self {
        let mut list_state = ListState::default();
        if !snapshot.entities.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            snapshot,
            list_state,
        }
    }

    /// Selects the next entity in the list.
    ///
    /// Loops back to the beginning if the end is reached.
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

    /// Selects the previous entity in the list.
    ///
    /// Loops back to the end if the beginning is reached.
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

#[cfg(test)]
mod tests {
    use super::*;
    use tui_shared::semantic::Entity;

    #[test]
    fn test_app_navigation() {
        let snapshot = Snapshot::new("test")
            .with_entity(Entity::new("e1"))
            .with_entity(Entity::new("e2"))
            .with_entity(Entity::new("e3"));

        let mut app = App::new(snapshot);
        assert_eq!(app.list_state.selected(), Some(0));

        app.next();
        assert_eq!(app.list_state.selected(), Some(1));

        app.next();
        assert_eq!(app.list_state.selected(), Some(2));

        app.next();
        assert_eq!(app.list_state.selected(), Some(0)); // Loop back

        app.previous();
        assert_eq!(app.list_state.selected(), Some(2)); // Loop back
    }
}

/// Entry point for the application.
///
/// 1. Reads the snapshot from input.
/// 2. Initializes the terminal.
/// 3. Runs the main event loop.
/// 4. Restores the terminal on exit.
fn main() -> Result<()> {
    // 1. Read input
    let snapshot = read_snapshot()?;

    // 2. Setup Terminal
    let mut tui = tui_shared::Tui::init()?;

    // 3. Run App
    let app = App::new(snapshot);
    let res = run_app(&mut tui.terminal, app);

    // 4. Restore Terminal (handled by Drop)

    if let Err(err) = res {
        tui.exit()?;
        eprintln!("{:?}", err);
    }

    Ok(())
}

/// Reads a `Snapshot` from command-line arguments (file) or stdin.
///
/// # Returns
///
/// * `Ok(Snapshot)` - The parsed snapshot.
/// * `Err` - If reading fails or JSON is invalid.
fn read_snapshot() -> Result<Snapshot> {
    let args: Vec<String> = env::args().collect();
    let mut json = String::new();

    let limit = 10 * 1024 * 1024;
    if args.len() > 1 {
        // Read from file
        let path = &args[1];
        let file = std::fs::File::open(path).context("Failed to open input file")?;
        let bytes_read = file.take(limit + 1)
            .read_to_string(&mut json)
            .context("Failed to read input file")?;
        if bytes_read > limit as usize {
            anyhow::bail!("Input file is too large! Maximum allowed size is 10MB.");
        }
    } else {
        // Read from stdin
        if atty::is(atty::Stream::Stdin) {
            eprintln!("Usage: semantic-spy < snapshot.json");
            eprintln!("   or: cargo run --bin orbital-decay -- --semantic | semantic-spy");
            std::process::exit(1);
        }
        let bytes_read = io::stdin()
            .take(limit + 1)
            .read_to_string(&mut json)
            .context("Failed to read stdin")?;
        if bytes_read > limit as usize {
            anyhow::bail!("Stdin input is too large! Maximum allowed size is 10MB.");
        }
    }

    let snapshot: Snapshot =
        serde_json::from_str(&json).context("Failed to parse JSON snapshot")?;
    Ok(snapshot)
}

/// Runs the main event loop.
///
/// Handles drawing the UI and processing key events.
///
/// * `q` - Quit the application.
/// * `Down` - Select next entity.
/// * `Up` - Select previous entity.
fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

/// Renders the user interface.
///
/// Layout:
/// - **Left Pane (30%)**: List of entities.
/// - **Right Pane (70%)**: Split vertically.
///     - **Top (70%)**: Canvas visualizer showing entity positions.
///     - **Bottom (30%)**: JSON details of the selected entity.
fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let left_pane = chunks[0];
    let right_pane = chunks[1];

    // Left Pane: Entity List
    let items: Vec<ListItem> = app
        .snapshot
        .entities
        .iter()
        .map(|e| {
            let id = e.id.as_deref().unwrap_or("?");
            let kind = &e.kind;
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:<10} ", kind)),
                Span::styled(id, Style::default().fg(Color::Yellow)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Entities "))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, left_pane, &mut app.list_state);

    // Right Pane: Split into Canvas (Top) and Details (Bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(right_pane);

    let canvas_area = right_chunks[0];
    let details_area = right_chunks[1];

    // Canvas
    let (vp_w, vp_h) = app.snapshot.viewport.unwrap_or((80, 40));

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Visualizer "))
        .x_bounds([0.0, vp_w as f64])
        .y_bounds([0.0, vp_h as f64])
        .paint(|ctx| {
            // Draw all entities
            for (i, entity) in app.snapshot.entities.iter().enumerate() {
                if let Some(pos) = entity.position {
                    let is_selected = Some(i) == app.list_state.selected();
                    let color = if is_selected {
                        Color::Cyan
                    } else {
                        Color::White
                    };

                    // Invert Y because TUI (0,0) is top-left but Cartesian is bottom-left usually?
                    let y = vp_h as f64 - pos.y; // Flip Y to match screen coords (0 at top)

                    ctx.print(
                        pos.x,
                        y,
                        Span::styled(
                            entity.display.clone().unwrap_or_else(|| "?".to_string()),
                            Style::default().fg(color),
                        ),
                    );
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Details
    let selected_entity = app
        .list_state
        .selected()
        .and_then(|i| app.snapshot.entities.get(i));

    let details_text = if let Some(entity) = selected_entity {
        serde_json::to_string_pretty(entity).unwrap_or_default()
    } else {
        "Select an entity to view details".to_string()
    };

    let details = Paragraph::new(details_text)
        .block(Block::default().borders(Borders::ALL).title(" Details "))
        .wrap(Wrap { trim: false });

    f.render_widget(details, details_area);
}

mod atty {
    pub enum Stream {
        Stdin,
    }
    pub fn is(_stream: Stream) -> bool {
        use std::io::IsTerminal;
        std::io::stdin().is_terminal()
    }
}
