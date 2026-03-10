use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph, Wrap,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use git_associates::model::Commit;
use git_associates::GitModel;
use gray_scott::GrayScott;

struct App {
    commits: Vec<Commit>,
    current_index: usize,
    simulation: GrayScott,
    sim_width: usize,
    sim_height: usize,
    feed: f32,
    kill: f32,
    auto_play: bool,
}

impl App {
    fn new() -> Result<Self> {
        let commits = match GitModel::open(".") {
            Ok(model) => match model.history(100) {
                Ok(c) if !c.is_empty() => c,
                _ => vec![Commit {
                    hash: "0000000000000000000000000000000000000000".to_string(),
                    short_hash: "0000000".to_string(),
                    author: "Nova".to_string(),
                    message: "No git history found. Enjoy this default pattern.".to_string(),
                    timestamp: chrono::Utc::now(),
                    parents: vec![],
                    stats: None,
                    files: vec![],
                }],
            },
            Err(_) => vec![Commit {
                hash: "0000000000000000000000000000000000000000".to_string(),
                short_hash: "0000000".to_string(),
                author: "Nova".to_string(),
                message: "No git repository found. Enjoy this default pattern.".to_string(),
                timestamp: chrono::Utc::now(),
                parents: vec![],
                stats: None,
                files: vec![],
            }],
        };

        let sim_width = 120;
        let sim_height = 60;
        let mut simulation = GrayScott::new(sim_width, sim_height);

        // Seed the center with chemical V
        for y in (sim_height / 2 - 5)..=(sim_height / 2 + 5) {
            for x in (sim_width / 2 - 5)..=(sim_width / 2 + 5) {
                if rand::random::<f32>() < 0.5 {
                    simulation.add_chemical(x, y, 1.0);
                }
            }
        }

        let mut app = Self {
            commits,
            current_index: 0,
            simulation,
            sim_width,
            sim_height,
            feed: 0.055,
            kill: 0.062,
            auto_play: false,
        };
        app.update_selection();
        Ok(app)
    }

    fn update_selection(&mut self) {
        if self.current_index >= self.commits.len() {
            self.current_index = 0;
        }
        let commit = &self.commits[self.current_index];

        // Map hash to feed and kill parameters
        // Typical values for interesting patterns:
        // feed: 0.010 - 0.100
        // kill: 0.045 - 0.070
        // We will use the first few bytes of the hash.
        let get_val = |hash: &str, start: usize, count: usize| -> u32 {
            if start + count > hash.len() {
                return 128; // Fallback to mid-range
            }
            let slice = &hash[start..start + count];
            u32::from_str_radix(slice, 16).unwrap_or(128)
        };

        let hash = &commit.hash;

        // Use bytes 0-1 for feed, 2-3 for kill
        let feed_val = get_val(hash, 0, 2) as f32; // 0-255
        let kill_val = get_val(hash, 2, 2) as f32; // 0-255

        self.feed = 0.010 + (feed_val / 255.0) * 0.090;
        self.kill = 0.045 + (kill_val / 255.0) * 0.025;

        // Add a drop of V to ensure the reaction continues or starts anew
        for y in (self.sim_height / 2 - 5)..=(self.sim_height / 2 + 5) {
            for x in (self.sim_width / 2 - 5)..=(self.sim_width / 2 + 5) {
                if rand::random::<f32>() < 0.5 {
                    self.simulation.add_chemical(x, y, 1.0);
                }
            }
        }
    }

