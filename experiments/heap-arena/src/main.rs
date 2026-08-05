//! # Heap Arena 🏟️
//!
//! **Lineage:** `func-arena` × `heap-hopper`
//!
//! A hybrid experiment combining RPG code metrics with memory-management platforming.
//!
//! ## 🧬 Concept
//!
//! You are the **Garbage Collector**. Your mission is to reclaim memory from complex, bloated functions.
//! The function is the level. Its lines of code form the ground you walk on.
//! The function is also the **Boss**, floating at the end of its memory allocation, throwing exceptions at you.
//!
//! ## 🎮 Mechanics
//!
//! - **Terrain Generation**: The level layout is procedurally generated from the source code of a random function in the target directory.
//!   - `let` / `fn`: Solid ground.
//!   - `unsafe` / `panic!`: Hazard blocks (Red).
//!   - Loops: Bouncy blocks (Blue).
//!   - Empty lines: Gaps (Fall = Segfault).
//!
//! - **Boss Stats**:
//!   - **HP**: Based on Lines of Code.
//!   - **Attack**: Based on Cyclomatic Complexity (determines projectile fire rate).
//!   - **Defense**: Based on Argument count.
//!
//! - **Objective**:
//!   - Survive the traversal of the function's heap.
//!   - Dodge "Exception" projectiles.
//!   - Reach the Return Address (end of level) to "Collect" the function.
//!
//! ## 🕹️ Controls
//!
//! - **WASD / Arrows**: Move and Jump.
//! - **Q / ESC**: Quit.
//!
//! ## 🚀 Usage
//!
//! ```bash
//! cargo run -p heap-arena -- [path_to_scan]
//! ```
//!
//! Example:
//! ```bash
//! cargo run -p heap-arena -- experiments/heap-arena/src
//! ```
//!
mod allocator;
mod game;
mod level_gen;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::Game;
use std::env;
use std::path::Path;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Generate Level
    let current_dir = env::current_dir()?;
    let arg = env::args().nth(1);
    let path_str = arg.unwrap_or_else(|| current_dir.to_string_lossy().into_owned());
    let path = Path::new(&path_str);

    println!("Scanning {} for a worthy opponent...", path.display());
    let level_profile = match level_gen::generate_level(path) {
        Some(p) => p,
        None => {
            eprintln!(
                "No suitable Rust functions found to battle in {}.",
                path.display()
            );
            return Ok(());
        }
    };

    println!(
        "Challenger Found: {} (HP: {}, ATK: {})",
        level_profile.boss.name, level_profile.boss.max_hp, level_profile.boss.attack
    );
    println!("Entering Arena...");
    std::thread::sleep(Duration::from_secs(1));

    let mut tui = Tui::init()?;
    let mut app = Game::new(level_profile);

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut Game) -> Result<()> {
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') | KeyCode::Up | KeyCode::Char('w') => app.player.jump(),
                        KeyCode::Left | KeyCode::Char('a') => app.player.move_left(),
                        KeyCode::Right | KeyCode::Char('d') => app.player.move_right(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
