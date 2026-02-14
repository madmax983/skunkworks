use anyhow::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use flocking::{compute_force, FlockingParams, PhysicsState};
use locus::Vec2;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use resonance_audio::audio::{AudioCommand, AudioModel, AudioSnapshot};
use std::time::{Duration, Instant};
use tui_shared::Tui;

const WIDTH: usize = 80;
const HEIGHT: usize = 40;

struct Boid {
    physics: PhysicsState,
}

impl Boid {
    fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut physics = PhysicsState::new(x, y);
        physics.velocity = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
        Self { physics }
    }

    fn update(&mut self, width: f64, height: f64, wave_grid: &[f32], grid_w: usize, grid_h: usize) {
        self.physics.update(1.0); // max_speed = 1.0

        // Wrap around
        if self.physics.position.x < 0.0 {
            self.physics.position.x += width;
        }
        if self.physics.position.x >= width {
            self.physics.position.x -= width;
        }
        if self.physics.position.y < 0.0 {
            self.physics.position.y += height;
        }
        if self.physics.position.y >= height {
            self.physics.position.y -= height;
        }

        // Wave interaction: steer away from high waves
        let gx = (self.physics.position.x / width * grid_w as f64) as usize;
        let gy = (self.physics.position.y / height * grid_h as f64) as usize;

        if gx < grid_w && gy < grid_h {
            let idx = gy * grid_w + gx;
            if idx < wave_grid.len() {
                let wave_val = wave_grid[idx].abs();
                if wave_val > 0.2 {
                    // Turn randomly if in high wave
                    let mut rng = rand::thread_rng();
                    self.physics.apply_force(Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5)));
                }
            }
        }
    }
}

struct App {
    boids: Vec<Boid>,
    wave_model: AudioModel,
    wave_grid: Vec<f32>,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: Receiver<AudioSnapshot>,
    running: bool,
}

impl App {
    fn new() -> Result<Self> {
        let (cmd_tx, cmd_rx) = bounded(100);
        let (snap_tx, snap_rx) = bounded(2);

        let wave_model = AudioModel::new(WIDTH, HEIGHT, cmd_rx, snap_tx);

        let mut boids = Vec::new();
        for _ in 0..50 {
            boids.push(Boid::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0));
        }

        Ok(Self {
            boids,
            wave_model,
            wave_grid: vec![0.0; WIDTH * HEIGHT],
            cmd_tx,
            snap_rx,
            running: true,
        })
    }

    fn update(&mut self) {
        // Run wave simulation
        let mut dummy_buffer = vec![0.0; 1024];
        self.wave_model.process(&mut dummy_buffer);

        // Check for snapshots
        while let Ok(snap) = self.snap_rx.try_recv() {
            self.wave_grid = snap.pressure;
        }

        // Update boids
        let physics_states: Vec<PhysicsState> = self.boids.iter().map(|b| b.physics).collect();
        let mut forces = Vec::with_capacity(self.boids.len());

        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        for (i, _) in self.boids.iter().enumerate() {
            let force = compute_force(&physics_states[i], &physics_states, i, &params);
            forces.push(force);
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.physics.apply_force(forces[i]);
            boid.update(WIDTH as f64, HEIGHT as f64, &self.wave_grid, WIDTH, HEIGHT);

            // Randomly emit ping
            if rand::thread_rng().gen_bool(0.01) {
                let gx = (boid.physics.position.x) as usize;
                let gy = (boid.physics.position.y) as usize;
                if gx < WIDTH && gy < HEIGHT {
                    let _ = self.cmd_tx.send(AudioCommand::Pluck {
                        x: gx,
                        y: gy,
                        strength: 0.5,
                    });
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => app.running = false,
                            KeyCode::Char(' ') => {
                                // Big splash
                                let _ = app.cmd_tx.send(AudioCommand::Pluck {
                                    x: WIDTH / 2,
                                    y: HEIGHT / 2,
                                    strength: 2.0,
                                });
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Sono-Boids "))
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .paint(|ctx| {
            // Draw Waves
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let idx = y * WIDTH + x;
                    if idx < app.wave_grid.len() {
                        let val = app.wave_grid[idx];
                        if val > 0.1 {
                            ctx.draw(&Points {
                                coords: &[(x as f64, (HEIGHT - y) as f64)],
                                color: Color::Red,
                            });
                        } else if val < -0.1 {
                            ctx.draw(&Points {
                                coords: &[(x as f64, (HEIGHT - y) as f64)],
                                color: Color::Blue,
                            });
                        }
                    }
                }
            }

            // Draw Boids
            for boid in &app.boids {
                ctx.print(
                    boid.physics.position.x,
                    HEIGHT as f64 - boid.physics.position.y,
                    Span::styled("*", Style::default().fg(Color::Yellow)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = Paragraph::new(vec![Line::from(vec![
        Span::raw("Space: Splash | Q: Quit | "),
        Span::styled("Red: Peak", Style::default().fg(Color::Red)),
        Span::raw(" | "),
        Span::styled("Blue: Trough", Style::default().fg(Color::Blue)),
        Span::raw(" | "),
        Span::styled("Yellow: Boid", Style::default().fg(Color::Yellow)),
    ])])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
}
