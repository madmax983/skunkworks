mod crawler;
mod game;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::crawler::crawl;
use crate::game::Game;
use crate::ui::draw;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!("Crawling repository at '{}'...", path);
    // Crawl 500 commits to have a decent history
    let nodes = crawl(&path, 500)?;

    if nodes.is_empty() {
        println!("No commits found!");
        return Ok(());
    }

    let mut app = Game::new(nodes);

    // Setup TUI
    let mut tui = Tui::init()?;

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        tui.terminal.draw(|f| draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.game_over {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                            running = false;
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => running = false,

                            // Digits 1-9 for Parents
                            KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                                let index = c.to_digit(10).unwrap() as usize - 1;
                                // Check if shift is pressed for Children (if terminal sends '1' + SHIFT)
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    move_child(&mut app, index);
                                } else {
                                    // Go to parent
                                    let node = app.current_node();
                                    if index < node.parents.len() {
                                        let next_hash = node.parents[index].clone();
                                        app.move_to(next_hash);
                                    }
                                }
                            }

                            // Shift + Digits (Symbols) for Children
                            KeyCode::Char('!') => move_child(&mut app, 0),
                            KeyCode::Char('@') => move_child(&mut app, 1),
                            KeyCode::Char('#') => move_child(&mut app, 2),
                            KeyCode::Char('$') => move_child(&mut app, 3),
                            KeyCode::Char('%') => move_child(&mut app, 4),
                            KeyCode::Char('^') => move_child(&mut app, 5),
                            KeyCode::Char('&') => move_child(&mut app, 6),
                            KeyCode::Char('*') => move_child(&mut app, 7),
                            KeyCode::Char('(') => move_child(&mut app, 8),

                            _ => {}
                        }
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

fn move_child(game: &mut Game, index: usize) {
    let node = game.current_node();
    if index < node.children.len() {
        let next_hash = node.children[index].clone();
        game.move_to(next_hash);
    }
}
