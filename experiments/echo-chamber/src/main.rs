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
    text::Span,
    widgets::{
        canvas::Canvas,
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

// --- Audio Engine ---

#[cfg(feature = "audio")]
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};

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

    fn play_char(&self, c: char) {
        if !self.enabled {
            return;
        }

        #[cfg(feature = "audio")]
        if let Some(handle) = &self.stream_handle {
            if let Ok(sink) = Sink::try_new(handle) {
                // Map char to frequency (Pentatonic Scale-ish)
                // C4 = 261.63
                let base_freq = 261.63;
                let scale = [1.0, 1.125, 1.25, 1.5, 1.667, 2.0]; // Major Pentatonic ratios
                let idx = (c as usize) % scale.len();
                let octave = ((c as usize) / scale.len()) % 3; // 3 octaves

                let freq = base_freq * scale[idx] * 2.0f32.powi(octave as i32);

                // Simple envelope
                let source = rodio::source::SineWave::new(freq)
                    .take_duration(Duration::from_millis(200))
                    .amplify(0.10)
                    .fade_in(Duration::from_millis(10))
                    .fade_out(Duration::from_millis(190));

                sink.append(source);
                sink.detach();
            }
        }

        // Suppress unused variable warning when audio feature is disabled
        #[cfg(not(feature = "audio"))]
        let _ = c;
    }
}

// --- Particle System ---

#[derive(Clone, Copy)]
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    char: char,
    color: Color,
    life: f64,     // 0.0 to 1.0
    decay: f64,    // per second
}

struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    fn spawn(&mut self, x: f64, y: f64, c: char, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let speed = rng.gen_range(10.0..30.0); // Characters per second

            // Randomize color slightly based on char
            let colors = [
                Color::Red,
                Color::Green,
                Color::Blue,
                Color::Yellow,
                Color::Magenta,
                Color::Cyan,
                Color::White,
            ];
            let color = colors[rng.gen_range(0..colors.len())];

            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed * 0.5, // Terminal cells are roughly 1:2 aspect ratio
                char: c,
                color,
                life: 1.0,
                decay: rng.gen_range(0.5..2.0),
            });
        }
    }

    fn update(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= p.decay * dt;

            // Gravity effect
            p.vy += 20.0 * dt;
        }

        self.particles.retain(|p| p.life > 0.0);
    }
}

// --- App ---

struct App {
    text: String,
    particles: ParticleSystem,
    audio: AudioEngine,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            text: String::new(),
            particles: ParticleSystem::new(),
            audio: AudioEngine::new(),
            exit: false,
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.text.push(c);
                self.audio.play_char(c);

                // Spawn particles at "approximate" cursor position
                // We'll calculate it based on text length for simplicity in this demo
                // A real text editor would need complex cursor tracking
                // Assuming terminal width ~80 chars for effect
                let screen_width = 80;
                let x = (self.text.len() % screen_width) as f64;
                let y = (self.text.len() / screen_width) as f64;

                // Adjust to canvas coordinates (0,0 is bottom left usually, but we can map it)
                // Actually canvas logic handles coords.
                // Let's assume we map 1:1 text chars to canvas units.

                // Visual "Echo"
                self.particles.spawn(x, -y, c, 5);
            }
            KeyCode::Backspace => {
                let _ = self.text.pop();
            }
            KeyCode::Enter => {
                self.text.push('\n');
                self.audio.play_char('\n');
            }
            KeyCode::Esc => {
                self.exit = true;
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, dt: f64) {
        self.particles.update(dt);
    }
}

// --- Main ---

use rand::prelude::*;

fn main() -> Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Teardown
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key.code);
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            app.on_tick(dt);
            last_tick = Instant::now();
        }

        if app.exit {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Main Area: Text + Particles
    let main_area = chunks[0];

    // 1. Render Text Layer (Background)
    // We wrap the text to fit the area
    let text_block = Paragraph::new(app.text.as_str())
        .wrap(ratatui::widgets::Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title(" Echo Chamber "));
    f.render_widget(text_block, main_area);

    // 2. Render Particle Layer (Foreground)
    // We use a Canvas overlaid on top
    // We need to coordinate the coordinate system.
    // Text Paragraph wraps at width.
    // Canvas coords are arbitrary float.

    let canvas_area = main_area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
    // Adjust bounds to inner area
    let inner_width = canvas_area.width as f64;
    let inner_height = canvas_area.height as f64;

    // Re-create canvas with inner bounds
    let canvas = Canvas::default()
        .x_bounds([0.0, inner_width])
        .y_bounds([-inner_height, 0.0]) // Text grows down
        .paint(|ctx| {
             for p in &app.particles.particles {
                let symbol = if p.life > 0.7 { p.char.to_string() } // Show the actual char first
                             else if p.life > 0.5 { "*".to_string() }
                             else { ".".to_string() };

                ctx.print(p.x, p.y, Span::styled(symbol, Style::default().fg(p.color)));
            }
        });

    f.render_widget(canvas, canvas_area);

    // Bottom Bar: Instructions
    let footer = Paragraph::new("Type to create echoes. [ESC] to quit.")
        .style(Style::default().add_modifier(ratatui::style::Modifier::ITALIC))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(footer, chunks[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_spawn() {
        let mut sys = ParticleSystem::new();
        sys.spawn(10.0, 10.0, 'A', 5);
        assert_eq!(sys.particles.len(), 5);
        assert_eq!(sys.particles[0].char, 'A');
    }

    #[test]
    fn test_particle_update() {
        let mut sys = ParticleSystem::new();
        sys.spawn(0.0, 0.0, 'X', 1);
        let initial_life = sys.particles[0].life;

        sys.update(0.1);

        assert!(sys.particles[0].life < initial_life);
    }
}
