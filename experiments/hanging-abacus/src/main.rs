use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};

mod abacus_plant;
mod lsystem;
mod turtle;

use abacus_plant::get_lsystem_for_column;
use soroban::Soroban;
use tui_shared::Tui;
use turtle::Turtle;

const PLANT_ITERATIONS: u32 = 4;
const PLANT_STEP_SIZE: f64 = 4.0;
const PLANT_ANGLE_INCREMENT: f64 = std::f64::consts::PI / 6.0; // 30 degrees

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut soroban = Soroban::new();
    let mut input_buffer = String::new();
    let mut input_error = String::new();
    let mut should_quit = false;

    // Animation / Refresh loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(area);

            let canvas_area = chunks[0];
            let width = canvas_area.width as f64;
            let height = canvas_area.height as f64 * 4.0; // Scale up vertical resolution for canvas

            // Draw Plants
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Hanging Abacus "),
                )
                .x_bounds([0.0, width])
                .y_bounds([0.0, height])
                .paint(|ctx| {
                    let col_spacing = width / 14.0; // 13 columns + margins

                    for (i, col) in soroban.columns.iter().enumerate() {
                        if i >= 13 {
                            break;
                        } // Safety against future column size changes

                        // Position: Col 0 (Ones) is rightmost usually on Soroban
                        // But let's map left-to-right: Col 12 (Trillions) -> Col 0 (Ones)
                        // Actually, index 0 is ones place in soroban crate.
                        // So index 0 should be rightmost?
                        // Let's do standard math notation: high power on left.
                        // Col 12 is 10^12. Should be left.
                        // Col 0 is 10^0. Should be right.

                        let visual_idx = 12usize.saturating_sub(i); // 0->12 (right), 12->0 (left)
                        let x = (visual_idx as f64 + 1.0) * col_spacing;
                        let y_start = height - 10.0; // Hang from top (height is top in our flipped view logic?)

                        // Wait, Canvas (0,0) is bottom-left.
                        // If we want to hang from top, start Y should be near `height`.
                        // And grow downwards.
                        // Turtle normally grows upwards (y increases).
                        // If we start at top, angle should be -PI/2 (down).

                        let lsys = get_lsystem_for_column(col);
                        let instructions = lsys.expand(PLANT_ITERATIONS);

                        let mut turtle = Turtle::new(
                            x,
                            y_start,
                            -std::f64::consts::FRAC_PI_2,
                            PLANT_STEP_SIZE,
                            PLANT_ANGLE_INCREMENT,
                        );
                        let lines = turtle.interpret(&instructions);

                        for line in lines {
                            // Colors based on bead status?
                            // Heaven active -> Red flower?
                            // Earth active -> Green leaves?

                            let color = if col.upper_active {
                                Color::Red
                            } else {
                                Color::Green
                            };

                            ctx.draw(&CanvasLine {
                                x1: line.x1,
                                y1: line.y1,
                                x2: line.x2,
                                y2: line.y2,
                                color,
                            });
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            // Status Bar
            let soroban_val = soroban.value();
            let status_text = format!(
                "Value: {} | Input: {} | [0-9] Type | [Enter] Add | [Bksp] Clear | [q] Quit | {}",
                soroban_val, input_buffer, input_error
            );

            f.render_widget(
                Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
                chunks[1],
            );
        })?;

        // Handle Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => should_quit = true,
                        KeyCode::Char(c) if c.is_digit(10) => {
                            if input_buffer.len() < 13 {
                                input_buffer.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            if let Ok(val) = input_buffer.parse::<u64>() {
                                soroban.add(val);
                                input_buffer.clear();
                                input_error.clear();
                            } else {
                                input_error = "Invalid Number".to_string();
                            }
                        }
                        KeyCode::Esc => {
                            input_buffer.clear();
                        }
                        _ => {}
                    }
                }
            }
        }

        let elapsed = last_tick.elapsed();
        if elapsed < tick_rate {
            std::thread::sleep(tick_rate - elapsed);
        }
        last_tick = Instant::now();

        if should_quit {
            break;
        }
    }

    Ok(())
}
