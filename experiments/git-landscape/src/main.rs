mod audio;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git_associates::{GitModel, model::Commit};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::VecDeque,
    env,
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};
use tui_shared::Tui;

use crate::audio::AudioEngine;

const WIDTH: usize = 100; // Number of X buckets (File Hash buckets)
const DEPTH: usize = 60; // Number of Z rows (Visible Commits)

struct App {
    commits: Vec<Commit>,   // All loaded commits (Oldest -> Newest)
    terrain: VecDeque<Vec<u8>>, // Current visible terrain (Rows of heights)
    commit_idx: usize,          // Index of next commit to enter the horizon
    speed: f64,                 // Flight speed (commits per second)
    paused: bool,
    accumulation: f64,
    audio: AudioEngine,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let model = GitModel::open(path)?;
        let mut commits = model.history(1000)?; // Load up to 1000 commits
        commits.reverse(); // Oldest -> Newest

        // Initialize flat terrain
        let mut terrain = VecDeque::new();
        for _ in 0..DEPTH {
            terrain.push_back(vec![0; WIDTH]);
        }

        let audio = AudioEngine::new();

        Ok(Self {
            commits,
            terrain,
            commit_idx: 0,
            speed: 5.0, // 5 commits per second
            paused: false,
            accumulation: 0.0,
            audio,
        })
    }

    fn update(&mut self, dt: f64) {
        if self.paused {
            return;
        }

        self.accumulation += dt * self.speed;

        while self.accumulation >= 1.0 {
            self.accumulation -= 1.0;
            self.step();
        }
    }

    fn step(&mut self) {
        if self.commit_idx >= self.commits.len() {
            // Loop or Stop? Let's Loop for infinite flight
            self.commit_idx = 0;
        }

        let commit = &self.commits[self.commit_idx];

        // Generate Row from Commit
        let row = self.generate_row(commit);

        // Move Terrain
        self.terrain.pop_front(); // Remove closest
        self.terrain.push_back(row); // Add new at horizon

        // Play Sound
        // We play the commit that is entering the horizon?
        // Or the one that is passing us (popping from front)?
        // Let's play the one entering to anticipate.
        self.audio.play_commit(commit);

        self.commit_idx += 1;
    }

    fn generate_row(&self, commit: &Commit) -> Vec<u8> {
        let mut row = vec![0u8; WIDTH];

        for change in &commit.files {
            // Hash file path to get bucket
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            change.path.hash(&mut hasher);
            let hash = hasher.finish();
            let bucket = (hash as usize) % WIDTH;

            // Height based on changes
            let magnitude = change.insertions + change.deletions;
            let height = if magnitude == 0 {
                0
            } else {
                (magnitude as f64).log2() as u8 + 1
            };

            // Accumulate height in bucket (capped at 40)
            row[bucket] = row[bucket].saturating_add(height).min(40);
        }

        row
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    let mut tui = Tui::init()?;
    let mut app = App::new(path)?;

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Up => app.speed = (app.speed + 1.0).min(20.0),
                        KeyCode::Down => app.speed = (app.speed - 1.0).max(1.0),
                        _ => {}
                    }
                }
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        last_tick = now;

        app.update(dt);
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Git Landscape: Flight over History "),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, 80.0]) // Z(0..60) * 0.5 + Height(0..40) ~ 70
        .paint(|ctx| {
            // Draw lines
            // Iterate through rows (z)
            // Z=0 is CLOSE (Bottom of screen). Z=DEPTH is HORIZON (Top of screen).
            // But usually landscape renders Z=0 at Bottom Y.
            // Ratatui Y=0 is Bottom.
            // So Z=0 should be at Y=0. Z=DEPTH should be at Y=Max.

            for (z, row) in app.terrain.iter().enumerate() {
                // z goes 0..DEPTH.
                // 0 is FRONT (Oldest in buffer, Closest to camera).
                // DEPTH is BACK (Newest in buffer, Horizon).

                let z_factor = z as f64;

                for (x, &h) in row.iter().enumerate() {
                    let h = h as f64;
                    if h == 0.0 {
                        continue;
                    }

                    let x_pos = x as f64;
                    // Perspective: Distant things are higher up on screen.
                    let y_base = z_factor * 1.0;
                    let y_top = y_base + h;

                    let color = if h > 10.0 {
                        Color::Red
                    } else if h > 5.0 {
                        Color::Yellow
                    } else {
                        Color::Green
                    };

                    // Draw vertical pillar
                    ctx.draw(&Line {
                        x1: x_pos,
                        y1: y_base,
                        x2: x_pos,
                        y2: y_top,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let current_hash = if app.commit_idx > 0 && app.commit_idx <= app.commits.len() {
        app.commits[app.commit_idx - 1].hash.clone()
    } else {
        "Starting...".to_string()
    };

    let msg = if app.commit_idx > 0 && app.commit_idx <= app.commits.len() {
        app.commits[app.commit_idx - 1].message.clone()
    } else {
        "".to_string()
    };

    let status = format!(
        "Commit: {} | Msg: {:.50} | Speed: {:.1} cps | Space: Pause | Up/Down: Speed",
        &current_hash[..7.min(current_hash.len())],
        msg,
        app.speed
    );
    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
