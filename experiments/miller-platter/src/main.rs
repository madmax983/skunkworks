use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use miller_lattice::Crystal;
use platter::Platter;
// Use TUI's re-exported ratatui
use tui_shared::ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use tui_shared::Tui;

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting immediately.");
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
    let root_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

    let width = 200;
    let height = 100;
    let mut platter = Platter::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);
    let mut running = true;

    while running {
        tui.terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(area);

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Miller Platter: Codebase Thermodynamic Shadow"))
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    for y in 0..height {
                        for x in 0..width {
                            let val = platter.get(x, y);
                            if val > 0.01 {
                                // Map val (0.0 -> 1.0+) to colors
                                let color = if val > 0.8 {
                                    Color::White
                                } else if val > 0.5 {
                                    Color::Yellow
                                } else if val > 0.2 {
                                    Color::Red
                                } else {
                                    Color::DarkGray
                                };
                                ctx.print(x as f64, y as f64, tui_shared::ratatui::text::Span::styled("█", Style::default().fg(color)));
                            }
                        }
                    }
                });
            f.render_widget(canvas, chunks[0]);

            let status = format!(
                " Atoms: {} | Heat generated from codebase structure",
                crystal.atoms.len()
            );
            f.render_widget(
                Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
                chunks[1],
            );
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc) {
                    running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Apply heat based on crystal structure
            for atom in &crystal.atoms {
                // Map lattice 3D coords to 2D platter (flatten Z)
                // Offset and scale so it fits on screen
                let px = (atom.position.x * 5 + width as i32 / 2) as i32;
                let py = (atom.position.y * 5 + height as i32 / 2) as i32;

                // Heat amount based on depth (Z)
                let heat = 0.05 + (atom.position.z.abs() as f64 * 0.01);

                if px >= 0 && py >= 0 && (px as usize) < width && (py as usize) < height {
                    platter.accumulate(px as usize, py as usize, heat);
                }
            }

            // Diffuse heat
            platter.decay(0.95);
            last_tick = Instant::now();
        }
    }

    Ok(())
}
