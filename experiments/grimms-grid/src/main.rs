mod grid;
mod phoneme;
mod rules;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use grid::Grid;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::Duration;
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;
    res
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 60;
    let height = 30;
    let mut grid = Grid::new(width, height);

    // Seed with some text initially
    grid.seed_text("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG. ");

    let mut running = true;
    let mut paused = true;
    let speed = Duration::from_millis(100);

    while running {
        tui.terminal.draw(|f| ui(f, &grid, paused))?;

        if event::poll(speed)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => running = false,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') => {
                        grid = Grid::new(width, height);
                        grid.seed_text("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG. ");
                    }
                    KeyCode::Char('s') => grid.seed_random(),
                    KeyCode::Char('n') => grid.update(), // Step
                    _ => {}
                }
            }
        }

        if !paused {
            grid.update();
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, grid: &Grid, paused: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let mut text = String::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(p) = grid.get(x as i32, y as i32) {
                text.push(p.symbol);
            } else {
                text.push(' ');
            }
        }
        text.push('\n');
    }

    let block = Block::default().borders(Borders::ALL).title("Grimm's Grid");
    let p = Paragraph::new(text).block(block);
    f.render_widget(p, chunks[0]);

    let status = format!(
        "Status: {} | [Space] Pause/Resume | [n] Step | [r] Reset Text | [s] Random Seed | [q] Quit",
        if paused { "PAUSED" } else { "RUNNING" }
    );
    let status_bar =
        Paragraph::new(status).style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(status_bar, chunks[1]);
}
