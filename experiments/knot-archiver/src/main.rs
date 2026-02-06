mod quipu;
mod serde_impl;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Terminal,
};
use serde_impl::to_quipu;
use std::{env, fs, io, time::Duration};

fn main() -> Result<()> {
    // 1. Load Data
    let args: Vec<String> = env::args().collect();
    let input_value: serde_json::Value = if args.len() > 1 {
        let content = fs::read_to_string(&args[1])?;
        serde_json::from_str(&content)?
    } else {
        // Default "Ancient" data
        serde_json::json!({
            "empire": "Inca",
            "period": "Late Horizon",
            "census": {
                "population": 12000000,
                "provinces": 4,
                "warehouses": [10, 20, 5, 100],
                "details": {
                    "crops": ["Maize", "Potato", "Quinoa"],
                    "llama_count": 5000
                }
            }
        })
    };

    // 2. Serialize to Quipu
    let quipu = to_quipu(&input_value).map_err(|e| anyhow::anyhow!(e))?;
    let quipu_text = format!("{}", quipu);
    let line_count = quipu_text.lines().count();

    // 3. TUI Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 4. Run Loop
    let mut vertical_scroll = 0;
    let mut scroll_state = ScrollbarState::new(line_count).position(vertical_scroll);

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(size);

            // Title
            let title = Paragraph::new("⚛️  GENESIS: KNOT ARCHIVER ⚛️")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Quipu View
            let content = Paragraph::new(quipu_text.clone())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title(" Quipu Artifact "))
                .scroll((vertical_scroll as u16, 0));
            f.render_widget(content, chunks[1]);

            // Scrollbar
            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                chunks[1],
                &mut scroll_state,
            );

            // Footer
            let footer = Paragraph::new("Controls: [↑/↓] Scroll | [Q] Quit")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down => {
                            vertical_scroll = vertical_scroll.saturating_add(1);
                            // Simple clamp
                            if vertical_scroll > line_count {
                                vertical_scroll = line_count;
                            }
                            scroll_state = scroll_state.position(vertical_scroll);
                        }
                        KeyCode::Up => {
                            vertical_scroll = vertical_scroll.saturating_sub(1);
                            scroll_state = scroll_state.position(vertical_scroll);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // 5. Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}
