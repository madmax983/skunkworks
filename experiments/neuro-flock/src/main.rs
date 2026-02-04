use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};

use tui_shared::Tui;
use crossbeam_channel::unbounded;

mod boid;
mod world;
mod neuron;
mod audio;

use world::World;
use audio::{AudioEngine, AudioEvent};

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Setup Audio
    let (audio_tx, audio_rx) = unbounded();
    let _audio = AudioEngine::new(audio_rx)?;

    let res = run_app(&mut tui.terminal, audio_tx);

    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    world: World,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        Self {
            world: World::new(width, height),
            running: true,
        }
    }

    fn on_tick(&mut self, audio_tx: &crossbeam_channel::Sender<AudioEvent>) {
        let spikes = self.world.update();

        // Send spikes to audio engine
        for freq in spikes {
            let _ = audio_tx.send(AudioEvent::PlayFreq(freq));
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    audio_tx: crossbeam_channel::Sender<AudioEvent>
) -> Result<()> {
    // Canvas dimensions (virtual units)
    let world_width = 200.0;
    let world_height = 100.0;

    let mut app = App::new(world_width, world_height);

    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        KeyCode::Char('r') => {
                            // Reset/Reseed
                            app.world = World::new(world_width, world_height);
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick(&audio_tx);
            last_tick = Instant::now();
        }

        if !app.running {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Neuro-Flock: Flying Spiking Neural Network"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            for boid in &app.world.boids {
                // Determine visual style based on spike state
                let (char_str, color) = if boid.spike_timer > 0 {
                    ("★".to_string(), Color::White) // Bright flash
                } else {
                    let base_char = boid.dna.char_representation.to_string();
                    let color = boid.dna.color;
                    (base_char, color)
                };

                ctx.print(
                    boid.position.0,
                    boid.position.1,
                    Span::styled(char_str, Style::default().fg(color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Population: {} | 'r': Reset | 'q': Quit | Audio: {}",
        app.world.boids.len(),
        if cfg!(feature = "audio") { "ON" } else { "OFF (enable feature 'audio')" }
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Blue));
    f.render_widget(p, chunks[1]);
}
