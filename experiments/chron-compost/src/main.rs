//! # 🧬 Chron Compost
//!
//! **"Time Scavengers consuming the ancient bedrock."**
//!
//! A hybrid experiment combining **Chrontext** (Git blame timeline visualization) and **Compost Chimera** (Chimera VM biological agents feeding on text).
//!
//! ## 🧬 Lineage
//!
//! - **Parent A**: `experiments/chrontext` (The Environment)
//!   - Provided the `BlameAnalyzer` which uses `git2` to analyze the age of lines of code.
//!   - Provided the visual rendering mapping age scores to colors (Cold/Warm/Hot).
//!
//! - **Parent B**: `experiments/compost-chimera` (The Life)
//!   - Provided the `ChimeraVM` agents.
//!   - Provided the core TUI simulation loop.
//!
//! ## 🧪 Phenotype
//!
//! **Time Scavengers**. Small agents (`@`) roam across the buffers of your source code. The code itself is mapped to its `git blame` history, where older code is colored blue/gray ("Cold") and newly committed code is yellow ("Hot").
//!
//! The agents feed on the "Cold" lines of code—the ancient bedrock of the codebase. As they consume it, they "refactor" the codebase history, resetting the age score of the line back to 1.0 (Hot/Fresh) and gaining energy in the process.
//!
//! ## 🎮 Controls
//!
//! - `j/k/Down/Up`: Scroll the viewport.
//! - `r`: Respawn agents.
//! - `q/Esc`: Quit.
//!
//! ## 🏗️ Architecture
//!
//! The simulation runs a TUI loop where `Simulation` manages a population of `Agent`s. Each agent runs an internal `ChimeraVM`. During each tick, the environment feeds the `age_score` of the current line to the agent's stack. If the `age_score` is low enough (old code), the agent "eats" it, mutating the underlying `blame_info` struct to reset the age score and simulate a refactoring event.
//!
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
    let app = App::new(file_path_str.to_string())?;

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
                            scroll_y = scroll_y.saturating_sub(1);
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
