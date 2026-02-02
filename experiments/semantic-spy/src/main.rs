use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::Canvas,
        Block, Borders, List, ListItem, ListState, Paragraph, Wrap,
    },
    Terminal,
};
use std::{
    env,
    io::{self, Read},
};
use tui_semantic::Snapshot;

struct App {
    snapshot: Snapshot,
    list_state: ListState,
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

#[cfg(test)]
mod tests {
    use super::*;
    use tui_semantic::Entity;

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

fn main() -> Result<()> {
    // 1. Read input
    let snapshot = read_snapshot()?;

    // 2. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Run App
    let app = App::new(snapshot);
    let res = run_app(&mut terminal, app);

    // 4. Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn read_snapshot() -> Result<Snapshot> {
    let args: Vec<String> = env::args().collect();
    let mut json = String::new();

    if args.len() > 1 {
        // Read from file
        let path = &args[1];
        json = std::fs::read_to_string(path).context("Failed to read input file")?;
    } else {
        // Read from stdin
        if atty::is(atty::Stream::Stdin) {
            eprintln!("Usage: semantic-spy < snapshot.json");
            eprintln!("   or: cargo run --bin orbital-decay -- --semantic | semantic-spy");
            std::process::exit(1);
        }
        io::stdin()
            .read_to_string(&mut json)
            .context("Failed to read stdin")?;
    }

    let snapshot: Snapshot = serde_json::from_str(&json).context("Failed to parse JSON snapshot")?;
    Ok(snapshot)
}

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
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
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
                    let color = if is_selected { Color::Cyan } else { Color::White };

                    // Invert Y because TUI (0,0) is top-left but Cartesian is bottom-left usually?
                    // Wait, ratatui Canvas 0,0 is bottom-left.
                    // TUI chars are top-left.
                    // Let's assume standard Cartesian for Canvas.
                    // But if the app uses screen coords (0,0 top-left), we might need to flip.
                    // Usually simulations use Cartesian. orbital-decay uses (0,0) top-left for TUI.
                    // But wait, orbital-decay's TUI drawing:
                    // cell.set_char('·')...
                    // It uses row/col.

                    // If I use Canvas, I need to map it correctly.
                    // Let's assume the snapshot positions are consistent with the viewport.
                    // If I draw it upside down, so be it for now.

                    // Actually, let's flip Y if it looks wrong.
                    // For now, raw coords.

                    let y = vp_h as f64 - pos.y; // Flip Y to match screen coords (0 at top)

                    ctx.print(pos.x, y, Span::styled(
                        entity.display.clone().unwrap_or_else(|| "?".to_string()),
                        Style::default().fg(color),
                    ));
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

// Minimal atty check since we don't want to add a dependency just for this if we can avoid it.
// Actually, `crossterm` might have `is_tty`? No.
// I'll add `atty` to Cargo.toml or just rely on failing to read stdin.
// Actually, `std::io::stdin().read_to_string` will hang if it's a TTY.
// I'll add `is-terminal` or `atty`.
// Wait, "Nova avoids massive dependencies". `is-terminal` is small.
// But I can't add dependencies without updating Cargo.toml.
// Let's just try to read. If it hangs, user knows they messed up.
// Or I can use `std::io::IsTerminal` if on Rust 1.70+.
// Let's check rust version.
// "edition = 2021" is set.

mod atty {
    pub enum Stream {
        Stdin,
    }
    pub fn is(_stream: Stream) -> bool {
        use std::io::IsTerminal;
        std::io::stdin().is_terminal()
    }
}
