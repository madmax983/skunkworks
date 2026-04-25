mod physics;
mod quipu_logic;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use physics::PhysicsWorld;
use quipu_logic::QuipuMachine;
use std::time::{Duration, Instant};
use tui_shared::Tui;
use ui::ui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Setup Physics
    let mut physics_world = PhysicsWorld::new();

    // Setup Quipu Machine with some input data (e.g., 12345)
    let mut machine = QuipuMachine::new(12345);

    let mut last_tick = Instant::now();
    let mut paused = false;

    loop {
        // Render
        tui.terminal.draw(|f| {
            ui(f, &physics_world, &machine);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') => {
                        // Reset
                        physics_world = PhysicsWorld::new();
                        machine = QuipuMachine::new(12345);
                    }
                    _ => {}
                }
            }
        }

        // Logic
        if !paused {
            let now = Instant::now();
            let dt = now.duration_since(last_tick).as_secs_f32();
            last_tick = now;

            // Step Physics
            physics_world.step();

            // Step Machine Logic
            machine.tick(&mut physics_world, dt);
        } else {
            last_tick = Instant::now();
        }
    }

    Ok(())
}
