//! # chaos-resonance
//!
//! ## Lineage
//! - **chaos-pendulum** (`experiments/chaos-pendulum`): Provides the chaotic double-pendulum node system.
//! - **resonance-audio** (`crates/resonance-audio`): Provides the 2D FDTD continuous acoustic wave simulation engine.
//!
//! ## Concept
//! Acoustic Chaos Volatility. The continuous, unpredictable path of a double pendulum is embedded within an FDTD acoustic wave simulation. As the chaotic pendulum swings, it plucks the acoustic grid, translating non-repeating mathematical chaos into continuous resonant waves and physical interference patterns.
//!
//! ## Novel Trait
//! An acoustic generator where true, non-repeating physical chaos drives cymatic interference patterns and acoustic pressure waves.
//!
use std::env;
use std::io::{self};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use chaos_pendulum::{Link, Node, PendulumSystem};
use resonance_audio::{AudioCommand, AudioModel, AudioSnapshot};

const TICK_RATE: Duration = Duration::from_millis(16);
const AUDIO_WIDTH: usize = 120;
const AUDIO_HEIGHT: usize = 60;

struct App {
    system: PendulumSystem,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: Receiver<AudioSnapshot>,
    latest_snapshot: Option<AudioSnapshot>,
    running: bool,
    ticks: u64,
}

impl App {
    fn new(cmd_tx: Sender<AudioCommand>, snap_rx: Receiver<AudioSnapshot>) -> Self {
        let mut system = PendulumSystem::new();

        // Setup a simple double pendulum
        system.nodes.push(Node {
            name: "Anchor".to_string(),
            pos: macroquad::math::Vec2::new(0.0, 0.0),
            prev_pos: macroquad::math::Vec2::new(0.0, 0.0),
            mass: 1.0,
            fixed: true,
        });

        system.nodes.push(Node {
            name: "Bob1".to_string(),
            pos: macroquad::math::Vec2::new(10.0, 0.0),
            prev_pos: macroquad::math::Vec2::new(10.0, 0.0),
            mass: 1.0,
            fixed: false,
        });

        system.nodes.push(Node {
            name: "Bob2".to_string(),
            pos: macroquad::math::Vec2::new(20.0, 0.0),
            prev_pos: macroquad::math::Vec2::new(20.0, 0.0),
            mass: 1.0,
            fixed: false,
        });

        system.links.push(Link {
            a: 0,
            b: 1,
            length: 10.0,
        });
        system.links.push(Link {
            a: 1,
            b: 2,
            length: 10.0,
        });

        Self {
            system,
            cmd_tx,
            snap_rx,
            latest_snapshot: None,
            running: true,
            ticks: 0,
        }
    }

