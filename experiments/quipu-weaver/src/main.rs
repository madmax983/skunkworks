use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quipu_weaver::quipu::Quipu;
use quipu_weaver::serializer::to_quipu;
use quipu_weaver::tui::QuipuWidget;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input JSON file
    #[arg(short, long)]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load data
    let quipu = if let Some(path) = args.input {
        let content = fs::read_to_string(path)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;
        to_quipu(&json).map_err(|e| anyhow::anyhow!(e.to_string()))?
    } else {
        // Default Demo Data
        let json = serde_json::json!({
            "project": "Quipu Weaver",
            "year": 2025,
            "moonshot": true,
            "stats": {
                "commits": 42,
                "complexity": 9999
            }
        });
        to_quipu(&json).map_err(|e| anyhow::anyhow!(e.to_string()))?
    };

    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, quipu);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, quipu: Quipu) -> Result<()> {
    let mut scroll_x = 0;
    let mut scroll_y = 0;

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let widget = QuipuWidget {
                quipu: &quipu,
                scroll_x,
                scroll_y,
            };
            f.render_widget(widget, size);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => scroll_x = scroll_x.saturating_sub(1),
                    KeyCode::Right => scroll_x = scroll_x.saturating_add(1),
                    KeyCode::Up => scroll_y = scroll_y.saturating_sub(1),
                    KeyCode::Down => scroll_y = scroll_y.saturating_add(1),
                    _ => {}
                }
            }
        }
    }
}
