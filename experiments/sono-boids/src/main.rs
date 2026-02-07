use anyhow::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::time::{Duration, Instant};
use tui_shared::math::Vec2;
use tui_shared::Tui;

const WIDTH: usize = 80;
const HEIGHT: usize = 40;

struct Boid {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
}

impl Boid {
    fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize(),
            acc: Vec2::new(0.0, 0.0),
        }
    }

    fn update(&mut self, width: f64, height: f64, wave_grid: &[f32], grid_w: usize, grid_h: usize) {
        // Wrap around
        if self.pos.x < 0.0 {
            self.pos.x += width;
        }
        if self.pos.x >= width {
            self.pos.x -= width;
        }
        if self.pos.y < 0.0 {
            self.pos.y += height;
        }
        if self.pos.y >= height {
            self.pos.y -= height;
        }

        self.vel += self.acc;
        if self.vel.magnitude() > 1.0 {
            self.vel = self.vel.normalize();
        }
        self.pos += self.vel;
        self.acc = Vec2::new(0.0, 0.0);

        // Wave interaction: steer away from high waves
        let gx = (self.pos.x / width * grid_w as f64) as usize;
        let gy = (self.pos.y / height * grid_h as f64) as usize;

        if gx < grid_w && gy < grid_h {
            let idx = gy * grid_w + gx;
            if idx < wave_grid.len() {
                let wave_val = wave_grid[idx].abs();
                if wave_val > 0.2 {
                    // Turn randomly if in high wave
                    let mut rng = rand::thread_rng();
                    self.acc += Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
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
    snap_rx: Receiver<Vec<f32>>,
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
            self.wave_grid = snap;
        }

        // Update boids
        // We clone boids for flocking check to avoid borrow checker
        let boids_state: Vec<(Vec2, Vec2)> = self.boids.iter().map(|b| (b.pos, b.vel)).collect();

        let mut forces = Vec::with_capacity(self.boids.len());
        for i in 0..self.boids.len() {
            let mut align = Vec2::new(0.0, 0.0);
            let mut coh = Vec2::new(0.0, 0.0);
            let mut sep = Vec2::new(0.0, 0.0);
            let mut count = 0;

            for j in 0..self.boids.len() {
                if i == j {
                    continue;
                }
                let pos_j = boids_state[j].0;
                let vel_j = boids_state[j].1;
                let d = boids_state[i].0.distance(pos_j);

                if d > 0.0 && d < 10.0 {
                    align += vel_j;
                    coh += pos_j;
                    sep += (boids_state[i].0 - pos_j).normalize() / d;
                    count += 1;
                }
            }

            let mut force = Vec2::new(0.0, 0.0);
            if count > 0 {
                align = align / count as f64;
                align = align.normalize() * 0.05;

                coh = coh / count as f64;
                coh = (coh - boids_state[i].0).normalize() * 0.01;

                sep = sep.normalize() * 0.05;
                force = align + coh + sep;
            }
            forces.push(force);
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.acc += forces[i];
            boid.update(WIDTH as f64, HEIGHT as f64, &self.wave_grid, WIDTH, HEIGHT);

            // Randomly emit ping
            if rand::thread_rng().gen_bool(0.01) {
                let gx = (boid.pos.x) as usize;
                let gy = (boid.pos.y) as usize;
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
                    boid.pos.x,
                    HEIGHT as f64 - boid.pos.y,
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
