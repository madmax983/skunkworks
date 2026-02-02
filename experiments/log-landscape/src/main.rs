mod processor;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::VecDeque,
    io::{self, BufRead},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};
use tui_shared::Tui;

use crate::processor::process_line;

const WIDTH: usize = 100; // Number of X buckets
const DEPTH: usize = 50; // Number of Z rows
const TICK_RATE_MS: u64 = 100; // Update rate for terrain movement

struct App {
    // History of rows. Index 0 = Newest (Front). Index N = Oldest (Back).
    terrain: VecDeque<Vec<u8>>,
    current_row: Vec<u8>,
    receiver: Receiver<String>,
    paused: bool,
    lines_processed: u64,
}

impl App {
    fn new(receiver: Receiver<String>) -> Self {
        // Initialize with flat terrain
        let mut terrain = VecDeque::new();
        for _ in 0..DEPTH {
            terrain.push_back(vec![0; WIDTH]);
        }

        Self {
            terrain,
            current_row: vec![0; WIDTH],
            receiver,
            paused: false,
            lines_processed: 0,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        // Consume all available lines from the channel
        // We accumulate them into the 'current_row' (buffer)
        // But actually, we want to scroll continuously.
        // If we receive data, we add it to the 'next' row.
        // But we only push the row every TICK.

        while let Ok(line) = self.receiver.try_recv() {
            self.lines_processed += 1;
            let (bucket, height) = process_line(&line, WIDTH);
            // Additive height? Or max?
            // Let's use additive but capped.
            self.current_row[bucket] = self.current_row[bucket].saturating_add(height).min(50);
        }
    }

    fn tick(&mut self) {
        if self.paused {
            return;
        }

        // Push current row to history
        self.terrain.push_front(self.current_row.clone());
        if self.terrain.len() > DEPTH {
            self.terrain.pop_back();
        }

        // Reset current row for next batch
        self.current_row = vec![0; WIDTH];
    }
}

fn main() -> Result<()> {
    // 1. Setup Stdin Reader
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(l) = line {
                if tx.send(l).is_err() {
                    break;
                }
            } else {
                break;
            }
        }
    });

    // 2. Setup TUI
    let mut tui = Tui::init()?;
    let mut app = App::new(rx);

    // 3. Run Loop
    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update(); // Process pending messages
            app.tick(); // Scroll terrain
            last_tick = Instant::now();
        } else {
            // We still want to process messages even between ticks to avoid channel lag
            app.update();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Draw Terrain
    // We project 3D (x, z, y=height) to 2D screen coordinates.
    // Isometric-ish projection.

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Log Landscape"),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, 60.0]) // Z+Height combined
        .paint(|ctx| {
            // Draw lines
            // Iterate through rows (z)
            for (z, row) in app.terrain.iter().enumerate() {
                let mut prev_x = 0.0;
                let mut prev_y = 0.0;

                for (x, &h) in row.iter().enumerate() {
                    let x_pos = x as f64;
                    let y_pos = h as f64 + z as f64 * 0.8; // z goes 0..DEPTH. 0 is newest.

                    // If z=0 (newest), it's at bottom. y = h.
                    // If z=50 (oldest), it's at top. y = h + 40.

                    if x > 0 {
                        // Horizontal line
                        let color = if h > 5 {
                            Color::Red
                        } else if h > 2 {
                            Color::Yellow
                        } else {
                            Color::Green
                        };

                        ctx.draw(&Line {
                            x1: prev_x,
                            y1: prev_y,
                            x2: x_pos,
                            y2: y_pos,
                            color,
                        });

                        // Vertical/Grid lines (connect to previous Z row)?
                        // Requires access to z-1.
                    }

                    prev_x = x_pos;
                    prev_y = y_pos;
                }

                // Connect to previous Z row (z-1)
                if z > 0 {
                    let prev_row = &app.terrain[z - 1];
                    for (x, &h) in row.iter().enumerate() {
                        let x_pos = x as f64;
                        let y_pos = h as f64 + z as f64 * 0.8;

                        let prev_h = prev_row[x];
                        let prev_y_pos = prev_h as f64 + (z - 1) as f64 * 0.8;

                        let color = Color::DarkGray;
                        ctx.draw(&Line {
                            x1: x_pos,
                            y1: y_pos,
                            x2: x_pos,
                            y2: prev_y_pos,
                            color,
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let status = format!(
        "Processed: {} | Paused: {} | Q: Quit | Space: Pause",
        app.lines_processed, app.paused
    );
    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
