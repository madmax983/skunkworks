use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    path::Path,
    time::{Duration, Instant},
};

mod radio;
mod scanner;

use radio::{demodulate, Tuner};
use scanner::{scan_workspace, Station};

struct App {
    stations: Vec<Station>,
    tuner: Tuner,
}

impl App {
    fn new(stations: Vec<Station>) -> Self {
        Self {
            stations,
            tuner: Tuner::new(),
        }
    }

    fn on_tick(&mut self) {
        self.tuner.update();
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load Data
    // For this experiment, we scan the current directory
    let root = Path::new(".");
    let stations = scan_workspace(root)?;

    // Create App
    let mut app = App::new(stations);

    // Run Loop
    let res = run_app(&mut terminal, &mut app);

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Left => app.tuner.velocity -= 0.05,
                        KeyCode::Right => app.tuner.velocity += 0.05,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(50), Constraint::Percentage(20)])
        .split(f.area());

    // Spectrum Analyzer
    let bandwidth = 0.5; // MHz
    let tuned_freq = app.tuner.freq;

    // Find nearest station
    let mut nearest_station: Option<&Station> = None;
    let mut min_dist = f64::MAX;

    for station in &app.stations {
        let dist = (station.freq - tuned_freq).abs();
        if dist < min_dist {
            min_dist = dist;
            nearest_station = Some(station);
        }
    }

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Spectrum Analyzer"))
        .x_bounds([88.0, 108.0])
        .y_bounds([0.0, 10.0])
        .paint(|ctx| {
            // Draw Stations
            for station in &app.stations {
                // Height based on size
                let height = (station.size as f64).log10() * 2.0;

                // Color based on proximity to tuner
                let dist = (station.freq - app.tuner.freq).abs();
                let color = if dist < bandwidth {
                    Color::Green
                } else {
                    Color::DarkGray
                };

                ctx.draw(&Line {
                    x1: station.freq,
                    y1: 0.0,
                    x2: station.freq,
                    y2: height,
                    color,
                });
            }

            // Draw Tuner Needle
            ctx.draw(&Line {
                x1: app.tuner.freq,
                y1: 0.0,
                x2: app.tuner.freq,
                y2: 10.0,
                color: Color::Red,
            });
        });

    f.render_widget(canvas, chunks[0]);

    // Monitor (Demodulated Text)
    let content = if let Some(station) = nearest_station {
        if min_dist < bandwidth * 2.0 {
             demodulate(station, tuned_freq, bandwidth)
        } else {
             demodulate(station, tuned_freq + 100.0, bandwidth) // Force static
        }
    } else {
        "NO SIGNAL".to_string()
    };

    // Slice content to fit screen roughly (avoid rendering huge strings)
    // Just take first 20 lines
    let display_content: String = content.lines().take(20).collect::<Vec<_>>().join("\n");

    f.render_widget(
        Paragraph::new(display_content)
            .block(Block::default().borders(Borders::ALL).title("Monitor"))
            .style(Style::default().fg(Color::Green)),
        chunks[1],
    );

    // Info Panel
    let info = if let Some(s) = nearest_station {
        format!(
            "Frequency: {:.2} MHz\nStation: {}\nSize: {} bytes\nSignal Delta: {:.4}",
            tuned_freq,
            s.path.display(),
            s.size,
            min_dist
        )
    } else {
        format!("Frequency: {:.2} MHz\nSearching...", tuned_freq)
    };

    f.render_widget(
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Tuner Info")),
        chunks[2],
    );
}
