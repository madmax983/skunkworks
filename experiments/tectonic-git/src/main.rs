mod git;
mod simulation;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Rectangle},
        Block, Borders, Paragraph,
    },
};
use std::{
    io,
    time::{Duration, Instant},
};

use git::GitScanner;
use simulation::World;

struct App {
    world: World,
    commits_queue: Vec<git::CommitData>,
    queue_idx: usize,
    is_paused: bool,
    speed: usize,
    should_quit: bool,
    zoom: f64,
}

impl App {
    fn new(commits: Vec<git::CommitData>) -> Self {
        Self {
            world: World::new(200.0), // Virtual width
            commits_queue: commits,
            queue_idx: 0,
            is_paused: false,
            speed: 1, // Commits per tick (or 1 every N ticks?) let's do ticks
            should_quit: false,
            zoom: 1.0,
        }
    }

    fn update(&mut self) {
        self.world.update();

        if !self.is_paused && self.queue_idx < self.commits_queue.len() {
            // Add commit(s)
            // If speed is slow, we might skip frames. If fast, add multiple.
            // Let's just add 1 per update for now, speed controls update freq maybe?
            // Actually let's use speed as commits per frame.
            for _ in 0..self.speed {
                if self.queue_idx < self.commits_queue.len() {
                    let commit = self.commits_queue[self.queue_idx].clone();
                    self.world.add_commit(commit);
                    self.queue_idx += 1;
                }
            }

            // Auto scroll to follow top
            if !self.world.strata.is_empty() {
                let last_y = self.world.strata.last().unwrap().y_pos;
                // Target scroll: keep last added stratum near middle/top
                // Smooth scroll
                let target = last_y - 50.0;
                self.world.scroll_y += (target - self.world.scroll_y) * 0.1;
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load Git History
    // Show loading?
    terminal.draw(|f| {
        let p = Paragraph::new("Scanning Tectonic History... (git log -p)")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(p, f.area());
    })?;

    let commits = match GitScanner::scan() {
        Ok(c) => c,
        Err(e) => {
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
            eprintln!("Error scanning git: {}", e);
            return Err(e);
        }
    };

    let mut app = App::new(commits);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char(' ') => app.is_paused = !app.is_paused,
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.speed = (app.speed + 1).min(10)
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            app.speed = (app.speed.saturating_sub(1)).max(1)
                        }
                        KeyCode::Char('z') => app.zoom *= 1.1,
                        KeyCode::Char('x') => app.zoom /= 1.1,
                        KeyCode::Up => app.world.scroll_y += 5.0,
                        KeyCode::Down => app.world.scroll_y -= 5.0,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(size);

    let canvas_area = main_layout[0];

    // Calculate bounds based on scroll and zoom
    let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
    let view_height = 100.0 / app.zoom;
    let view_width = view_height * aspect * 2.0;

    let center_y = app.world.scroll_y;
    let center_x = app.world.width / 2.0;

    let x_min = center_x - view_width / 2.0;
    let x_max = center_x + view_width / 2.0;
    let y_min = center_y - view_height / 2.0;
    let y_max = center_y + view_height / 2.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Tectonic Git History "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {
            // Draw Strata
            for strata in &app.world.strata {
                // Optimization: cull off-screen
                if strata.y_pos < y_min - 10.0 || strata.y_pos > y_max + 10.0 {
                    continue;
                }

                let color = match strata.color_idx {
                    0 => Color::Rgb(100, 100, 100), // Bedrock
                    1 => Color::Rgb(120, 110, 100),
                    2 => Color::Rgb(140, 120, 100),
                    3 => Color::Rgb(160, 130, 100),
                    4 => Color::Rgb(180, 140, 100),
                    _ => Color::Rgb(200, 150, 100),
                };

                // Shifted X
                let x = strata.offset_x;
                let width = app.world.width;

                ctx.draw(&Rectangle {
                    x: x,
                    y: strata.y_pos,
                    width: width,
                    height: 4.8, // Gap of 0.2
                    color,
                });

                // If stressed, draw "Magma" core or indicator?
                if strata.commit.stress_level > 0.0 {
                    ctx.draw(&Rectangle {
                        x: x + width * 0.45,
                        y: strata.y_pos + 1.0,
                        width: width * 0.1,
                        height: 2.8,
                        color: Color::Red,
                    });
                }
            }

            // Draw Fissures
            for fissure in &app.world.fissures {
                // Optimization check
                // ...

                for i in 0..fissure.points.len().saturating_sub(1) {
                    let p1 = fissure.points[i];
                    let p2 = fissure.points[i + 1];

                    ctx.draw(&CanvasLine {
                        x1: p1.x,
                        y1: p1.y,
                        x2: p2.x,
                        y2: p2.y,
                        color: Color::Yellow,
                    });
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Status Bar
    let current_info = if app.queue_idx > 0 {
        let c = &app.commits_queue[app.queue_idx - 1];
        format!("Commit: {} | Stress: {:.1}", &c.hash[0..7], c.stress_level)
    } else {
        "Waiting to start...".to_string()
    };

    let status = format!(
        "{} | Loaded: {}/{} | Speed: {} | Zoom: {:.1}x | [Space] Pause [+/-] Speed [z/x] Zoom [Arrows] Scroll",
        current_info,
        app.queue_idx,
        app.commits_queue.len(),
        app.speed,
        app.zoom
    );

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        main_layout[1],
    );
}
