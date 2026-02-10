use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::time::Duration;
use syn::visit::Visit;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

mod painter;
mod musician;
mod parser;

use painter::Canvas;
use musician::Musician;
use parser::SyntaxParser;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Logic
    let args: Vec<String> = env::args().collect();
    let default_path = "experiments/syntax-pollock/src/main.rs";
    let path = if args.len() > 1 { &args[1] } else { default_path };

    // Run app
    let res = run_app(&mut terminal, path);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, path: &str) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    let code = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;
    let ast = syn::parse_file(&code).context("Failed to parse Rust file")?;

    let mut canvas = Canvas::new(800, 600);
    let mut musician = Musician::new();

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.area());
        let block = Block::default().title("Syntax Pollock").borders(Borders::ALL);
        let text = Paragraph::new(format!("Parsing {}...\nPlease wait.", path)).block(block);
        f.render_widget(text, chunks[0]);
    })?;

    let mut parser = SyntaxParser {
        canvas: &mut canvas,
        musician: &mut musician,
        depth: 0,
    };

    parser.visit_file(&ast);

    canvas.image.save("syntax_pollock.png")?;
    musician.save("syntax_pollock.wav")?;

    // Show success
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.area());
        let block = Block::default().title("Syntax Pollock").borders(Borders::ALL);
        let text = Paragraph::new(format!("Done! Saved syntax_pollock.png and .wav\nPress 'q' to quit.")).block(block);
        f.render_widget(text, chunks[0]);
    })?;

    loop {
        if event::poll(Duration::from_millis(100))? {
             if let Event::Key(key) = event::read()? {
                 if let KeyCode::Char('q') = key.code {
                     return Ok(());
                 }
             }
        }
    }
}
