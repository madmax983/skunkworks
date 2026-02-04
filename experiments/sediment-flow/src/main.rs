mod git;
mod lbm;
mod terrain;

use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
};
use git::GitScanner;
use terrain::Terrain;
use lbm::Fluid;

struct App {
    terrain: Terrain,
    fluid: Fluid,
    commits: Vec<git::Commit>,
    commit_idx: usize,
    speed: usize,
    paused: bool,
    should_quit: bool,
}

impl App {
    fn new() -> Result<Self> {
        // Handle git scanner failure gracefully
        let commits = match GitScanner::load_history() {
             Ok(c) => c,
             Err(_) => vec![], // Empty if no git repo or error
        };
        let width = 120;
        let height = 60;
        Ok(Self {
            terrain: Terrain::new(width, height),
            fluid: Fluid::new(width, height),
            commits,
            commit_idx: 0,
            speed: 1,
            paused: false,
            should_quit: false,
        })
    }

    fn update(&mut self) {
        if !self.paused && self.commit_idx < self.commits.len() {
            let end_idx = (self.commit_idx + self.speed).min(self.commits.len());

            for i in self.commit_idx..end_idx {
                let commit = &self.commits[i];
                for file in &commit.files {
                    let path_str = file.to_string_lossy();
                    let (x, y) = GitScanner::map_path(&path_str, self.terrain.width, self.terrain.height);

                    // Uplift (Magma/Growth)
                    self.terrain.uplift(x, y, 5.0);
                }
            }
            self.commit_idx = end_idx;
        }

        // Sync Terrain to Fluid Obstacles
        self.fluid.clear_obstacles();
        for y in 0..self.terrain.height {
            for x in 0..self.terrain.width {
                if self.terrain.get_height(x, y) > 10.0 {
                    self.fluid.add_obstacle(x, y);
                }
            }
        }

        // Step Fluid
        self.fluid.step();
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init App
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            disable_raw_mode()?;
            execute!(std::io::stdout(), LeaveAlternateScreen)?;
            eprintln!("Error initializing: {}", e);
            return Err(e);
        }
    };

    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('+') => app.speed = (app.speed * 2).min(100),
                        KeyCode::Char('-') => app.speed = (app.speed / 2).max(1),
                        KeyCode::Char('r') => {
                            let width = 120;
                            let height = 60;
                            app.terrain = Terrain::new(width, height);
                            app.fluid = Fluid::new(width, height);
                            app.commit_idx = 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let w = app.terrain.width as f64;
    let h = app.terrain.height as f64;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Sediment Flow: Fluid Dynamics on Code Terrain "))
        .x_bounds([0.0, w])
        .y_bounds([0.0, h])
        .paint(|ctx| {
            // Draw Terrain & Fluid
            // We iterate over the grid
            for y in 0..app.terrain.height {
                for x in 0..app.terrain.width {
                    // Coordinate flip for rendering (Canvas 0,0 is bottom-left usually)
                    // Terrain 0,0 is usually top-left.
                    // Let's invert Y.
                    let render_y = (app.terrain.height - 1 - y) as f64;

                    let height = app.terrain.get_height(x, y);
                    let is_obstacle = app.fluid.obstacles[y * app.fluid.width + x];

                    // Fluid properties
                    // Safety check for bounds
                    let idx = y * app.fluid.width + x;
                    if idx >= app.fluid.rho.len() { continue; }

                    let _rho = app.fluid.rho[idx];
                    let ux = app.fluid.u_x[idx];
                    let uy = app.fluid.u_y[idx];
                    let velocity = (ux*ux + uy*uy).sqrt();

                    let color = if is_obstacle {
                        // Terrain
                        if height < 20.0 { Color::DarkGray }
                        else if height < 50.0 { Color::Gray }
                        else { Color::White }
                    } else {
                        // Fluid
                        // Visualize curl or velocity?
                        // Simple velocity:
                        if velocity > 0.1 {
                             // Cyan for fast fluid
                             Color::Cyan
                        } else if velocity > 0.05 {
                             Color::Blue
                        } else {
                             // Empty space / calm fluid
                             Color::Reset
                        }
                    };

                    if color != Color::Reset {
                        ctx.draw(&Rectangle {
                            x: x as f64,
                            y: render_y,
                            width: 1.0,
                            height: 1.0,
                            color,
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status
    let progress = if !app.commits.is_empty() {
        (app.commit_idx as f64 / app.commits.len() as f64) * 100.0
    } else {
        0.0
    };

    let status = format!(
        "Commit: {}/{} | Progress: {:.1}% | Speed: {} | [Space] Pause [+/-] Speed [r] Reset [q] Quit",
        app.commit_idx,
        app.commits.len(),
        progress,
        app.speed,
    );

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1]
    );
}
