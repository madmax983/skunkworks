mod git;
mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git::get_commit_history;
use physics::Universe;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};
use tui_shared::Tui;

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
    // 200x100 world
    let mut universe = Universe::new(200.0, 100.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    // Get commits and convert them to magnets
    let commits = get_commit_history().unwrap_or_default();

    // We suppress the unused message warning from the git log parser,
    // though the lineage parent used it for display.
    for commit in &commits {
        let _ = &commit.message;
    }

    // Distribute commits as magnets across the screen
    let num_commits = commits.len().max(1);
    for (i, commit) in commits.iter().enumerate() {
        // Space out magnets horizontally
        let x = (i as f64 / num_commits as f64) * 180.0 + 10.0;

        // Use hash to determine height
        let mut hasher = DefaultHasher::new();
        commit.hash.hash(&mut hasher);
        let hash_val = hasher.finish();

        let y = 10.0 + (hash_val % 80) as f64;

        // Use author to determine polarity
        let mut author_hasher = DefaultHasher::new();
        commit.author.hash(&mut author_hasher);
        let author_val = author_hasher.finish();
        let polarity = (author_val % 2) == 0;

        // Strength varies slightly by hash
        let strength = 1000.0 + (hash_val % 2000) as f64;

        universe.add_magnet(x, y, polarity, strength);
    }

    // If no commits, add a default one to make things interesting
    if universe.magnets.is_empty() {
        universe.add_magnet(100.0, 50.0, true, 2000.0);
    }

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Git Fluid (Codebase Magnetohydrodynamics) "))
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            Span::styled("·", Style::default().fg(Color::Cyan)),
                        );
                    }

                    // Draw Commits as Magnets
                    for mag in &universe.magnets {
                        let color = if mag.polarity { Color::Red } else { Color::Blue };
                        let symbol = if mag.polarity { "+" } else { "-" };
                        ctx.print(
                            mag.pos.x,
                            mag.pos.y,
                            Span::styled(symbol, Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let p = Paragraph::new("Press 'q' to quit").block(Block::default().borders(Borders::ALL));
            f.render_widget(p, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.update(0.1); // Fixed timestep
            last_tick = Instant::now();
        }
    }
}
