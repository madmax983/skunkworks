#![allow(clippy::collapsible_if)]
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use hyphal_commute::{Grid, World};

fn main() -> Result<()> {
    // Setup Terminal
    let mut tui = Tui::init()?;

    // Setup Simulation
    let mut world = World::new(200, 100, 2000); // 200x100 grid, 2000 agents
    world.generate_city();

    // Run Loop
    let res = run_app(&mut tui.terminal, &mut world);

    // Cleanup
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    world: &mut World,
) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 FPS

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            // Main Area
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Hyphal Commute: Urban Myco-Web");

            let inner_area = block.inner(chunks[0]);
            f.render_widget(block, chunks[0]);

            // Render Map
            f.render_widget(HyphalMap { grid: &world.grid }, inner_area);

            // Status Bar
            let status = Paragraph::new("Press 'q' to Quit | 'r' to Reset | 'g' to Generate City")
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('g') => {
                            world.generate_city();
                        }
                        KeyCode::Char('r') => {
                            // Reset
                            let mut new_world =
                                World::new(world.grid.width, world.grid.height, 2000);
                            new_world.generate_city();
                            *world = new_world;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.tick();
            last_tick = Instant::now();
        }
    }
}

struct HyphalMap<'a> {
    grid: &'a Grid,
}

impl<'a> Widget for HyphalMap<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        for y in 0..area.height {
            for x in 0..area.width {
                // Map to grid coordinates
                // We want 2 vertical pixels per cell (HalfBlock)

                // Top half
                let gy_top = (y as f64 * 2.0 / (area.height as f64 * 2.0) * self.grid.height as f64)
                    as usize;
                let gx = (x as f64 / area.width as f64 * self.grid.width as f64) as usize;

                // Bottom half
                let gy_bot = ((y as f64 * 2.0 + 1.0) / (area.height as f64 * 2.0)
                    * self.grid.height as f64) as usize;

                let v_top = self.grid.get(gx, gy_top);
                let v_bot = self.grid.get(gx, gy_bot);

                let c_top = val_to_color(v_top);
                let c_bot = val_to_color(v_bot);

                let cell = buf.cell_mut((area.left() + x, area.top() + y));
                if let Some(cell) = cell {
                    cell.set_symbol("▀").set_fg(c_top).set_bg(c_bot);
                }
            }
        }
    }
}

fn val_to_color(v: f32) -> Color {
    if v < 0.1 {
        Color::Black
    } else {
        // Gradient: Dark Green -> Bright Green -> White
        // Green: 50..255
        let g = (50.0 + v * 40.0).clamp(50.0, 255.0) as u8;
        // Red/Blue increase only at high intensity (hotspots)
        let rb = if v > 5.0 {
            ((v - 5.0) * 50.0).clamp(0.0, 255.0) as u8
        } else {
            0
        };
        Color::Rgb(rb, g, rb)
    }
}
