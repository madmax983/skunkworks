mod scan;
mod sim;

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
use sim::World;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Scan
    let root = std::env::current_dir()?;
    let files = scan::scan_codebase(&root)?;

    // 2. Init World
    let mut world = World::new(files);

    // 3. Init TUI
    let mut tui = Tui::init()?;

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let mut paused = false;

    loop {
        tui.terminal.draw(|f| ui(f, &world, paused))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                world.tick();
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, world: &World, paused: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let w = world.width as f64;
    let h = world.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ant Colony - Codebase Maintenance"),
        )
        .x_bounds([0.0, w])
        .y_bounds([0.0, h])
        .paint(|ctx| {
            // Draw Cells
            for (i, cell) in world.cells.iter().enumerate() {
                let x = (i % world.width) as f64;
                // Ratatui Y grows UP, but our grid index grows DOWN usually.
                // Let's invert Y for display so index 0 is top-left.
                // world.height - 1 - (i / width)
                let grid_y = i / world.width;
                let y = (world.height - 1 - grid_y) as f64;

                let color = if let Some(file) = &cell.file {
                    if file.todo_count > 0 {
                        Color::White
                    } else {
                        Color::Gray
                    }
                } else if cell.food_pheromone > 1.0 {
                    Color::Green
                } else if cell.home_pheromone > 1.0 {
                    Color::Blue
                } else {
                    Color::DarkGray
                };

                // Draw Cell Background
                if color != Color::DarkGray {
                    ctx.draw(&Rectangle {
                        x,
                        y,
                        width: 1.0,
                        height: 1.0,
                        color,
                    });
                }
            }

            // Draw Ants
            for ant in &world.ants {
                let grid_y = ant.y;
                let y = (world.height - 1 - grid_y) as f64;

                let color = if ant.has_food {
                    Color::Red
                } else {
                    Color::Yellow
                };
                ctx.print(
                    ant.x as f64 + 0.5,
                    y + 0.5,
                    ratatui::text::Span::styled("●", Style::default().fg(color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Food Collected: {} | Ants: {} | Paused: {} | Press [Space] to Pause, [q] to Quit",
        world.total_food_collected,
        world.ants.len(),
        paused
    );
    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
