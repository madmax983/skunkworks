mod game;
mod parser;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::BattleState;
use parser::{scan_files, Fighter};
use rand::seq::SliceRandom;
use std::{
    env,
    path::Path,
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct App {
    state: BattleState,
    fighters: Vec<Fighter>,
    auto_play: bool,
    running: bool,
}

impl App {
    fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();
        // Default to current directory if no arg
        let path_str = if args.len() > 1 { &args[1] } else { "." };
        let path = Path::new(path_str);

        // Scan
        let fighters = scan_files(path)?;
        if fighters.len() < 2 {
            anyhow::bail!(
                "Need at least 2 functions to fight! Found {} in {:?}",
                fighters.len(),
                path
            );
        }

        let mut rng = rand::thread_rng();
        let f1 = fighters.choose(&mut rng).unwrap().clone();
        let f2 = fighters.choose(&mut rng).unwrap().clone();

        Ok(Self {
            state: BattleState::new(f1, f2),
            fighters,
            auto_play: false,
            running: true,
        })
    }

    fn restart(&mut self) {
        let mut rng = rand::thread_rng();
        let f1 = self.fighters.choose(&mut rng).unwrap().clone();
        let f2 = self.fighters.choose(&mut rng).unwrap().clone();
        self.state = BattleState::new(f1, f2);
    }

    fn on_tick(&mut self) {
        if self.auto_play {
            self.state.step();
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            // Explicitly restore terminal if app init fails (e.g. no files found)
            // Tui struct drop should handle it, but explicit drop ensures it happens before eprintln
            drop(tui);
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal.draw(|f| ui::draw(f, &app.state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        KeyCode::Char(' ') => app.state.step(),
                        KeyCode::Enter => app.auto_play = !app.auto_play,
                        KeyCode::Char('r') => app.restart(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
