mod boid;
mod git;
mod strata;
mod world;

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
use world::World;

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
            world: World::new(200.0, 100.0), // Virtual dimensions
            commits_queue: commits,
            queue_idx: 0,
            is_paused: false,
            speed: 1,
            should_quit: false,
            zoom: 1.0,
        }
    }

    fn update(&mut self) {
        // Update World Physics
        self.world.update();

        if !self.is_paused && self.queue_idx < self.commits_queue.len() {
            // Add commit(s)
            for _ in 0..self.speed {
                if self.queue_idx < self.commits_queue.len() {
                    let commit = self.commits_queue[self.queue_idx].clone();
                    self.world.strata.add_commit(commit);
                    self.queue_idx += 1;
                }
            }

            // Auto scroll logic (keep last added stratum in view)
            if !self.world.strata.strata.is_empty() {
                let last_y = self.world.strata.strata.last().unwrap().y_pos;
                // Target scroll: keep slightly below center so we see history rising?
                // Or falling? In tectonic-git, new strata are added at bottom (higher Y).
                // So we want to increase scroll_y to follow them.
                let target = last_y - 50.0;
                self.world.strata.scroll_y += (target - self.world.strata.scroll_y) * 0.1;
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
    let tick_rate = Duration::from_millis(33); // ~30 FPS
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
                        KeyCode::Up => app.world.strata.scroll_y += 5.0,
                        KeyCode::Down => app.world.strata.scroll_y -= 5.0,
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
    let view_width = view_height * aspect * 2.0; // Double width for better aspect ratio in terminal

    let center_y = app.world.strata.scroll_y;
    let center_x = app.world.width / 2.0;

    let x_min = center_x - view_width / 2.0;
    let x_max = center_x + view_width / 2.0;
    let y_min = center_y - view_height / 2.0;
    let y_max = center_y + view_height / 2.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Tectonic Flock: Boids Debugging History "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {
            // Draw Strata
            for strata in &app.world.strata.strata {
                if strata.y_pos < y_min - 10.0 || strata.y_pos > y_max + 10.0 {
                    continue;
                }

                let color = match strata.color_idx {
                    0 => Color::Rgb(50, 50, 50),
                    1 => Color::Rgb(60, 55, 50),
                    2 => Color::Rgb(70, 60, 50),
                    3 => Color::Rgb(80, 65, 50),
                    4 => Color::Rgb(90, 70, 50),
                    _ => Color::Rgb(100, 75, 50),
                };

                let x = strata.offset_x; // Wait, offset_x is shift, not absolute X?
                // In tectonic-git: x = strata.offset_x. But width was app.world.width.
                // Here offset_x seems to be 0 + shift.
                // So draw full width but shifted?
                // Let's draw centered width + shift.

                let width = app.world.width;

                ctx.draw(&Rectangle {
                    x: x, // Assuming offset_x is absolute left?
                    // No, StrataManager::add_commit: offset_x: prev_offset + shift.
                    // Start at 0?
                    // Tectonic-git drew at x: strata.offset_x.
                    // If offset_x drifts, the whole strata drifts. That's fine.
                    y: strata.y_pos,
                    width: width,
                    height: 4.8,
                    color,
                });
            }

            // Draw Fissures
            for fissure in &app.world.strata.fissures {
                for i in 0..fissure.points.len().saturating_sub(1) {
                    let p1 = fissure.points[i];
                    let p2 = fissure.points[i + 1];

                    if p1.y < y_min || p1.y > y_max { continue; }

                    ctx.draw(&CanvasLine {
                        x1: p1.x,
                        y1: p1.y,
                        x2: p2.x,
                        y2: p2.y,
                        color: Color::Yellow,
                    });
                }
            }

            // Draw Boids
            // Boids live in screen space (0..width, 0..height)?
            // Wait, in World::new we init boids 0..width, 0..height.
            // But we render strata at y_pos (which grows indefinitely).
            // So boids are flying at y=0..100, but strata are at y=0..5000?
            // This is a coordinate mismatch!

            // FIX:
            // Boids should fly in the *viewport* (camera space).
            // So when we draw boids, we add `app.world.strata.scroll_y` to their Y position?
            // Or subtract?
            // If boids are "UI elements" (debuggers on screen), they stay in view.
            // If boids are "in the world", they should have absolute coordinates.

            // Let's make boids live in the VIEWPORT.
            // So boid.y is relative to the camera.
            // Strata move relative to the camera.

            // In World::update, we update boids. They move in 0..width, 0..height.
            // So we just draw them at boid.y + y_min?
            // y_min is the bottom of the visible area in World Space.
            // So if boid is at y=10 (bottom of screen), and we are looking at y=500..600.
            // We draw at 510.

            for boid in &app.world.boids {
                // Determine visual style
                let (char_str, color) = if boid.flash_timer > 0 {
                     ("★", Color::White)
                } else if boid.nearby_fissure_intensity > 0.5 {
                     ("⚠", Color::Red) // Alert!
                } else {
                     match boid.dna.char_representation {
                         '✦' => ("✦", boid.dna.color),
                         _ => ("•", boid.dna.color),
                     }
                };

                // Map viewport-relative boid pos to world space for rendering
                let world_x = boid.position.x; // X matches?
                // app.world.width is 200. strata width is 200.
                // Rendering X bounds: x_min..x_max centered on center_x (100).
                // So if zoom=1.0, view_width ~= 200. x_bounds ~= 0..200.
                // So boid.x is fine.

                // Y: boid.y is 0..100.
                // Viewport Y is y_min..y_max.
                // So draw at y_min + boid.y?
                let world_y = y_min + boid.position.y;

                ctx.print(
                    world_x,
                    world_y,
                    Span::styled(char_str, Style::default().fg(color)),
                );
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
        "{} | Boids: {} | Speed: {} | Zoom: {:.1}x | [Space] Pause [+/-] Speed [z/x] Zoom [Arrows] Scroll",
        current_info,
        app.world.boids.len(),
        app.speed,
        app.zoom
    );

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        main_layout[1],
    );
}
