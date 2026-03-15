mod app;
mod blame;
mod simulation;

use anyhow::Result;
use app::App;
use blame::BlameAnalyzer;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{env, io, path::Path, time::Duration};

use crate::simulation::Simulation;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chron-compost <file_path>");
        return Ok(());
    }
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);

    // Analyze
    println!("Analyzing {}...", file_path_str);
    let analyzer = BlameAnalyzer::new(".");
    let blame_info = analyzer.analyze(file_path)?;

    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App State
    let app = App::new(file_path_str.to_string(), blame_info.clone())?;

    // Sim State
    let max_len = app.content.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut sim = Simulation::new(blame_info, app.content.len(), max_len)?;

    // Run Loop
    let mut scroll_y = 0;
    loop {
        sim.tick();

        terminal.draw(|f| {
            draw(f, &app, &sim, scroll_y);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Down | KeyCode::Char('j') => scroll_y += 1,
                        KeyCode::Up | KeyCode::Char('k') => {
                            if scroll_y > 0 {
                                scroll_y -= 1;
                            }
                        }
                        KeyCode::Char('r') => {
                            sim.spawn_agents(5);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw(f: &mut Frame, app: &App, sim: &Simulation, scroll_y: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Draw Source Code
    let mut text = Vec::new();
    let start_idx = scroll_y;
    let view_height = chunks[0].height as usize;

    for i in start_idx..std::cmp::min(app.content.len(), start_idx + view_height) {
        let line_content = &app.content[i];
        let age_score = sim.blame_info.get(i).map_or(1.0, |info| info.age_score);

        // Color mapped to age (like chrontext)
        let color = if age_score > 0.8 {
            Color::Yellow // Hot
        } else if age_score > 0.5 {
            Color::Green // Warm
        } else if age_score > 0.2 {
            Color::Cyan // Cool
        } else {
            Color::DarkGray // Cold
        };

        // We need to construct a line of spans, interleaving agents if any are on this line
        let mut chars = line_content.chars();

        // Apply agents
        let mut spans = Vec::new();
        let mut current_span_str = String::new();

        for x in 0..sim.width {
            let c = chars.next().unwrap_or(' ');

            // Check if agent is here
            let agent_here = sim.agents.iter().find(|a| a.y == i && a.x == x);

            if let Some(_agent) = agent_here {
                // Push existing string
                if !current_span_str.is_empty() {
                    spans.push(Span::styled(
                        current_span_str.clone(),
                        Style::default().fg(color),
                    ));
                    current_span_str.clear();
                }
                // Push agent
                spans.push(Span::styled(
                    "@",
                    Style::default().fg(Color::Red).bg(Color::Black),
                ));
            } else {
                current_span_str.push(c);
            }
        }
        if !current_span_str.is_empty() {
            spans.push(Span::styled(current_span_str, Style::default().fg(color)));
        }

        text.push(Line::from(spans));
    }

    let code_block = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Time Scavengers "),
        )
        .style(Style::default());

    f.render_widget(code_block, chunks[0]);

    // Status bar
    let status_str = format!(
        " Population: {} | File: {} | [j/k] Scroll | [r] Respawn | [q] Quit ",
        sim.agents.len(),
        app.path
    );
    let status_block = Paragraph::new(status_str)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(status_block, chunks[1]);
}