    fn update(&mut self) {
        // Step the chaotic pendulum system
        self.system.step(0.016);

        // Map pendulum node positions to acoustic grid
        // The pendulum centers around (0,0), lengths are ~20.
        // Map to AUDIO_WIDTH / 2, AUDIO_HEIGHT / 2 with a scale
        let scale = 2.0;
        let cx = AUDIO_WIDTH as f32 / 2.0;
        let cy = AUDIO_HEIGHT as f32 / 2.0;

        for node in &self.system.nodes {
            if !node.fixed {
                let ax = (cx + node.pos.x * scale).clamp(0.0, (AUDIO_WIDTH - 1) as f32) as usize;
                let ay = (cy + node.pos.y * scale).clamp(0.0, (AUDIO_HEIGHT - 1) as f32) as usize;

                // Send a pluck to the audio grid based on node's kinetic energy/velocity
                let vel = node.pos - node.prev_pos;
                let energy = vel.length() * 5.0; // amplify

                if energy > 0.01 {
                    let _ = self.cmd_tx.send(AudioCommand::Pluck {
                        x: ax,
                        y: ay,
                        strength: energy.min(1.0),
                    });
                }
            }
        }

        // Receive the latest snapshot
        while let Ok(snap) = self.snap_rx.try_recv() {
            self.latest_snapshot = Some(snap);
        }

        self.ticks += 1;
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Acoustic Chaos Volatility 🌀🔊 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            let pressure_map = self.latest_snapshot.as_ref().map(|s| &s.pressure);

            // Determine rendering boundaries
            let max_y = inner_area.height as usize;
            let max_x = inner_area.width as usize;
            let r_height = AUDIO_HEIGHT.min(max_y * 2); // Terminal characters are roughly 2:1 height/width
            let r_width = AUDIO_WIDTH.min(max_x);

            // Create a small frame buffer
            let mut display_chars =
                vec![vec![(" ", Color::Reset, Color::Reset); r_width]; r_height / 2];

            // Render audio pressure to background colors
            if let Some(map) = pressure_map {
                #[allow(clippy::needless_range_loop)]
                for y in 0..r_height / 2 {
                    for x in 0..r_width {
                        let sim_x = (x as f32 / r_width as f32 * AUDIO_WIDTH as f32) as usize;
                        let sim_y =
                            (y as f32 / (r_height / 2) as f32 * AUDIO_HEIGHT as f32) as usize;

                        let p = map[sim_y * AUDIO_WIDTH + sim_x];
                        let bg_color = if p > 0.5 {
                            Color::LightMagenta
                        } else if p > 0.1 {
                            Color::Magenta
                        } else if p < -0.5 {
                            Color::LightCyan
                        } else if p < -0.1 {
                            Color::Cyan
                        } else {
                            Color::Reset
                        };
                        display_chars[y][x].2 = bg_color;
                    }
                }
            }

            // Draw pendulum nodes and links
            let scale = 2.0;
            let cx = AUDIO_WIDTH as f32 / 2.0;
            let cy = AUDIO_HEIGHT as f32 / 2.0;

            for link in &self.system.links {
                let p1 = self.system.nodes[link.a].pos;
                let p2 = self.system.nodes[link.b].pos;

                // Super basic line drawing approximation for TUI
                for t in 0..10 {
                    let tf = t as f32 / 10.0;
                    let p = p1 * (1.0 - tf) + p2 * tf;

                    let dx = (cx + p.x * scale) as usize;
                    let dy = (cy + p.y * scale) as usize;

                    let out_x = (dx as f32 / AUDIO_WIDTH as f32 * r_width as f32) as usize;
                    let out_y = (dy as f32 / AUDIO_HEIGHT as f32 * (r_height / 2) as f32) as usize;

                    if out_x < r_width && out_y < r_height / 2 {
                        display_chars[out_y][out_x].0 = "·";
                        display_chars[out_y][out_x].1 = Color::DarkGray;
                    }
                }
            }

            for node in &self.system.nodes {
                let dx = (cx + node.pos.x * scale).clamp(0.0, (AUDIO_WIDTH - 1) as f32) as usize;
                let dy = (cy + node.pos.y * scale).clamp(0.0, (AUDIO_HEIGHT - 1) as f32) as usize;

                let out_x = (dx as f32 / AUDIO_WIDTH as f32 * r_width as f32) as usize;
                let out_y = (dy as f32 / AUDIO_HEIGHT as f32 * (r_height / 2) as f32) as usize;

                if out_x < r_width && out_y < r_height / 2 {
                    display_chars[out_y][out_x].0 = if node.fixed { "+" } else { "O" };
                    display_chars[out_y][out_x].1 = Color::Yellow;
                }
            }

            #[allow(clippy::needless_range_loop)]
            for y in 0..r_height / 2 {
                let mut spans = Vec::new();
                for x in 0..r_width {
                    let (ch, fg, bg) = display_chars[y][x];
                    spans.push(Span::styled(ch, Style::default().fg(fg).bg(bg)));
                }
                lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(lines);
            f.render_widget(paragraph, inner_area);
        })?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(1);

    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let model = AudioModel::new(AUDIO_WIDTH, AUDIO_HEIGHT, cmd_rx, snap_tx, None);

    let host = cpal::default_host();
    let _stream = if let Some(device) = host.default_output_device() {
        if let Ok(config) = device.default_output_config() {
            let model_lock = std::sync::Arc::new(std::sync::Mutex::new(model));
            let model_clone = model_lock.clone();

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        if let Ok(mut model) = model_clone.lock() {
                            model.process(data);
                        }
                    },
                    |err| eprintln!("an error occurred on stream: {}", err),
                    None,
                ),
                _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
            };

            if let Ok(s) = stream {
                if s.play().is_ok() {
                    Some((s, model_lock))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let _fallback_thread = if _stream.is_none() {
        let mut model =
            AudioModel::new(AUDIO_WIDTH, AUDIO_HEIGHT, bounded(1).1, bounded(1).0, None);
        Some(thread::spawn(move || {
            let mut buffer = vec![0.0; 256];
            loop {
                model.process(&mut buffer);
                thread::sleep(Duration::from_millis(5));
            }
        }))
    } else {
        None
    };

    if args.contains(&"--headless".to_string()) {
        let mut app = App::new(cmd_tx, snap_rx);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(cmd_tx, snap_rx);
    let mut last_tick = Instant::now();

    while app.running {
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.update();
            last_tick = Instant::now();
        }

        app.render(&mut terminal)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
