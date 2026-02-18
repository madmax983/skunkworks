pub mod model;
pub mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::model::Network;
use crate::ui::draw_ui;

struct App {
    network: Network,
    running: bool,
}

impl App {
    fn new() -> Self {
        let mut network = Network::new();
        // Generate a 5x5 mesh
        network.generate_mesh(5, 5);
        Self {
            network,
            running: true,
        }
    }

    fn update(&mut self) {
        self.network.tick();
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal.draw(|f| draw_ui(f, &app.network))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                    KeyCode::Char('b') => app.network.burst(10),
                    KeyCode::Char('r') => {
                        app.network = Network::new();
                        app.network.generate_mesh(5, 5);
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
