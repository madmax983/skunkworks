mod app;
mod blame;
mod ui;

use anyhow::Result;
use app::App;
use blame::BlameAnalyzer;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io, path::Path, time::Duration};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chrontext <file_path>");
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
    let mut app = App::new(file_path_str.to_string(), blame_info)?;

    // Run Loop
    let res = run_app(&mut terminal, &mut app);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        let size = terminal.size()?;
        let view_height = (size.height as usize).saturating_sub(5); // -3 for status, -2 for borders
        app.update_scroll(view_height);

        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next_line(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_line(),
                    _ => {}
                }
            }
        }
    }
}
