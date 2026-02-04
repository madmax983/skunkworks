pub mod game;
pub mod scanner;
pub mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::path::Path;
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::game::Game;
use crate::scanner::{load_snippet, scan_files};
use crate::ui::draw;

fn main() -> Result<()> {
    // 1. Initialize TUI
    let mut tui = Tui::init()?;

    // 2. Scan files
    // Scan the current directory
    let root = Path::new(".");
    let files = scan_files(root);

    // 3. Game Loop
    let mut game = if files.is_empty() {
        Game::new("fn main() {\n    println!(\"No .rs files found!\");\n}".to_string())
    } else {
        load_new_game(&files)?
    };

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        tui.terminal.draw(|f| draw(f, &game))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => running = false,
                        KeyCode::Tab => {
                            if !files.is_empty() {
                                game = load_new_game(&files)?;
                            }
                        }
                        KeyCode::Enter => {
                            if game.finished {
                                if !files.is_empty() {
                                    game = load_new_game(&files)?;
                                }
                            } else {
                                game.input_char('\n');
                            }
                        }
                        KeyCode::Backspace => {
                            game.backspace();
                        }
                        KeyCode::Char(c) => {
                            // Handle Ctrl+C
                            if key.modifiers == crossterm::event::KeyModifiers::CONTROL && c == 'c'
                            {
                                running = false;
                            } else {
                                game.input_char(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn load_new_game(files: &[std::path::PathBuf]) -> Result<Game> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();

    // Pick a file
    if let Some(file) = files.choose(&mut rng) {
        let snippet = load_snippet(file)?;
        Ok(Game::new(snippet))
    } else {
        // Fallback
        Ok(Game::new(
            "fn error() {\n    panic!(\"No files!\");\n}".to_string(),
        ))
    }
}
