mod git;
mod grid;

use crate::grid::{Grid, HEIGHT, WIDTH};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Setup Data
    let mut grid = Grid::new();
    // Try to get hash, if not in git repo or fails, use random?
    // Or just 0.
    let seed = git::get_head_diff_hash().unwrap_or(0xDEADBEEF);

    // If seed is 0 (empty diff), maybe use rand?
    // No, we want to visualize "Empty Diff" as "Empty Grid" or similar.
    // Actually, empty diff hash is hash_string("") which is 5381.

    git::seed_grid(&mut grid, seed);

    let mut generation: u64 = 0;

    // 2. Setup TUI
    let mut tui = Tui::init()?;

    // 3. Loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut paused = false;

    loop {
        // Stats
        let mut population: usize = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if grid.cells[y][x] {
                    population += 1;
                }
            }
        }

        tui.terminal
            .draw(|f| ui(f, &grid, generation, population, seed, paused))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => paused = !paused,
                        KeyCode::Char('r') => {
                            grid = Grid::new();
                            git::seed_grid(&mut grid, seed);
                            generation = 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                grid.step();
                generation += 1;
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, grid: &Grid, generation: u64, population: usize, seed: u64, paused: bool) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Git Cellular Automata "),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .paint(|ctx| {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if grid.cells[y][x] {
                        // Invert Y because Canvas 0,0 is bottom-left, but array 0,0 is usually top-left logic
                        ctx.print(
                            x as f64,
                            (HEIGHT - 1 - y) as f64,
                            Span::styled("█", Style::default().fg(Color::Green)),
                        );
                    }
                }
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Stats
    let status = if paused { "PAUSED" } else { "RUNNING" };
    let status_color = if paused { Color::Red } else { Color::Green };

    let text = vec![
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled(status, Style::default().fg(status_color)),
        ]),
        Line::from(format!("Generation: {}", generation)),
        Line::from(format!("Population: {}", population)),
        Line::from(format!("Seed Hash: {:x}", seed)),
        Line::from(""),
        Line::from("Controls:"),
        Line::from(" [Space] Pause/Resume"),
        Line::from(" [R]     Reset"),
        Line::from(" [Q]     Quit"),
    ];

    let info = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Stats "));
    f.render_widget(info, chunks[1]);
}
