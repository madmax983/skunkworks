//! # 🧬 Splice: origami-platter
//!
//! **Lineage:** `crates/origami` × `crates/platter`
//!
//! **Concept:** Topological Soft-Body Heatmap.
//!
//! **Novel trait:** The physical 3D vertices of a continuous procedural Miura-ori soft-body mesh are projected down onto a continuous 2D scalar field (`platter`). The height (Z-depth) of the folds dictates the heat deposited into the field. As the mesh breathes and folds dynamically, it leaves behind a fading trail of topographical stress on the canvas.
//!
//! **Predicted Phenotype:** An emergent organic drone visualizer. The soft-body folds map their physical properties (mountain vs valley folds) directly into localized heat accumulation on the 2D grid, producing a pulsing, dissipating heat map corresponding directly to physical tension and geometry.
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use origami::{generate_miura_grid, MiuraParams, Orientation};
use platter::Platter;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct OrigamiPlatterApp {
    platter: Platter,
    width: usize,
    height: usize,
    time: f32,
}

impl OrigamiPlatterApp {
    fn new(width: usize, height: usize) -> Self {
        Self {
            platter: Platter::new(width, height),
            width,
            height,
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;

        // Oscillate the origami extension factor between 0.2 and 1.0 to simulate breathing/folding
        let extension = 0.6 + (self.time.sin() * 0.4);

        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 60.0_f32.to_radians(),
            orientation: Orientation::Horizontal,
        };

        // Generate the 3D points of the origami mesh
        let cols = 15;
        let rows = 15;
        let points = generate_miura_grid(params, (cols, rows), extension);

        // Project the 3D points onto the 2D platter scalar field
        for p in points {
            // Map the physical point coordinates to the 2D grid
            // The points are centered around 0,0 and scaled based on cols/rows.
            // Width/height max is roughly cols * a and rows * b.
            let max_w = cols as f32 * 1.0;
            let max_h = rows as f32 * 1.0;

            // Map x from [-max_w/2, max_w/2] to [0, width]
            let x_norm = (p.x + max_w / 2.0) / max_w;
            // Map y from [-max_h/2, max_h/2] to [0, height]
            let y_norm = (p.y + max_h / 2.0) / max_h;

            let grid_x = (x_norm * self.width as f32) as isize;
            let grid_y = (y_norm * self.height as f32) as isize;

            if grid_x >= 0
                && grid_x < self.width as isize
                && grid_y >= 0
                && grid_y < self.height as isize
            {
                // The heat represents the stress/presence of the folding mesh
                // Points with higher Z (mountains) deposit more heat, lower Z (valleys) deposit less
                let z_weight = (p.z.abs() + 0.1).clamp(0.0, 1.0) as f64;
                self.platter
                    .accumulate(grid_x as usize, grid_y as usize, 0.2 * z_weight);
            }
        }

        // Decay the field
        self.platter.decay(0.92);
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let title = " 🧬 Splice: origami × platter (Topological Soft-Body Heatmap) ";
        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let val = self.platter.get(x, y);
                        if val > 0.05 {
                            // Map scalar value to a color density
                            let color = if val > 0.8 {
                                Color::Red
                            } else if val > 0.5 {
                                Color::LightRed
                            } else if val > 0.2 {
                                Color::Yellow
                            } else {
                                Color::DarkGray
                            };

                            // Map grid to abstract coords
                            let render_x = (x as f64 / self.width as f64) * 100.0;
                            let render_y = (y as f64 / self.height as f64) * 100.0;

                            ctx.print(
                                render_x,
                                render_y,
                                ratatui::text::Span::styled("█", Style::default().fg(color)),
                            );
                        }
                    }
                }
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        frame.render_widget(canvas, chunks[0]);

        let info = Paragraph::new(format!(
            "Extension factor: {:.2} | Accumulating heat from 3D paper mesh...",
            0.6 + (self.time.sin() * 0.4)
        ))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    // Determine if we should run in headless mode (for CI/Testing)
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("🧬 origami-platter running in headless mode for 10 ticks.");
        let mut app = OrigamiPlatterApp::new(100, 100);
        for _ in 0..10 {
            app.tick();
        }
        println!("Finished headless run.");
        return Ok(());
    }

    let mut tui = Tui::init()?;

    let mut app = OrigamiPlatterApp::new(200, 100);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
                {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    // Drop trait automatically restores
    Ok(())
}
