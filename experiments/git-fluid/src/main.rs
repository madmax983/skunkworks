mod git;
mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git::{get_commit_history, Commit};
use physics::Universe;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn update_magnets_from_commit(universe: &mut Universe, commit: &Commit) {
    universe.magnets.clear();
    let hash = &commit.hash;

    // Use chunks of the hash to create magnets.
    // Hash is 40 chars long. We can create, say, 5 magnets.
    // 8 chars per magnet.
    // xyz pos, strength, polarity
    if hash.len() < 40 {
        return;
    }

    for i in 0..5 {
        let chunk = &hash[i * 8..(i + 1) * 8];
        // Parse 8 chars into integer
        if let Ok(val) = u32::from_str_radix(chunk, 16) {
            let x_ratio = (val & 0xFF) as f64 / 255.0;
            let y_ratio = ((val >> 8) & 0xFF) as f64 / 255.0;
            let strength_ratio = ((val >> 16) & 0xFF) as f64 / 255.0;
            let polarity = ((val >> 24) & 0x1) == 1;

            // map to universe
            let x = x_ratio * universe.width;
            let y = y_ratio * universe.height;
            // Strength between 10.0 and 100.0
            let strength = 10.0 + strength_ratio * 90.0;

            universe.add_magnet(x, y, strength, polarity);
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let commits = get_commit_history()?;
    let mut commit_idx = 0;

    // 200x100 world
    let mut universe = Universe::new(200.0, 100.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    // Initial magnets
    if !commits.is_empty() {
        update_magnets_from_commit(&mut universe, &commits[commit_idx]);
    }

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Git Fluid "))
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            ratatui::text::Span::styled("·", Style::default().fg(Color::Cyan)),
                        );
                    }

                    // Draw Magnets
                    for mag in &universe.magnets {
                        let color = if mag.polarity { Color::Red } else { Color::Blue };
                        let label = if mag.polarity { "N" } else { "S" };
                        ctx.print(
                            mag.pos.x,
                            mag.pos.y,
                            ratatui::text::Span::styled(label, Style::default().fg(color).bg(Color::White)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let commit_info = if commits.is_empty() {
                "No commits found.".to_string()
            } else {
                let c = &commits[commit_idx];
                format!(
                    "Commit: {} by {} | '{}' (Idx: {}/{}) | Left/Right to change commit | [Q] Quit",
                    &c.hash[0..7],
                    c.author,
                    c.message,
                    commit_idx + 1,
                    commits.len()
                )
            };

            let stats = Paragraph::new(commit_info)
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Left | KeyCode::Char('h') => {
                                if commit_idx > 0 {
                                    commit_idx -= 1;
                                    update_magnets_from_commit(&mut universe, &commits[commit_idx]);
                                }
                            }
                            KeyCode::Right | KeyCode::Char('l') => {
                                if commit_idx + 1 < commits.len() {
                                    commit_idx += 1;
                                    update_magnets_from_commit(&mut universe, &commits[commit_idx]);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.update(0.05); // Fixed time step, keeping as f64 (literals are f64 by default)
            last_tick = Instant::now();
        }
    }
}
