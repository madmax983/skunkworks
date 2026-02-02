use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::env;
use std::time::{Duration, Instant};
use tui_shared::Tui;

use code_bio_dome::harvester::harvest_functions;
use code_bio_dome::simulation::World;
use code_bio_dome::ui::ui;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut path = ".".to_string();
    let mut semantic_mode = false;

    // Simple arg parsing
    for arg in args.iter().skip(1) {
        if arg == "--semantic" {
            semantic_mode = true;
        } else {
            path = arg.clone();
        }
    }

    // Harvest
    if !semantic_mode {
        println!("Harvesting code in '{}'...", path);
    }
    let signatures = harvest_functions(&path);

    if !semantic_mode {
        println!("Found {} functions.", signatures.len());
        if signatures.is_empty() {
            println!("No functions found. Try pointing to a directory with .rs files.");
            // Continue anyway to show empty world
        }
    }

    // Setup World
    let width = 200.0;
    let height = 150.0;
    let mut world = World::new(width, height);
    world.populate(&signatures);

    if semantic_mode {
        // Run a few ticks to let things settle?
        world.update();
        let snapshot = world.snapshot();
        println!("{}", snapshot.to_json_pretty());
        return Ok(());
    }

    // Interactive Mode
    let mut tui = Tui::init()?;
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        tui.terminal.draw(|f| ui(f, &world))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => running = false,
                        KeyCode::Char('s') => {
                            // Dump semantic state to file or log?
                            // For now just beep or something, or maybe write to a file
                            // But in TUI mode we can't print to stdout easily without breaking layout.
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
