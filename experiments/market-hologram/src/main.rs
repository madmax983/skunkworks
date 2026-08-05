//! # market-hologram 🧬
//!
//! **Status**: Experimental
//! **Lineage**: `market-sim` × `hologram-text`
//!
//! This hybrid crosses the 2D physical double auction market logic of `market-sim` with the frequency domain spectral imaging of `hologram-text`.
//!
//! ## Phenotype
//! **Spectral Liquidity:** The continuous physical double auction market particles (Bids/Asks) act as a density field, transformed via 2D FFT into a holographic projection. This visualizes the resonant modes of the market structure in the frequency domain.
//!
//! ## Execution
//!
//! ```bash
//! cargo run -p market-hologram
//! ```
//!
//! ## Controls
//!
//! *   **Arrows**: Tune holographic reconstruction angles (X/Y axis offset).
//! *   **b**: Spawn multiple Bids at random locations (X-axis).
//! *   **a**: Spawn multiple Asks at random locations (X-axis).
//! *   **Q**: Quit
//!
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use market_sim::{Grid, Particle};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod hologram;
use hologram::Hologram;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;

struct App {
    market: Grid,
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    next_id: usize,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> Self {
        Self {
            market: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            hologram: Hologram::new(GRID_WIDTH, GRID_HEIGHT),
            reconstruction_angle_x: 20,
            reconstruction_angle_y: 10,
            status_msg: "Press Spawner Keys | Arrows: Tune Angle | Q: Quit".to_string(),
            next_id: 0,
            reconstruction_data: vec![0.0; GRID_WIDTH * GRID_HEIGHT],
        }
    }

    fn spawn_wall(&mut self, x: usize, y: usize) {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            let idx = y * GRID_WIDTH + x;
            self.market.cells[idx] = Particle::Wall;
        }
    }

    fn spawn_agent(&mut self, x: usize, y: usize, ptype: Particle) {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            let idx = y * GRID_WIDTH + x;
            self.market.cells[idx] = ptype;
            self.next_id += 1;
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()>
    where
        <B as Backend>::Error: Send + Sync + 'static,
    {
        let mut last_tick = std::time::Instant::now();
        let tick_rate = Duration::from_millis(50);
        let mut rng = rand::thread_rng();

        // initial liquidity pool and a wall
        for x in 20..44 {
            self.spawn_wall(x, 32);
        }

        for _ in 0..30 {
            self.spawn_agent(10, 10, Particle::Bid(self.next_id));
            self.spawn_agent(50, 50, Particle::Ask(self.next_id));
        }

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Left => self.reconstruction_angle_x -= 1,
                            KeyCode::Right => self.reconstruction_angle_x += 1,
                            KeyCode::Up => self.reconstruction_angle_y -= 1,
                            KeyCode::Down => self.reconstruction_angle_y += 1,
                            KeyCode::Char('b') => {
                                for _ in 0..10 {
                                    self.spawn_agent(
                                        rng.gen_range(0..GRID_WIDTH),
                                        10,
                                        Particle::Bid(self.next_id),
                                    );
                                }
                                self.status_msg = "Spawned Bids".to_string();
                            }
                            KeyCode::Char('a') => {
                                for _ in 0..10 {
                                    self.spawn_agent(
                                        rng.gen_range(0..GRID_WIDTH),
                                        50,
                                        Particle::Ask(self.next_id),
                                    );
                                }
                                self.status_msg = "Spawned Asks".to_string();
                            }
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                // Update Market Simulation
                self.market.update();

                // Compute Hologram Data
                let mut density_grid = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
                for (idx, p) in self.market.cells.iter().enumerate() {
                    if idx < density_grid.len() {
                        let weight = match p {
                            Particle::Bid(_) => 1.0,
                            Particle::Ask(_) => -1.0,
                            Particle::Trade { .. } => 5.0,
                            Particle::Wall => 0.5,
                            Particle::Empty => 0.0,
                        };
                        density_grid[idx] += weight;
                    }
                }

                self.hologram = Hologram::from_grid(&density_grid, GRID_WIDTH, GRID_HEIGHT);

                // Get FFT reconstructed data
                self.reconstruction_data = self
                    .hologram
                    .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);

                terminal.draw(|f| self.draw(f))?;
                last_tick = std::time::Instant::now();
            }
        }
    }

    fn draw(&self, f: &mut ratatui::Frame) {
        let size = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(size);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);

        // Draw Spatial Market
        let market_canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Spatial Market Density"),
            )
            .x_bounds([0.0, GRID_WIDTH as f64])
            .y_bounds([0.0, GRID_HEIGHT as f64])
            .paint(|ctx| {
                for (idx, p) in self.market.cells.iter().enumerate() {
                    let x = idx % GRID_WIDTH;
                    let y = idx / GRID_WIDTH;
                    let color = match p {
                        Particle::Bid(_) => Some(Color::Green),
                        Particle::Ask(_) => Some(Color::Red),
                        Particle::Trade { .. } => Some(Color::Yellow),
                        Particle::Wall => Some(Color::Gray),
                        Particle::Empty => None,
                    };
                    if let Some(c) = color {
                        ctx.draw(&Points {
                            coords: &[(x as f64, (GRID_HEIGHT - y - 1) as f64)],
                            color: c,
                        });
                    }
                }
            });

        f.render_widget(market_canvas, main_chunks[0]);

        // Draw Holographic Reconstruction
        let holo_canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(format!(
                "Spectral Reconstruction (Angle: {}, {})",
                self.reconstruction_angle_x, self.reconstruction_angle_y
            )))
            .x_bounds([0.0, GRID_WIDTH as f64])
            .y_bounds([0.0, GRID_HEIGHT as f64])
            .paint(|ctx| {
                for y in 0..GRID_HEIGHT {
                    for x in 0..GRID_WIDTH {
                        let i = y * GRID_WIDTH + x;
                        let mag = self.reconstruction_data[i].abs();
                        if mag > 0.05 {
                            // Brightness threshold
                            let color = if mag > 1.0 {
                                Color::White
                            } else if mag > 0.5 {
                                Color::LightCyan
                            } else if mag > 0.2 {
                                Color::Cyan
                            } else {
                                Color::DarkGray
                            };

                            ctx.draw(&Points {
                                coords: &[(x as f64, (GRID_HEIGHT - y - 1) as f64)],
                                color,
                            });
                        }
                    }
                }
            });

        f.render_widget(holo_canvas, main_chunks[1]);

        let status_block = Block::default().borders(Borders::ALL).title("Status");
        let status_paragraph = Paragraph::new(Span::styled(
            &self.status_msg,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .block(status_block);

        f.render_widget(status_paragraph, chunks[1]);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
