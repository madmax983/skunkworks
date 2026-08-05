//! # Chron Fluid ⏳🧲
//!
//! **"Magnetic Code Strata"**
//!
//! A hybrid experiment combining Git Blame history with fluid dynamics.
//!
//! ## 🧬 Lineage
//!
//! - **Parent A:** `experiments/chrontext` (Git Blame visualization)
//! - **Parent B:** `experiments/ferrous-fluid` (Magnetic particle simulation)
//! - **Genetic Engineer:** The Splice Surgeon
//!
//! ## 🔬 Concept
//!
//! The age of each line of code in a file creates a magnetic field.
//! - **Ancient Code (Blue):** Acts as a strong magnetic attractor (South pole).
//! - **Recent Code (Red):** Acts as a strong magnetic repulsor (North pole).
//!
//! Particles flow over the codebase, naturally settling into the "cold" bedrock of old, stable code, while being actively repelled by "hot", actively changing areas.
//!
//! ## 🕹️ Controls
//!
//! - **Up/Down Arrow**: Scroll through the file.
//! - **R**: Scatter particles randomly across the screen.
//! - **Q / Esc**: Quit.
//!
//! ## 🧠 The "Why"
//!
//! By visualizing code history not just as color, but as a physical force field, we can tangibly "feel" the stability of a codebase. Particles pooling in one area show a stable foundation, while turbulent repulsions show active development zones.
//!
mod blame;
mod physics;

use anyhow::Result;
use blame::BlameAnalyzer;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use physics::Universe;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chron-fluid <file_path>");
        return Ok(());
    }
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);

    // Analyze Git Blame Data
    println!("Analyzing {}...", file_path_str);
    let analyzer = BlameAnalyzer::new(".");
    let blame_info = analyzer.analyze(file_path)?;
    let file = fs::File::open(file_path)?;
    let mut content = String::new();
    let limit = 1024 * 1024; // 1MB limit
    let bytes_read = file.take(limit + 1).read_to_string(&mut content)?;

    if bytes_read as u64 > limit {
        anyhow::bail!("File {:?} exceeds 1MB limit", file_path);
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui, file_path_str, blame_info, content);
    tui.exit()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    tui: &mut Tui,
    title: &str,
    blame_info: Vec<blame::LineInfo>,
    content: String,
) -> Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    let num_lines = lines.len();

    // We map the lines of code onto the physics universe grid.
    // The width is somewhat arbitrary (max line length), height depends on num lines.
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(80) as f64;
    let width = max_len.max(80.0) + 10.0;
    let height = num_lines as f64 + 10.0;

    let mut universe = Universe::new(width, height);

    // Convert blame data into magnetic forces on the platter
    // "Hot" code (recent) has high age score, making it a repulsor (North).
    // "Cold" code (old) has low age score, making it an attractor (South).
    for info in &blame_info {
        // Blame lines are 1-based.
        let y = info.line_number as f64;
        if y < height {
            let line_idx = info.line_number.saturating_sub(1);
            if let Some(line) = lines.get(line_idx) {
                // Score ranges from 0.0 (old) to 1.0 (new)
                // Let's make old code strong attractors (South), new code strong repulsors (North)
                // Normalize to [-1.0, 1.0] where 1.0 is newest, -1.0 is oldest.
                let strength_mult = info.age_score * 2.0 - 1.0;

                // Add a line-wide magnetic field to the platter
                for x in 0..line.len() {
                    universe
                        .platter
                        .accumulate(x, y as usize, strength_mult * 5.0);
                }
            }
        }
    }

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    // Initial camera scroll
    let mut scroll_y = 0.0;

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(" Chron-Fluid: {} ", title)),
                )
                .paint(|ctx| {
                    let view_height = chunks[0].height as f64;
                    // Render lines of text
                    for (i, line) in lines.iter().enumerate() {
                        let y = i as f64;
                        if y >= scroll_y && y < scroll_y + view_height {
                            let draw_y = height - y; // Canvas is Y-up, text is Y-down

                            // Find blame info for color
                            let mut score = 0.5;
                            for info in &blame_info {
                                if info.line_number == i + 1 {
                                    score = info.age_score;
                                    break;
                                }
                            }

                            // Color mapping (Old = Blue, New = Red)
                            let r = (score * 255.0) as u8;
                            let b = ((1.0 - score) * 255.0) as u8;
                            let color = Color::Rgb(r, 100, b);

                            ctx.print(
                                0.0,
                                draw_y,
                                Span::styled(line.to_string(), Style::default().fg(color)),
                            );
                        }
                    }

                    // Render particles
                    for p in &universe.particles {
                        let draw_y = height - p.pos.y;
                        if draw_y >= 0.0 && draw_y <= height {
                            ctx.print(
                                p.pos.x,
                                draw_y,
                                Span::styled(
                                    "•",
                                    Style::default()
                                        .fg(Color::Yellow)
                                        .add_modifier(Modifier::BOLD),
                                ),
                            );
                        }
                    }
                })
                .x_bounds([0.0, width])
                .y_bounds([
                    height - scroll_y - chunks[0].height as f64,
                    height - scroll_y,
                ]); // Scrolling bounds

            f.render_widget(canvas, chunks[0]);

            let instructions = Paragraph::new(Line::from(vec![
                Span::raw(" Use "),
                Span::styled("Up/Down", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to scroll, "),
                Span::styled("R", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to scatter particles, "),
                Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quit. Old code (Blue) attracts, New code (Red) repels."),
            ]))
            .block(Block::default().borders(Borders::ALL));

            f.render_widget(instructions, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('r') => {
                            universe.scatter();
                        }
                        KeyCode::Up => {
                            scroll_y = (scroll_y - 1.0).max(0.0);
                        }
                        KeyCode::Down => {
                            scroll_y = (scroll_y + 1.0).min(height - 10.0); // Arbitrary max scroll
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.update(0.1);
            last_tick = Instant::now();
        }
    }
}
