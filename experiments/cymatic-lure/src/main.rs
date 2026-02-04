mod sim;
mod audio;

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tui_shared::Tui;

use sim::WaveTank;
use audio::ScannedSynth;

struct App {
    tank: WaveTank,
    synth: ScannedSynth,
    running: bool,
    freq: f32,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        Self {
            tank: WaveTank::new(width, height),
            synth: ScannedSynth::new(),
            running: true,
            freq: 110.0, // Low A
        }
    }

    fn on_tick(&mut self) {
        self.tank.step();

        // Scan the middle row
        let mid = self.tank.height / 2;
        let row = self.tank.get_row(mid);

        // Push 1/60th of a second of audio
        // Frequency varies slightly with total energy to make it "sing"?
        // Or just fixed pitch modulated by wave shape (timbre).
        self.synth.push_frame(row, 1.0 / 60.0, self.freq);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::init()?;

    // Size tank based on terminal
    let size = tui.terminal.size()?;
    let width = size.width as usize;
    let height = (size.height.saturating_sub(2)) as usize;

    let mut app = App::new(width, height);

    // Initial Drop
    app.tank.rain(5);

    let res = run_app(&mut tui.terminal, &mut app);

    // Save audio
    if let Err(e) = app.synth.save("cymatic_session.wav") {
        eprintln!("Failed to save audio: {}", e);
    } else {
        // We can't print easily after TUI restore unless we ensure it's restored.
        // tui-shared Tui restores on Drop.
    }

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            // Render Tank
            // We build a text buffer.
            let mut lines = Vec::new();

            // Map height to chars
            let chars = [' ', ' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

            for y in 0..app.tank.height {
                let mut current_string = String::new();

                // Simple optimization: just one string per line if monocolor
                // But we want color for height!

                for x in 0..app.tank.width {
                    let h = app.tank.get_height(x, y);
                    // normalize roughly -1.0 to 1.0 to 0..8
                    let idx = ((h + 0.5).clamp(0.0, 1.0) * (chars.len() - 1) as f32) as usize;
                    let c = chars[idx];

                    current_string.push(c);
                }

                // For speed, just cyan for now.
                lines.push(Line::from(Span::styled(current_string, Style::default().fg(Color::Cyan))));
            }

            let tank_widget = Paragraph::new(lines);
            f.render_widget(tank_widget, chunks[0]);

            // Status
            let status = format!("Freq: {:.1}Hz | Click/Drag to Ripple | 'r' Rain | 'q' Quit", app.freq);
            f.render_widget(
                Paragraph::new(status).block(Block::default().borders(Borders::TOP)),
                chunks[1],
            );
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        app.running = false;
                    }
                    if key.code == KeyCode::Char('r') {
                        app.tank.rain(5);
                    }
                    if key.code == KeyCode::Up {
                        app.freq += 10.0;
                    }
                    if key.code == KeyCode::Down {
                        app.freq -= 10.0;
                    }
                }
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::Down(crossterm::event::MouseButton::Left)
                       || mouse.kind == MouseEventKind::Drag(crossterm::event::MouseButton::Left) {
                        let x = mouse.column as usize;
                        let y = mouse.row as usize;
                        app.tank.disturb(x, y, 2.0);
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }
    Ok(())
}
