//! Acoustic Soft-Body Morphogenesis
//!
//! This experiment is a hybrid crossing `origami` and `resonance-audio`. It demonstrates the
//! sonification of physical soft-body paper deformations. The physical 3D vertices of a continuous
//! procedural Miura-ori soft-body mesh (`origami`) are mapped directly to a 2D acoustic
//! simulation grid (`resonance-audio`).
//!
//! As the soft-body mesh breathes, folds, and crumples, the structural tension (Z-depth or
//! motion) acts as a physical exciter (pluck/tone), injecting audio waves into the acoustic grid.

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
use glam::Vec3;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::PbdSystem;
use resonance_audio::{AudioCommand, AudioModel, AudioSnapshot};

const TICK_RATE: Duration = Duration::from_millis(50);
const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 30;

struct App {
    physics: PbdSystem,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: Receiver<AudioSnapshot>,
    latest_snapshot: Option<AudioSnapshot>,
    running: bool,
    ticks: u64,
}

impl App {
    fn new(cmd_tx: Sender<AudioCommand>, snap_rx: Receiver<AudioSnapshot>) -> Self {
        let cols = 15;
        let rows = 15;

        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 1.2,
            orientation: Orientation::Horizontal,
        };

        let points = generate_miura_grid(params, (cols, rows), 0.5);

        let mut physics = PbdSystem::new();
        let mut p_indices = Vec::with_capacity(points.len());

        // Center the mesh in the grid
        let cx = GRID_WIDTH as f32 / 2.0;
        let cy = GRID_HEIGHT as f32 / 2.0;

        for pos in &points {
            let offset_pos = Vec3::new(pos.x + cx, pos.y + cy, pos.z);
            p_indices.push(physics.add_particle(offset_pos, 1.0));
        }

        // Add structural distance constraints
        let w = cols + 1;
        for y in 0..=rows {
            for x in 0..=cols {
                let i = y * w + x;

                if x < cols {
                    let right = y * w + (x + 1);
                    let d = physics.particles[p_indices[i]]
                        .pos
                        .distance(physics.particles[p_indices[right]].pos);
                    let _ = physics.add_distance_constraint(p_indices[i], p_indices[right], d);
                }

                if y < rows {
                    let down = (y + 1) * w + x;
                    let d = physics.particles[p_indices[i]]
                        .pos
                        .distance(physics.particles[p_indices[down]].pos);
                    let _ = physics.add_distance_constraint(p_indices[i], p_indices[down], d);
                }
            }
        }

        Self {
            physics,
            cmd_tx,
            snap_rx,
            latest_snapshot: None,
            running: true,
            ticks: 0,
        }
    }

    fn update(&mut self) {
        // Slowly compress and expand the mesh based on a sine wave to simulate breathing/folding
        let fold_progress = (self.ticks as f32 * 0.1).sin() * 0.5 + 0.5;

        // Apply forces to expand/contract
        let center = Vec3::new(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0, 0.0);
        for p in self.physics.particles.iter_mut() {
            let dir = (p.pos - center).normalize_or_zero();
            if fold_progress > 0.5 {
                p.vel -= dir * 0.1; // contract
            } else {
                p.vel += dir * 0.1; // expand
            }
        }

        self.physics.step(0.05, 5);

        // Analyze the Z-depth (structural tension/folds) of the paper
        // and map it to acoustic plucks in the resonance grid.
        for p in &self.physics.particles {
            let z_depth = p.pos.z;
            let speed = p.vel.length();

            // Map 3D pos down to 2D grid
            let grid_x = p.pos.x.clamp(0.0, GRID_WIDTH as f32 - 1.0) as usize;
            let grid_y = p.pos.y.clamp(0.0, GRID_HEIGHT as f32 - 1.0) as usize;

            // Pluck the wave tank based on tension (z_depth variation) or velocity
            if z_depth.abs() > 0.5 || speed > 0.5 {
                let _ = self.cmd_tx.send(AudioCommand::Pluck {
                    x: grid_x,
                    y: grid_y,
                    strength: (z_depth.abs() * 0.1 + speed * 0.05).clamp(0.1, 1.0),
                });
            }
        }

        while let Ok(snap) = self.snap_rx.try_recv() {
            self.latest_snapshot = Some(snap);
        }

        self.ticks += 1;
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Acoustic Soft-Body Morphogenesis 📄🔊 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            let pressure_map = self.latest_snapshot.as_ref().map(|s| &s.pressure);

            for y in 0..GRID_HEIGHT.min(inner_area.height as usize) {
                let mut spans = Vec::new();
                for x in 0..GRID_WIDTH.min(inner_area.width as usize) {
                    let mut p = 0.0;
                    if let Some(map) = pressure_map {
                        if y < GRID_HEIGHT && x < GRID_WIDTH {
                            p = map[y * GRID_WIDTH + x];
                        }
                    }

                    // Acoustic pressure background
                    let bg_color = if p > 0.5 {
                        Color::LightBlue
                    } else if p > 0.1 {
                        Color::Blue
                    } else if p < -0.5 {
                        Color::Red
                    } else if p < -0.1 {
                        Color::LightRed
                    } else {
                        Color::Reset
                    };

                    // Check if a paper vertex is here
                    let mut ch = " ";
                    let mut fg_color = Color::Reset;

                    for p_obj in &self.physics.particles {
                        let px = p_obj.pos.x.round() as usize;
                        let py = p_obj.pos.y.round() as usize;
                        if px == x && py == y {
                            if p_obj.pos.z > 0.2 {
                                ch = "^"; // Mountain fold
                                fg_color = Color::Yellow;
                            } else if p_obj.pos.z < -0.2 {
                                ch = "v"; // Valley fold
                                fg_color = Color::DarkGray;
                            } else {
                                ch = "."; // Flat
                                fg_color = Color::White;
                            }
                            break;
                        }
                    }

                    spans.push(Span::styled(ch, Style::default().fg(fg_color).bg(bg_color)));
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

    // Audio stream initialization
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let host = cpal::default_host();
    let _stream = if let Some(device) = host.default_output_device() {
        if let Ok(config) = device.default_output_config() {
            let model = AudioModel::new(
                GRID_WIDTH,
                GRID_HEIGHT,
                cmd_rx.clone(),
                snap_tx.clone(),
                None,
            );
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
        let mut fallback_model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);
        Some(thread::spawn(move || {
            let mut buffer = vec![0.0; 256];
            loop {
                fallback_model.process(&mut buffer);
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
