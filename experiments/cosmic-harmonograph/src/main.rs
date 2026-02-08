mod git;
mod physics;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git::Commit;
use glam::Vec3;
use physics::CosmicString;
use rand::{Rng, SeedableRng, rngs::StdRng};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph, Wrap,
    },
    Terminal,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{self, Stdout},
    time::{Duration, Instant},
};

struct App {
    string_x: CosmicString,
    string_y: CosmicString,
    trace: Vec<(f64, f64)>,
    commits: Vec<Commit>,
    current_commit_idx: usize,
    auto_play: bool,
    last_pluck_time: Instant,
    pluck_interval: Duration,
    status_msg: String,
}

impl App {
    fn new() -> Result<Self> {
        let commits = git::get_recent_commits(100).unwrap_or_else(|_| vec![Commit {
            hash: "0000000000000000000000000000000000000000".to_string(),
            author: "Cosmic Default".to_string(),
            message: "No git repo found. Enjoy the default vibrations.".to_string(),
        }]);

        // String X: Drives X coordinate.
        // String Y: Drives Y coordinate.
        // Both horizontal strings for simulation simplicity, but we map their displacement differently.
        let string_x = CosmicString::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0), 40, 50.0, 0.5);
        let string_y = CosmicString::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0), 40, 60.0, 0.5);

        Ok(Self {
            string_x,
            string_y,
            trace: Vec::with_capacity(10000),
            commits,
            current_commit_idx: 0,
            auto_play: false,
            last_pluck_time: Instant::now(),
            pluck_interval: Duration::from_millis(500),
            status_msg: "Ready. Press [Space] to pluck.".to_string(),
        })
    }

    fn update(&mut self, dt: f32) {
        self.string_x.update(dt);
        self.string_y.update(dt);

        // Map physics to harmonograph trace
        // We take the displacement of the middle node
        let mid_x = self.string_x.nodes.len() / 2;
        let mid_y = self.string_y.nodes.len() / 2;

        let x_val = self.string_x.nodes[mid_x].pos.y; // Vertical displacement of String X
        let y_val = self.string_y.nodes[mid_y].pos.y; // Vertical displacement of String Y

        // Scale up for visibility
        let scale = 20.0;
        self.trace.push((x_val as f64 * scale, y_val as f64 * scale));

        // Limit trace length
        if self.trace.len() > 2000 {
            self.trace.remove(0);
        }

        // Auto-play logic
        if self.auto_play && self.last_pluck_time.elapsed() >= self.pluck_interval {
            self.pluck_next_commit();
            self.last_pluck_time = Instant::now();
        }
    }

    fn pluck_next_commit(&mut self) {
        if self.commits.is_empty() {
            return;
        }

        let commit = &self.commits[self.current_commit_idx];

        // Derive parameters from commit hash
        let mut hasher = DefaultHasher::new();
        commit.hash.hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = StdRng::seed_from_u64(seed);

        // Tune the strings based on commit
        // Author name length influences tension
        let author_len = commit.author.len() as f32;
        self.string_x.tension = 20.0 + (author_len * 2.0).min(100.0);
        self.string_y.tension = 30.0 + (rng.gen::<f32>() * 50.0);

        // Pluck forces
        let force_x = rng.gen_range(-15.0..15.0);
        let force_y = rng.gen_range(-15.0..15.0);

        // Pluck random nodes
        let node_x = rng.gen_range(1..self.string_x.nodes.len() - 1);
        let node_y = rng.gen_range(1..self.string_y.nodes.len() - 1);

        self.string_x.pluck(node_x, Vec3::new(0.0, force_x, 0.0));
        self.string_y.pluck(node_y, Vec3::new(0.0, force_y, 0.0));

        self.status_msg = format!("Plucked: {} ({})", &commit.hash[..7], &commit.author);

        // Advance index
        self.current_commit_idx = (self.current_commit_idx + 1) % self.commits.len();
    }

    fn reset_trace(&mut self) {
        self.trace.clear();
        // Reset strings
         let start = Vec3::new(-10.0, 0.0, 0.0);
         let end = Vec3::new(10.0, 0.0, 0.0);
         self.string_x = CosmicString::new(start, end, 40, 50.0, 0.5);
         self.string_y = CosmicString::new(start, end, 40, 60.0, 0.5);
         self.status_msg = "Reset.".to_string();
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut app = App::new()?;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16);

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => app.pluck_next_commit(),
                        KeyCode::Char('p') => app.auto_play = !app.auto_play,
                        KeyCode::Char('r') => app.reset_trace(),
                        KeyCode::Char('c') => app.trace.clear(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update(0.05); // Fixed time step for physics
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Left: Harmonograph Canvas
    let canvas_block = Block::default()
        .borders(Borders::ALL)
        .title(" Cosmic Harmonograph ");

    let canvas = Canvas::default()
        .block(canvas_block)
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // Draw Trace
            ctx.draw(&Points {
                coords: &app.trace,
                color: Color::Cyan,
            });

            // Draw Current Pen Position
            if let Some(last) = app.trace.last() {
                 ctx.draw(&Points {
                    coords: &[*last],
                    color: Color::Red,
                });
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Right: Info & Controls
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Strings Visualization
            Constraint::Length(3),  // Status
            Constraint::Min(0),     // Commits
        ])
        .split(chunks[1]);

    // Visualize the strings (miniature)
    let strings_canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" String States "))
        .x_bounds([-10.0, 10.0])
        .y_bounds([-5.0, 5.0]) // Vertical displacement is usually small
        .paint(|ctx| {
            // Draw String X (Cyan)
            for i in 0..app.string_x.nodes.len()-1 {
                let p1 = app.string_x.nodes[i].pos;
                let p2 = app.string_x.nodes[i+1].pos;
                ctx.draw(&CanvasLine {
                    x1: p1.x as f64,
                    y1: p1.y as f64 + 2.0, // Offset up
                    x2: p2.x as f64,
                    y2: p2.y as f64 + 2.0,
                    color: Color::Cyan,
                });
            }

            // Draw String Y (Magenta)
            for i in 0..app.string_y.nodes.len()-1 {
                let p1 = app.string_y.nodes[i].pos;
                let p2 = app.string_y.nodes[i+1].pos;
                ctx.draw(&CanvasLine {
                    x1: p1.x as f64,
                    y1: p1.y as f64 - 2.0, // Offset down
                    x2: p2.x as f64,
                    y2: p2.y as f64 - 2.0,
                    color: Color::Magenta,
                });
            }
        });
    f.render_widget(strings_canvas, right_chunks[0]);

    let status = Paragraph::new(app.status_msg.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Status "));
    f.render_widget(status, right_chunks[1]);

    let controls_text = vec![
        "Controls:",
        "[Space] Pluck Next Commit",
        "[P] Toggle Auto-Play",
        "[R] Reset Physics",
        "[C] Clear Trace",
        "[Q] Quit",
        "",
        "Physics:",
        &format!("Tension X: {:.1}", app.string_x.tension),
        &format!("Tension Y: {:.1}", app.string_y.tension),
    ].join("\n");

    let controls = Paragraph::new(controls_text)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" Controls "));
    f.render_widget(controls, right_chunks[2]);
}
