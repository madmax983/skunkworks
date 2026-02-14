use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Context, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use resonance_audio::audio::AudioSnapshot;
use std::io;

pub fn init_tui() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_tui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn draw_ui(f: &mut Frame, snapshot: &Option<AudioSnapshot>, msg: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Main Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" ⚛️ Git Harmony ⚛️ "))
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            if let Some(snap) = snapshot {
                draw_snapshot(ctx, snap);
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let p = Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[1]);
}

fn draw_snapshot(ctx: &mut Context, snap: &AudioSnapshot) {
    // Grid is 100x100
    // To optimize, only draw non-zero pressure
    // We check against a small threshold to avoid drawing noise
    let threshold = 0.01;

    for (i, &pressure) in snap.pressure.iter().enumerate() {
        if pressure.abs() > threshold {
            let x = (i % 100) as f64;
            let y = (i / 100) as f64; // Note: In Canvas 0 is bottom, in Grid 0 is usually top. Flipped is fine.

            // Map pressure to color intensity
            // Simple mapping for now
            let color = if pressure > 0.0 {
                Color::Green
            } else {
                Color::Red
            };

            ctx.draw(&Rectangle {
                x,
                y,
                width: 1.0,
                height: 1.0,
                color,
            });
        }
    }
}
