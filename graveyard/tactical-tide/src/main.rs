use std::{
    io::stdout,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tactical_tide::{sim::Grid, ui};

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Game State
    let width = 60;
    let height = 30;
    let mut grid = Grid::new(width, height);
    let mut cursor_x = width / 2;
    let mut cursor_y = height / 2;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 FPS

    loop {
        // Render
        terminal.draw(|f| {
            let size = f.size();
            ui::draw_grid(f, &grid, size, Some((cursor_x, cursor_y)));
        })?;

        // Input Handling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left | KeyCode::Char('a') => {
                        cursor_x = cursor_x.saturating_sub(1);
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        if cursor_x < width - 1 {
                            cursor_x += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        cursor_y = cursor_y.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        if cursor_y < height - 1 {
                            cursor_y += 1;
                        }
                    }
                    KeyCode::Char(' ') => {
                        // Add Water
                        if let Some(cell) = grid.get_mut(cursor_x, cursor_y) {
                            cell.water += 5.0;
                        }
                    }
                    KeyCode::Enter => {
                        // Raise Terrain
                        if let Some(cell) = grid.get_mut(cursor_x, cursor_y) {
                            cell.terrain += 2.0;
                        }
                    }
                    KeyCode::Backspace => {
                         // Lower Terrain
                        if let Some(cell) = grid.get_mut(cursor_x, cursor_y) {
                            cell.terrain -= 2.0;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Update Physics
        if last_tick.elapsed() >= tick_rate {
            grid.update(0.1); // Fixed dt
            last_tick = Instant::now();
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
