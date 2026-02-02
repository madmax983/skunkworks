mod garden;
mod hamming;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use garden::Garden;
use hamming::HammingStatus;

// Initial seed text: A quote about entropy or data.
const SEED_TEXT: &str = r#"The data was born in the fires of the CPU, forged in the silicon mines of the RAM. It lived a glorious life, calculating, processing, rendering. But time is the enemy of all things, even the digital. Bit by bit, the magnetic domains flipped. The electrons drifted. The format rotted.
What was once meaning became noise. What was once Truth became Null.
Yet, the Gardener walks the rows. With parity and syndrome, they fight the inevitable.
A check here, a correction there. The garden lives, for one more cycle.
-- Genesis, The Archivist"#;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Calculate size based on terminal?
    // Ideally we want to fill the screen.
    // Let's start with a fixed size and let it flow.
    let area = tui.terminal.size()?;
    let width = (area.width - 2) as usize; // account for borders
    let height = (area.height - 10) as usize; // leave room for stats

    let mut app = App::new(width, height, SEED_TEXT);

    run_app(&mut tui, &mut app)?;

    Ok(())
}

struct App {
    garden: Garden,
    tick_rate: Duration,
    last_tick: Instant,
}

impl App {
    fn new(width: usize, height: usize, text: &str) -> Self {
        Self {
            garden: Garden::new(width, height, text),
            tick_rate: Duration::from_millis(50),
            last_tick: Instant::now(),
        }
    }

    fn on_tick(&mut self) {
        self.garden.update();
    }
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = app.tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        // Reset
                        // We need to keep dimensions
                        let w = app.garden.width;
                        let h = app.garden.height;
                        app.garden = Garden::new(w, h, SEED_TEXT);
                    }
                    KeyCode::Up => {
                        app.garden.entropy_rate *= 1.5;
                        if app.garden.entropy_rate > 1.0 { app.garden.entropy_rate = 1.0; }
                    }
                    KeyCode::Down => {
                        app.garden.entropy_rate /= 1.5;
                        if app.garden.entropy_rate < 0.000001 { app.garden.entropy_rate = 0.000001; }
                    }
                    KeyCode::Right => {
                        app.garden.repair_rate = (app.garden.repair_rate as f64 * 1.5) as usize;
                    }
                    KeyCode::Left => {
                        app.garden.repair_rate = (app.garden.repair_rate as f64 / 1.5) as usize;
                        if app.garden.repair_rate < 1 { app.garden.repair_rate = 1; }
                    }
                    _ => {}
                }
            }
        }

        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            app.last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(8),
        ])
        .split(f.area());

    // Render Grid
    let grid_block = Block::default().borders(Borders::ALL).title("Hamming Garden");

    // We render the garden as a paragraph of text
    // Each cell is a nibble.
    // We can just iterate and build lines.

    let inner_area = grid_block.inner(chunks[0]);
    // Re-calculate garden dimensions if screen resized?
    // For now, just render what fits.

    let visible_width = inner_area.width as usize;
    let visible_height = inner_area.height as usize;

    let mut lines = Vec::new();

    for y in 0..visible_height {
        let mut spans = Vec::new();
        for x in 0..visible_width {
            let idx = y * app.garden.width + x; // This mapping might be wrong if garden width != visible width
            // Let's assume garden width is large enough, or wrap.
            // Actually garden is 1D vector logically, wrapped at width.

            if idx < app.garden.cells.len() {
                let cell = &app.garden.cells[idx];

                // Decode to get current value (even if corrupted)
                // If clean/corrected, it's the data.
                // If double error, it's potentially garbage data.
                // Note: cell.encoded_byte is what we have.
                let (data, _) = hamming::decode(cell.encoded_byte);

                let char_val = format!("{:X}", data);

                let style = match cell.status {
                    HammingStatus::Clean => Style::default().fg(Color::DarkGray),
                    HammingStatus::Corrected(_) => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    HammingStatus::DoubleError => Style::default().fg(Color::Red).bg(Color::Black).add_modifier(Modifier::BOLD),
                };

                spans.push(Span::styled(char_val, style));
            } else {
                spans.push(Span::raw(" "));
            }
        }
        lines.push(Line::from(spans));
    }

    let p = Paragraph::new(lines).block(grid_block);
    f.render_widget(p, chunks[0]);

    // Render Stats
    let stats_block = Block::default().borders(Borders::ALL).title("Status");

    let entropy_percent = app.garden.entropy_rate * 100.0;

    let stats_text = vec![
        Line::from(vec![
            Span::styled("Entropy Rate (Up/Down): ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{:.6}%", entropy_percent)),
        ]),
        Line::from(vec![
            Span::styled("Repair Rate (Left/Right): ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{} cells/tick", app.garden.repair_rate)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total Flips: ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{}", app.garden.total_flips)),
        ]),
        Line::from(vec![
            Span::styled("Total Repairs: ", Style::default().fg(Color::Green)),
            Span::raw(format!("{}", app.garden.total_repairs)),
        ]),
        Line::from(vec![
            Span::styled("Total Deaths (Unrecoverable): ", Style::default().fg(Color::Red)),
            Span::raw(format!("{}", app.garden.total_deaths)),
        ]),
        Line::from(""),
        Line::from(Span::styled("Press 'r' to Reset, 'q' to Quit", Style::default().add_modifier(Modifier::ITALIC))),
    ];

    f.render_widget(Paragraph::new(stats_text).block(stats_block), chunks[1]);
}
