use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Context},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};
use tui_shared::Tui;
use walkdir::WalkDir;

struct RainDrop {
    x: f64,
    y: f64,
    speed: f64,
    stream: Vec<char>,
}

impl RainDrop {
    fn new(x: f64, _width: f64, lines: &[String]) -> Self {
        let mut rng = rand::thread_rng();

        // Pick a random line from source lines, or random chars if empty
        let content = if !lines.is_empty() {
            let line = &lines[rng.gen_range(0..lines.len())];
            // If line is too short, maybe repeat it or pick another?
            // Let's just use it.
            line.chars().collect::<Vec<char>>()
        } else {
            // Fallback
            (0..20)
                .map(|_| rng.gen_range(33..126) as u8 as char)
                .collect()
        };

        // If content is empty (empty line), retry with fallback
        let stream = if content.is_empty() {
            (0..10)
                .map(|_| rng.gen_range(33..126) as u8 as char)
                .collect()
        } else {
            // Make it vertical.
            // Wait, "Matrix" rain is vertical. A line of code is horizontal.
            // Should I rotate the text?
            // Matrix code is usually random chars.
            // "Source Rain" concept: "Use your own source code".
            // Option A: Vertical streams of random chars from your source code's alphabet.
            // Option B: Vertical streams where the chars form the words of your code, read vertically? (Hard to read).
            // Option C: The stream IS the line of code, but displayed vertically?
            // Let's go with Option C: Vertical text.
            content
        };

        Self {
            x,
            y: rng.gen_range(-100.0..0.0), // Start above screen
            speed: rng.gen_range(0.5..2.0),
            stream,
        }
    }
}

struct App {
    drops: Vec<RainDrop>,
    source_lines: Vec<String>,
    width: f64,
    height: f64,
}

impl App {
    fn new() -> Self {
        let source_lines = scan_files(".");
        Self {
            drops: Vec::new(),
            source_lines,
            width: 100.0,
            height: 100.0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Spawn new drops
        // Target density: ~1 drop per 2 horizontal units?
        let target_drops = (self.width / 2.0) as usize;

        if self.drops.len() < target_drops {
            if rng.gen_bool(0.1) {
                let x = rng.gen_range(0.0..self.width);
                self.drops
                    .push(RainDrop::new(x, self.width, &self.source_lines));
            }
        }

        // Update drops
        for drop in &mut self.drops {
            drop.y += drop.speed;
        }

        // Remove off-screen drops
        // Assume screen height is self.height
        // Allow them to go a bit below
        self.drops
            .retain(|d| d.y - (d.stream.len() as f64) < self.height);
    }
}

fn scan_files(root: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    if let Ok(file) = File::open(entry.path()) {
                        let reader = BufReader::new(file);
                        for line in reader.lines().filter_map(|l| l.ok()) {
                            let trimmed = line.trim();
                            if !trimmed.is_empty() {
                                lines.push(trimmed.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    // If no lines found (e.g. running in empty dir), add some defaults
    if lines.is_empty() {
        lines.push("let x = 42;".to_string());
        lines.push("fn main() {}".to_string());
        lines.push("println!(\"Hello World\");".to_string());
    }
    lines
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Update app dimensions based on render area
    app.width = chunks[0].width as f64;
    app.height = chunks[0].height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Source Rain "),
        )
        .x_bounds([0.0, app.width])
        .y_bounds([app.height, 0.0]) // Invert Y so 0 is top
        .paint(|ctx| {
            for drop in &app.drops {
                draw_drop(ctx, drop);
            }
        });

    f.render_widget(canvas, chunks[0]);

    let stats = Paragraph::new(format!(
        "Drops: {} | Source Lines: {} | [Q] Quit",
        app.drops.len(),
        app.source_lines.len()
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats, chunks[1]);
}

fn draw_drop(ctx: &mut Context, drop: &RainDrop) {
    // Draw characters vertically ending at drop.y
    for (i, &ch) in drop.stream.iter().enumerate() {
        let char_y = drop.y - i as f64;

        // If char is mostly off screen, skip?
        // But we handle culling in update.

        let color = if i == 0 {
            Color::White // Head
        } else if i < 3 {
            Color::LightGreen
        } else {
            Color::DarkGray
        };

        // Random glitch effect?
        let mut rng = rand::thread_rng();
        let display_char = if rng.gen_bool(0.01) {
            rng.gen_range(33..126) as u8 as char
        } else {
            ch
        };

        ctx.print(
            drop.x,
            char_y,
            ratatui::text::Span::styled(display_char.to_string(), Style::default().fg(color)),
        );
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let tick_rate = Duration::from_millis(33); // ~30 FPS is enough for rain
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_files() {
        // We expect to find at least this file itself
        let lines = scan_files(".");
        assert!(!lines.is_empty());

        // Check if some specific content from this file is in there
        // Note: scan_files trims lines
        let found = lines.iter().any(|l| l.contains("fn test_scan_files()"));
        assert!(found, "Should have found this test function signature");
    }
}
