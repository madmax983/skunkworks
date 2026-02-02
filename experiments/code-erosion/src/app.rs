use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{path::Path, time::{Duration, Instant}};

use crate::{erosion::Droplet, terrain::TerrainMap};

pub struct App {
    terrain: TerrainMap,
    droplets: Vec<Droplet>,
    should_quit: bool,
    iterations: usize,
}

impl App {
    pub fn new() -> Result<Self> {
        // Try to load from current directory, or fallback
        let path = Path::new(".");
        let terrain = TerrainMap::from_filesystem(path).unwrap_or_else(|_| TerrainMap::new(50, 50));

        Ok(Self {
            terrain,
            droplets: Vec::new(),
            should_quit: false,
            iterations: 0,
        })
    }

    pub fn run<B: ratatui::backend::Backend>(&mut self, terminal: &mut ratatui::Terminal<B>) -> Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16); // ~60 fps

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                        KeyCode::Char('r') => {
                            // Reset
                            if let Ok(t) = TerrainMap::from_filesystem(Path::new(".")) {
                                self.terrain = t;
                            }
                            self.droplets.clear();
                            self.iterations = 0;
                        }
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.update();
                last_tick = Instant::now();
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn update(&mut self) {
        // Spawn droplets
        // Try to spawn a few per frame
        for _ in 0..10 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0.0..self.terrain.width as f64);
            let y = rng.gen_range(0.0..self.terrain.height as f64);
            self.droplets.push(Droplet::new(x, y));
        }

        // Erode droplets
        // Only keep droplets that are still active
        let mut alive_droplets = Vec::with_capacity(self.droplets.len());

        for mut droplet in self.droplets.drain(..) {
            if droplet.erode(&mut self.terrain) {
                alive_droplets.push(droplet);
            }
        }

        self.droplets = alive_droplets;
        self.iterations += 1;
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        let main_area = chunks[0];
        let status_area = chunks[1];

        // Draw Terrain
        // Map height to colors?
        // Canvas is limited in color per shape usually, unless we draw many small rects.
        // Drawing a pixel for every cell might be slow if the map is huge.
        // But let's try.

        let width = self.terrain.width as f64;
        let height = self.terrain.height as f64;

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Code Erosion"))
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(|ctx| {
                // We need to iterate over the visible area or just all points.
                // Optimally we only draw what we see.
                // For now, draw all points as 1x1 rectangles

                for y in 0..self.terrain.height {
                    for x in 0..self.terrain.width {
                        let h = self.terrain.get(x, y);
                        // Determine color based on height
                        let color = if h < 0.5 {
                            Color::Blue // Water/Deep
                        } else if h < 2.0 {
                            Color::Green
                        } else if h < 5.0 {
                            Color::Yellow
                        } else if h < 10.0 {
                            Color::Red
                        } else {
                            Color::White
                        };

                        // Only draw if not empty (optimization?)
                        // Draw a small rectangle
                        ctx.draw(&Rectangle {
                            x: x as f64,
                            y: (self.terrain.height - 1 - y) as f64, // Flip Y for canvas? Canvas 0,0 is bottom-left usually. Terrain 0,0 is top-left usually.
                            width: 1.0,
                            height: 1.0,
                            color,
                        });
                    }
                }

                // Draw droplets?
                for d in &self.droplets {
                    ctx.draw(&Rectangle {
                        x: d.x,
                        y: (self.terrain.height as f64 - 1.0 - d.y),
                        width: 0.5,
                        height: 0.5,
                        color: Color::Cyan,
                    });
                }
            });

        f.render_widget(canvas, main_area);

        // Status
        let status = format!("Iter: {} | Droplets: {} | Size: {}x{}",
            self.iterations, self.droplets.len(), self.terrain.width, self.terrain.height);
        f.render_widget(Paragraph::new(status).style(Style::default().bg(Color::Blue)), status_area);
    }
}
