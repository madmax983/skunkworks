mod brain;

use anyhow::Result;
use brain::Brain;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and start brain
    let grid_width = 20;
    let grid_height = 20;
    let num_neurons = grid_width * grid_height;
    let mut brain = Brain::new(num_neurons);
    brain.start();

    // Run app loop
    let res = run_app(&mut terminal, &brain, grid_width, grid_height);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    // Brain handles threads automatically via Drop, but stop explicitly to be clean
    brain.stop();

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    brain: &Brain,
    width: usize,
    height: usize,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static
{
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50); // 20 FPS

    loop {
        terminal.draw(|f| {
            let size = f.area();

            // Layout: Title at top, Grid below
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let title = Paragraph::new("Neuro-Syncopation: Parallel Neural Dynamics\nBlue: Resting | Red: Spiking | White: Contention Event")
                .block(Block::default().borders(Borders::ALL).title("Controls: 'q' to Quit"));
            f.render_widget(title, chunks[0]);

            // Grid Layout
            // We want a grid of w x h cells.
            // ratatui doesn't have a Grid widget built-in easily for 400 items without custom rendering.
            // But we can render Paragraphs or Blocks in computed Rects.
            // Or use a Canvas for performance? Canvas with points might be better.

            // Let's use simple Block rendering for now, or maybe just characters in a Paragraph?
            // Paragraph with colored blocks "█" is easiest for a grid.

            render_brain_grid(f, chunks[1], brain, width, height);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn render_brain_grid(
    f: &mut ratatui::Frame,
    area: Rect,
    brain: &Brain,
    width: usize,
    height: usize,
) {
    // We will render a grid of characters.
    // Each character represents a neuron.

    let mut rows = Vec::new();

    for y in 0..height {
        let mut row_spans = Vec::new();
        for x in 0..width {
            let idx = y * width + x;
            if idx >= brain.neuron_states.len() {
                break;
            }

            let state = brain.neuron_states[idx].lock();
            let v = state.voltage; // -65 to +30

            // Map voltage to color
            // -65 (Blue) -> 0 (Purple) -> +30 (Red)
            let color = if state.spiked {
                Color::White // Flash white on spike
            } else if state.contention_events > 0 {
                 // Decay contention display?
                 // Since we don't clear contention_events in the loop (we just increment),
                 // this will eventually stay permanently on.
                 // Ideally we should visualize *recent* contention.
                 // But for now, let's just color by voltage.

                 // Map -90..30 to Hue?
                 // -90 -> Blue
                 // -65 -> Cyan/Green
                 // -40 -> Yellow
                 // 30 -> Red
                 if v < -60.0 {
                     Color::Blue
                 } else if v < -40.0 {
                     Color::Cyan
                 } else if v < 0.0 {
                     Color::Yellow
                 } else {
                     Color::Red
                 }
            } else {
                 if v < -60.0 {
                     Color::Blue
                 } else if v < -40.0 {
                     Color::Cyan
                 } else if v < 0.0 {
                     Color::Yellow
                 } else {
                     Color::Red
                 }
            };

            // Use block character
            let cell = ratatui::text::Span::styled("█ ", Style::default().fg(color));
            row_spans.push(cell);
        }
        rows.push(ratatui::text::Line::from(row_spans));
    }

    let p = Paragraph::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Cortex"));
    f.render_widget(p, area);
}
