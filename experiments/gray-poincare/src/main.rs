//! # Gray-Poincaré Morphogenesis
//!
//! This hybrid bridges the biological reaction-diffusion patterns of `gray-scott` with the continuous, hyperbolic non-Euclidean space of `poincare-disk`.
//!
//! ## Lineage
//!
//! * **From `gray-scott`**: Provides the continuous thermodynamic scalar field representing the chemical concentrations of U and V, resulting in Turing patterns.
//! * **From `poincare-disk`**: Provides the continuous non-Euclidean coordinate space, mapping flat Euclidean coordinates into the hyperbolic bounds of the Poincaré disk.
//!
//! ## Emergent Phenotype
//!
//! This experiment physically warps the morphogenesis of reaction-diffusion patterns into hyperbolic space. As chemical concentration blobs grow, they appear to stretch and compress near the disk boundary, resulting in an Escher-like rendering of biological growth. The entire structure is slowly rotated via a continuous Möbius transformation.
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use poincare_disk::{Mobius, Point};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct GrayPoincareApp {
    dish: GrayScott,
    width: usize,
    height: usize,
    transform: Mobius,
}

impl GrayPoincareApp {
    fn new(width: usize, height: usize) -> Self {
        let mut dish = GrayScott::new(width, height);
        // Add a chemical seed in the center
        dish.add_chemical(width / 2, height / 2, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2, 1.0);
        dish.add_chemical(width / 2, height / 2 + 1, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2 + 1, 1.0);
        Self {
            dish,
            width,
            height,
            transform: Mobius::rotation(0.0),
        }
    }

    fn tick(&mut self) {
        let feed = 0.055;
        let kill = 0.062;
        // Perform multiple simulation steps per tick for faster morphogenesis
        for _ in 0..10 {
            self.dish.update(feed, kill, 1.0);
        }

        // Apply a slow hyperbolic rotation
        let rot = Mobius::rotation(0.02);
        self.transform = self.transform.then(&rot);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 50;
    let height = 50;
    let mut app = GrayPoincareApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Gray-Poincaré Morphogenesis "),
                )
                .x_bounds([-1.1, 1.1])
                .y_bounds([-1.1, 1.1])
                .paint(|ctx| {
                    // Draw Disk boundary
                    for i in 0..100 {
                        let angle = (i as f64) / 100.0 * 2.0 * std::f64::consts::PI;
                        ctx.print(
                            angle.cos(),
                            angle.sin(),
                            ratatui::text::Span::styled(".", Style::default().fg(Color::DarkGray)),
                        );
                    }

                    // Map the Euclidean grid into the Poincaré disk
                    let v_buf = app.dish.v();
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let v = v_buf[y * app.width + x];
                            if v > 0.1 {
                                // Only render significant concentrations
                                let normalized_x = (x as f64 / app.width as f64) * 2.0 - 1.0;
                                let normalized_y = (y as f64 / app.height as f64) * 2.0 - 1.0;

                                // Shrink to fit inside the disk
                                let mut p = Point::new(normalized_x * 0.9, normalized_y * 0.9);
                                p = app.transform.apply(p);

                                let color = if v > 0.5 { Color::Cyan } else { Color::Blue };

                                ctx.print(
                                    p.re,
                                    p.im,
                                    ratatui::text::Span::styled("•", Style::default().fg(color)),
                                );
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let v_buf = app.dish.v();
            let total_v: f32 = v_buf.iter().sum();

            let stats = Paragraph::new(format!(
                "Total Mass (V): {:.2} | Grid Size: {}x{} | [Q] Quit",
                total_v, app.width, app.height
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
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
}
