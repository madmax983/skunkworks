//! # Git Rogue
//!
//! A TUI-based rogue-like explorer for your git history. Crawl through commits, battle bugs, and discover features!
//!
//! ## 🎮 How to Play
//!
//! Run the game in any git repository:
//!
//! ```bash
//! cargo run --release
//! ```
//!
//! **Objective:** Navigate from HEAD back to the initial commit (or as far as you can go) without running out of HP.
//!
//! ### Mechanics
//!
//! *   **🐛 Bugs (Damage):** Commits containing `fix`, `bug`, `panic`, or `error` are unstable! They deal damage to your HP.
//! *   **✨ Features (XP/Heal):** Commits containing `feat`, `add`, or `new` are rewarding! They grant XP and restore a small amount of HP.
//! *   **🐉 Merge Dragons:** Merge commits are dangerous bosses. Beware!
//!
//! ### Controls
//!
//! *   `1-9`: Go to **Parent** (Move backward in time).
//! *   `Shift + 1-9`: Go to **Child** (Move forward in time).
//! *   `Q`: Quit the game.
//!
//! ## 🎨 UI Polish (Mosaic)
//!
//! This tool features a polished TUI interface designed by Mosaic:
//! *   **Visual Feedback:** HP Bar changes color (Green -> Yellow -> Red) as you take damage.
//! *   **Aesthetics:** Rounded borders and emoji indicators (📍, 🚪, 📜) for a modern terminal look.
//! *   **Readability:** Log messages are color-coded to highlight damage (Red) and rewards (Green).
//!
//! ## License
//!
//! MIT
//!
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
                handle_input(key, &mut app, &mut running);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn handle_input(key: crossterm::event::KeyEvent, app: &mut Game, running: &mut bool) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    if app.game_over {
        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
            *running = false;
        }
        return;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => *running = false,

        // Digits 1-9 for Parents
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            if let Some(digit) = c.to_digit(10) {
                let index = digit as usize - 1;
                // Check if shift is pressed for Children (if terminal sends '1' + SHIFT)
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    move_child(app, index);
                } else {
                    // Go to parent
                    if let Some(node) = app.current_node() {
                        if index < node.parents.len() {
                            let next_hash = node.parents[index].clone();
                            app.move_to(next_hash);
                        }
                    }
                }
            }
        }

        // Shift + Digits (Symbols) for Children
        KeyCode::Char('!') => move_child(app, 0),
        KeyCode::Char('@') => move_child(app, 1),
        KeyCode::Char('#') => move_child(app, 2),
        KeyCode::Char('$') => move_child(app, 3),
        KeyCode::Char('%') => move_child(app, 4),
        KeyCode::Char('^') => move_child(app, 5),
        KeyCode::Char('&') => move_child(app, 6),
        KeyCode::Char('*') => move_child(app, 7),
        KeyCode::Char('(') => move_child(app, 8),

        _ => {}
    }
}

fn move_child(game: &mut Game, index: usize) {
    if let Some(node) = game.current_node() {
        if index < node.children.len() {
            let next_hash = node.children[index].clone();
            game.move_to(next_hash);
        }
    }
}
