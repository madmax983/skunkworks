use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self};
use std::time::{Duration, Instant};

use git_galaxy::harvester::harvest_repo;
use git_galaxy::physics::Graph;
use git_galaxy::ui::ui;

fn main() -> Result<()> {
    // 1. Harvest
    // Default to current directory, or first arg
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!("Harvesting repo at: {}", path);
    let commits = harvest_repo(&path)?;

    if commits.is_empty() {
        println!("No commits found!");
        return Ok(());
    }

    let mut graph = Graph::new(commits);

    // 2. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Loop
    let tick_rate = Duration::from_millis(16); // 60 FPS
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        terminal.draw(|f| ui(f, &graph))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        running = false;
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update physics
            // Pass delta time in seconds
            let dt = last_tick.elapsed().as_secs_f64();
            // Cap dt to avoid explosion on lag
            let dt = dt.min(0.1);

            graph.update(dt);

            last_tick = Instant::now();
        }
    }

    // 4. Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