    fn next(&mut self) {
        if self.current_index + 1 < self.commits.len() {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
        self.update_selection();
    }

    fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.commits.len() - 1;
        }
        self.update_selection();
    }

    fn tick(&mut self) {
        // Run several simulation steps per tick for visual speed
        for _ in 0..10 {
            self.simulation.update(self.feed, self.kill, 1.0);
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new()?;
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    anyhow::Error: From<<B as Backend>::Error>,
{
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let auto_play_rate = Duration::from_secs(3);
    let mut last_auto_play = Instant::now();

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
                        KeyCode::Right | KeyCode::Char('l') => app.next(),
                        KeyCode::Left | KeyCode::Char('h') => app.prev(),
                        KeyCode::Char(' ') => app.auto_play = !app.auto_play,
                        KeyCode::Char('r') => {
                            // Reset simulation completely
                            app.simulation = GrayScott::new(app.sim_width, app.sim_height);
                            app.update_selection();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        if app.auto_play && last_auto_play.elapsed() >= auto_play_rate {
            app.next();
            last_auto_play = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Left: Canvas
    let canvas_block = Block::default()
        .borders(Borders::ALL)
        .title(" Git Diffusion: Turing Patterns ");

    let canvas = Canvas::default()
        .block(canvas_block)
        .x_bounds([0.0, app.sim_width as f64])
        .y_bounds([0.0, app.sim_height as f64])
        .paint(|ctx| {
            let mut points_high = Vec::new();
            let mut points_mid = Vec::new();
            let mut points_low = Vec::new();

            let v_grid = app.simulation.v();

            for y in 0..app.sim_height {
                for x in 0..app.sim_width {
                    let idx = app.simulation.get_index(x, y);
                    let val = v_grid[idx];

                    if val > 0.1 {
                        let coord = (x as f64, (app.sim_height - 1 - y) as f64);
                        if val > 0.4 {
                            points_high.push(coord);
                        } else if val > 0.25 {
                            points_mid.push(coord);
                        } else {
                            points_low.push(coord);
                        }
                    }
                }
            }

            ctx.draw(&Points {
                coords: &points_high,
                color: Color::Cyan,
            });
            ctx.draw(&Points {
                coords: &points_mid,
                color: Color::Blue,
            });
            ctx.draw(&Points {
                coords: &points_low,
                color: Color::DarkGray,
            });
        });

    f.render_widget(canvas, chunks[0]);

    // Right: Info
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Title
            Constraint::Min(0),    // Details
            Constraint::Length(3), // Controls
        ])
        .split(chunks[1]);

    let title = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            "Git Diffusion",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw(format!(
            "Commit {}/{}",
            app.current_index + 1,
            app.commits.len()
        ))]),
        Line::from(vec![Span::styled(
            if app.auto_play {
                "Auto-Play: ON"
            } else {
                "Auto-Play: OFF"
            },
            Style::default().fg(if app.auto_play {
                Color::Green
            } else {
                Color::Gray
            }),
        )]),
    ])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, info_chunks[0]);

    let commit = &app.commits[app.current_index];

    let details_text = vec![
        Line::from(Span::styled("Hash:", Style::default().fg(Color::Gray))),
        Line::from(Span::raw(&commit.hash)),
        Line::from(""),
        Line::from(Span::styled("Author:", Style::default().fg(Color::Gray))),
        Line::from(Span::styled(
            &commit.author,
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled("Message:", Style::default().fg(Color::Gray))),
        Line::from(Span::raw(&commit.message)),
        Line::from(""),
        Line::from(Span::styled(
            "Reaction Parameters:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(format!("Feed (f): {:.4}", app.feed)),
        Line::from(format!("Kill (k): {:.4}", app.kill)),
        Line::from(""),
        Line::from(Span::styled(
            "Morphology:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(match (app.feed, app.kill) {
            (f, k) if k < 0.05 => "Stripes / Maze",
            (f, k) if f < 0.03 => "Spots / Coral",
            (f, k) if k > 0.06 => "Holes / Rings",
            _ => "Complex / Transient",
        }),
    ];

    let details = Paragraph::new(details_text)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" Metadata "));
    f.render_widget(details, info_chunks[1]);

    let controls = Paragraph::new("←/→: Nav | Space: Auto | R: Reset Grid | Q: Quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls, info_chunks[2]);
}
