mod git;
mod pet;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pet::Pet;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state
    let mut pet = Pet::load()?;

    // Initial sync
    sync_git(&mut pet);

    let res = run_app(&mut terminal, &mut pet);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Save on exit
    pet.save()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn sync_git(pet: &mut Pet) {
    let last_commit = git::get_last_commit_time().unwrap_or(pet.last_fed);
    let count = git::count_commits_last_24h().unwrap_or(0);
    pet.update(last_commit, count);
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, pet: &mut Pet) -> Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let mut last_git_check = Instant::now();
    let git_check_interval = Duration::from_secs(10);

    loop {
        terminal.draw(|f| ui::render(f, pet))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('f') => {
                        // Cheat code: feed
                        pet.feed(1);
                        pet.save()?; // Save immediately on interaction
                    }
                    KeyCode::Char('r') => {
                        // Manual refresh
                        sync_git(pet);
                        pet.save()?;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update decay frequently
            // We reuse the last known commit time and activity to avoid spamming git
            pet.update(pet.last_fed, pet.activity_today);
            last_tick = Instant::now();
        }

        if last_git_check.elapsed() >= git_check_interval {
            sync_git(pet);
            last_git_check = Instant::now();
        }
    }
}
