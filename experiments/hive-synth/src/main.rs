pub mod world;

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
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    collections::HashMap,
    io,
    time::{Duration, Instant},
};
use world::{Material, World};

#[cfg(feature = "audio")]
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};

fn y_to_freq(y: usize, _height: usize) -> f32 {
    // Chromatic scale starting from C2 (approx 65.41Hz)
    // y = 0 is bottom (C2)
    let semitone = y as f32;
    let base_freq = 65.41;
    base_freq * 2.0_f32.powf(semitone / 12.0)
}

struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<OutputStreamHandle>,
    enabled: bool,
}

impl AudioEngine {
    fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Self {
                    _stream: Some(stream),
                    stream_handle: Some(handle),
                    enabled: true,
                },
                Err(_) => Self {
                    _stream: None,
                    stream_handle: None,
                    enabled: false,
                },
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            Self { enabled: false }
        }
    }

    fn play_notes(&self, notes: Vec<f32>) {
        // Suppress unused warning when audio feature is disabled
        #[cfg(not(feature = "audio"))]
        let _ = notes;

        if self.enabled {
            #[cfg(feature = "audio")]
            if let Some(handle) = &self.stream_handle {
                for freq in notes {
                    if let Ok(sink) = Sink::try_new(handle) {
                        let source = rodio::source::SineWave::new(freq)
                            .take_duration(Duration::from_millis(100))
                            .amplify(0.05)
                            .fade_in(Duration::from_millis(5))
                            .fade_out(Duration::from_millis(90));
                        sink.append(source);
                        sink.detach();
                    }
                }
            }
        }
    }
}

struct App {
    world: World,
    audio: AudioEngine,
    scan_pos: usize,
    sim_running: bool,
    playback_running: bool,
    exit: bool,
    tempo_ms: u64,
}

impl App {
    fn new() -> Self {
        let width = 100;
        let height = 60;
        let mut world = World::new(width, height);

        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Central server
        for y in height / 2 - 5..height / 2 + 5 {
            for x in width / 2 - 5..width / 2 + 5 {
                world.add_server(x, y);
            }
        }

        // Random Notes
        for _ in 0..1000 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if matches!(world.get_cell(x, y).material, Material::Empty) {
                world.add_note(x, y);
            }
        }

        // Termites
        for _ in 0..500 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            world.add_termite(x, y);
        }

        Self {
            world,
            audio: AudioEngine::new(),
            scan_pos: 0,
            sim_running: true,
            playback_running: true,
            exit: false,
            tempo_ms: 50,
        }
    }

    fn on_tick(&mut self) {
        if self.sim_running {
            self.world.update();
        }

        if self.playback_running {
            let x = self.scan_pos;
            let mut frequencies = Vec::new();

            for y in 0..self.world.height {
                let cell = self.world.get_cell(x, y);
                if matches!(cell.material, Material::Note) {
                    frequencies.push(y_to_freq(y, self.world.height));
                }
            }

            if !frequencies.is_empty() {
                self.audio.play_notes(frequencies);
            }

            self.scan_pos = (self.scan_pos + 1) % self.world.width;
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let mut tick_rate = Duration::from_millis(app.tempo_ms);

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.exit = true,
                            KeyCode::Char(' ') => app.playback_running = !app.playback_running,
                            KeyCode::Enter => app.sim_running = !app.sim_running,
                            KeyCode::Char('+') => {
                                app.tempo_ms = app.tempo_ms.saturating_sub(5).max(10);
                                tick_rate = Duration::from_millis(app.tempo_ms);
                            }
                            KeyCode::Char('-') => {
                                app.tempo_ms = app.tempo_ms.saturating_add(5).min(500);
                                tick_rate = Duration::from_millis(app.tempo_ms);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
            tick_rate = Duration::from_millis(app.tempo_ms);
        }

        if app.exit {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let width = app.world.width as f64;
    let height = app.world.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Hive Synth (Thermo-Tarmites x Cellular-Beats)"),
        )
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|ctx| {
            let mut points_by_color: HashMap<Color, Vec<(f64, f64)>> = HashMap::new();

            for y in 0..app.world.height {
                for x in 0..app.world.width {
                    let cell = app.world.get_cell(x, y);
                    let color = match cell.material {
                        Material::Server => Color::Red,
                        Material::Note => Color::White,
                        Material::Empty => {
                            if cell.heat > 50.0 {
                                Color::Rgb(50, 0, 0)
                            } else if cell.heat > 20.0 {
                                Color::Rgb(20, 0, 0)
                            } else {
                                continue;
                            }
                        }
                    };

                    points_by_color
                        .entry(color)
                        .or_default()
                        .push((x as f64, y as f64));
                }
            }

            for (color, coords) in points_by_color {
                ctx.draw(&Points {
                    coords: &coords,
                    color,
                });
            }

            for termite in &app.world.termites {
                let color = if termite.carrying {
                    Color::Green
                } else {
                    Color::Blue
                };
                ctx.print(
                    termite.x as f64,
                    termite.y as f64,
                    Span::styled("t", Style::default().fg(color)),
                );
            }

            let scan_x = app.scan_pos as f64;
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: scan_x,
                y1: 0.0,
                x2: scan_x,
                y2: height,
                color: Color::Yellow,
            });
        });

    f.render_widget(canvas, chunks[0]);

    let info = Paragraph::new(vec![Line::from(vec![
        Span::raw(" [Space] Play/Pause "),
        Span::raw(" [Enter] Sim On/Off "),
        Span::raw(" [+/-] Tempo "),
        Span::raw(" [q] Quit "),
        Span::raw(format!(" Tempo: {}ms ", app.tempo_ms)),
        Span::raw(format!(" Termites: {} ", app.world.termites.len())),
    ])])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}
