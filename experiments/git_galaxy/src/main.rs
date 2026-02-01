use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use git_galaxy::harvester::harvest_repo;
use git_galaxy::physics::Graph;
use git_galaxy::ui::{ui, ViewState};

struct App {
    graph: Graph,
    view_state: ViewState,
    running: bool,
}

impl App {
    fn new(path: String) -> Result<Self> {
        println!("Harvesting repo at: {}", path);
        let commits = harvest_repo(&path)?;

        if commits.is_empty() {
            println!("No commits found!");
        }

        let graph = Graph::new(commits);
        let view_state = ViewState::default();

        Ok(Self {
            graph,
            view_state,
            running: true,
        })
    }

    fn run(&mut self) -> Result<()> {
        // Setup Terminal
        let mut tui = Tui::init()?;

        // Loop
        let tick_rate = Duration::from_millis(16); // 60 FPS
        let mut last_tick = Instant::now();

        while self.running {
            tui.terminal
                .draw(|f| ui(f, &self.graph, &self.view_state))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => self.running = false,
                            KeyCode::Left => self.view_state.pan_x -= 10.0,
                            KeyCode::Right => self.view_state.pan_x += 10.0,
                            KeyCode::Up => self.view_state.pan_y += 10.0,
                            KeyCode::Down => self.view_state.pan_y -= 10.0,
                            KeyCode::Char('+') | KeyCode::Char('=') => self.view_state.zoom *= 1.1,
                            KeyCode::Char('-') | KeyCode::Char('_') => self.view_state.zoom /= 1.1,
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                // Update physics
                let dt = last_tick.elapsed().as_secs_f64();
                let dt = dt.min(0.1);

                self.graph.update(dt);

                last_tick = Instant::now();
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let mut app = App::new(path)?;
    app.run()
}
