use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod hologram;
mod organism;
mod simulation;

use simulation::Simulation;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static
{
    let width = 64;
    let height = 32;
    let mut sim = Simulation::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                ].as_ref())
                .split(f.area());

            let area = chunks[0];
            let status_area = chunks[1];

            // Render World
            // We need to map sim coords to terminal coords.
            // Assuming terminal is large enough, or we scale.
            // Let's just use 1:1 for simplicity, centered or top-left.

            let display_width = area.width as usize;
            let display_height = area.height as usize;

            // Render Hologram Background (Channel 0 Magnitude)
            // Or maybe a composite?
            // Let's render Channel 0 (Food) as Green intensity

            let bg_channel = 0;
            if sim.channel_cache.len() > bg_channel {
                let cache = &sim.channel_cache[bg_channel];

                // Find max for normalization
                let max_val = cache.iter().cloned().fold(0.0_f64, f64::max).max(0.001);

                for y in 0..display_height.min(sim.height) {
                    for x in 0..display_width.min(sim.width) {
                        let val = cache[y * sim.width + x];
                        // let intensity = (val / max_val * 255.0) as u8;
                        // let color = Color::Rgb(0, intensity, 0);

                        // We can't easily set background per cell efficiently with Paragraph.
                        // We should use a buffer or canvas.
                        // But `Paragraph` with styled spans works.
                        // Or just render organisms.
                    }
                }
            }

            // Actually, ratatui Canvas is good for pixels, but limited resolution (Braille).
            // Let's use a Paragraph with a String constructed frame.

            // Build the frame buffer
            // We want to show organisms.
            // Hologram background is hard in TUI without true color block characters.
            // Let's stick to showing organisms.

            let mut buffer = vec![vec![' '; sim.width]; sim.height];
            let mut fg_colors = vec![vec![Color::Reset; sim.width]; sim.height];
            let mut bg_colors = vec![vec![Color::Reset; sim.width]; sim.height];

            // Calculate background from Channel 0 (Food)
            if !sim.channel_cache.is_empty() {
                let cache = &sim.channel_cache[0];
                let max_val = cache.iter().cloned().fold(0.0_f64, f64::max).max(0.001);

                for y in 0..sim.height {
                    for x in 0..sim.width {
                        let val = cache[y * sim.width + x];
                        let intensity = (val / max_val * 50.0).min(50.0) as u8; // Dim background
                        if intensity > 5 {
                            bg_colors[y][x] = Color::Rgb(0, intensity, 0);
                        }
                    }
                }
            }

            // Draw organisms
            for org in &sim.organisms {
                if org.x < sim.width && org.y < sim.height {
                    let char_code = match org.channel_idx {
                        0 => 'C', // Consume
                        1 => 'P', // Photo
                        2 => 'D', // Divide
                        3 => 'M', // Move
                        _ => '?',
                    };

                    let color = match org.channel_idx {
                        0 => Color::Green,
                        1 => Color::Yellow,
                        2 => Color::Cyan,
                        3 => Color::Magenta,
                        _ => Color::White,
                    };

                    buffer[org.y][org.x] = char_code;
                    fg_colors[org.y][org.x] = color;
                }
            }

            // Render to Text
            use ratatui::text::{Span, Line, Text};
            let mut lines = Vec::new();
            for y in 0..sim.height {
                let mut spans = Vec::new();
                for x in 0..sim.width {
                    spans.push(Span::styled(
                        buffer[y][x].to_string(),
                        Style::default()
                            .fg(fg_colors[y][x])
                            .bg(bg_colors[y][x]),
                    ));
                }
                lines.push(Line::from(spans));
            }

            let world_widget = Paragraph::new(Text::from(lines))
                .block(Block::default().borders(Borders::ALL).title("Holographic Chimera"));

            f.render_widget(world_widget, area);

            // Status Bar
            let status_text = format!(
                "Tick: {} | Organisms: {} | Q: Quit",
                sim.tick,
                sim.organisms.len()
            );
            f.render_widget(Paragraph::new(status_text), status_area);

        })?;

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sim.step();
            last_tick = Instant::now();
        }
    }
}
