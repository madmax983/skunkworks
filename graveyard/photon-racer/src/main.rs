use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod game;
mod physics;

use game::{Cell, GameMode, GameState};

fn main() -> Result<()> {
    // Initialize TUI
    let mut tui = Tui::init()?;

    // Run app
    let res = run_app(&mut tui.terminal);

    // Restore terminal
    drop(tui);

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

#[allow(clippy::collapsible_if)]
fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // Game dimensions
    let width = 40;
    let height = 20;

    let mut state = GameState::new(width, height);

    let tick_rate = Duration::from_millis(50); // 20 FPS
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        terminal.draw(|f| ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => running = false,

                        // Edit Controls
                        KeyCode::Left => {
                            if state.cursor.0 > 0 {
                                state.cursor.0 -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if state.cursor.0 < state.grid.width - 1 {
                                state.cursor.0 += 1;
                            }
                        }
                        KeyCode::Up => {
                            if state.cursor.1 > 0 {
                                state.cursor.1 -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if state.cursor.1 < state.grid.height - 1 {
                                state.cursor.1 += 1;
                            }
                        }

                        // Placing Items (Only in Edit Mode)
                        KeyCode::Char('/') if state.mode == GameMode::Edit => {
                            state
                                .grid
                                .set(state.cursor.0, state.cursor.1, Cell::MirrorSlash);
                        }
                        KeyCode::Char('\\') if state.mode == GameMode::Edit => {
                            state
                                .grid
                                .set(state.cursor.0, state.cursor.1, Cell::MirrorBackslash);
                        }
                        KeyCode::Char('#') if state.mode == GameMode::Edit => {
                            state.grid.set(state.cursor.0, state.cursor.1, Cell::Block);
                        }
                        KeyCode::Char('s') if state.mode == GameMode::Edit => {
                            state.grid.set(state.cursor.0, state.cursor.1, Cell::Source);
                        }
                        KeyCode::Char('t') if state.mode == GameMode::Edit => {
                            state.grid.set(state.cursor.0, state.cursor.1, Cell::Target);
                        }
                        KeyCode::Char('x') if state.mode == GameMode::Edit => {
                            // 'x' to clear cell. Space triggers fire, so we use x to clear.
                            state.grid.set(state.cursor.0, state.cursor.1, Cell::Empty);
                        }

                        // Actions
                        KeyCode::Char(' ') => {
                            if state.mode == GameMode::Edit {
                                state.fire();
                            }
                        }
                        KeyCode::Char('r') => {
                            state.reset();
                        }

                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            state.update();
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn ui(f: &mut ratatui::Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Calculate canvas size based on grid
    // Each cell is 2x2 pixels on canvas for better visibility?
    // Or just 1x1. Let's try 1x1 logical units, but mapped to screen.
    // Ratatui Canvas coordinates are f64.

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Photon Racer "),
        )
        .x_bounds([0.0, state.grid.width as f64])
        .y_bounds([0.0, state.grid.height as f64])
        .paint(|ctx| {
            // Draw Grid
            for y in 0..state.grid.height {
                for x in 0..state.grid.width {
                    // Ratatui Y is usually bottom-up, but we can flip it if needed.
                    // Or just act like it's cartesian.
                    // Let's assume (0,0) is bottom-left for Canvas.
                    // But our grid is (0,0) top-left usually.
                    // Let's invert Y: draw_y = height - 1 - y

                    let draw_y = (state.grid.height - 1 - y) as f64;
                    let draw_x = x as f64;

                    if let Some(cell) = state.grid.get(x, y) {
                        let (char_str, color) = match cell {
                            Cell::Empty => continue,
                            Cell::Block => ("#", Color::Gray),
                            Cell::MirrorSlash => ("/", Color::Cyan),
                            Cell::MirrorBackslash => ("\\", Color::Cyan),
                            Cell::Target => ("@", Color::Green),
                            Cell::Source => ("*", Color::Yellow),
                        };

                        ctx.print(
                            draw_x + 0.5,
                            draw_y + 0.5,
                            Span::styled(char_str, Style::default().fg(color)),
                        );
                    }
                }
            }

            // Draw Photon Trail
            if let Some(ref photon) = state.photon {
                for pos in &photon.trail {
                    let draw_y = state.grid.height as f64 - 1.0 - pos.y;
                    let draw_x = pos.x;
                    ctx.print(
                        draw_x + 0.5,
                        draw_y + 0.5,
                        Span::styled(".", Style::default().fg(Color::DarkGray)),
                    );
                }

                // Draw Photon Head
                if photon.active {
                    let draw_y = state.grid.height as f64 - 1.0 - photon.position.y;
                    let draw_x = photon.position.x;
                    ctx.print(
                        draw_x + 0.5,
                        draw_y + 0.5,
                        Span::styled("O", Style::default().fg(Color::White)),
                    );
                }
            }

            // Draw Cursor
            let cursor_y = (state.grid.height - 1 - state.cursor.1) as f64;
            let cursor_x = state.cursor.0 as f64;

            // Draw a bracket around cursor or just a different bg?
            // Canvas doesn't support BG easily on print.
            // We can draw a '+' on top.
            ctx.print(
                cursor_x + 0.5,
                cursor_y + 0.5,
                Span::styled("+", Style::default().fg(Color::Red)),
            );
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let mode_str = match state.mode {
        GameMode::Edit => "EDIT MODE",
        GameMode::Run => "RUNNING",
    };

    let status_text = vec![
        Span::styled(
            format!("[{}] ", mode_str),
            Style::default()
                .fg(Color::Blue)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        Span::raw(format!(
            "Cursor: ({}, {}) | ",
            state.cursor.0, state.cursor.1
        )),
        Span::raw(&state.message),
    ];

    let help_text = Line::from(vec![Span::raw(
        "Controls: Arrows=Move, [/ \\ # s t]=Place, Space=Fire, r=Reset, q=Quit",
    )]);

    let p = Paragraph::new(vec![Line::from(status_text), help_text])
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(p, chunks[1]);
}
