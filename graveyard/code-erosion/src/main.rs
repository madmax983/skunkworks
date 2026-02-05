mod git;
mod terrain;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git::GitScanner;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};
use terrain::Terrain;

struct App {
    terrain: Terrain,
    commits: Vec<git::Commit>,
    commit_idx: usize,
    speed: usize,
    paused: bool,
    should_quit: bool,
}

impl App {
    fn new() -> Result<Self> {
        let commits = GitScanner::load_history()?;
        Ok(Self {
            terrain: Terrain::new(120, 60), // Match canvas resolution roughly
            commits,
            commit_idx: 0,
            speed: 5,
            paused: false,
            should_quit: false,
        })
    }

    fn update(&mut self) {
        if self.paused || self.commit_idx >= self.commits.len() {
            // Even if paused or done, we can run global erosion or water decay
            self.terrain.decay_water();
            return;
        }

        let end_idx = (self.commit_idx + self.speed).min(self.commits.len());

        for i in self.commit_idx..end_idx {
            let commit = &self.commits[i];
            for file in &commit.files {
                let path_str = file.to_string_lossy();
                let (x, y) =
                    GitScanner::map_path(&path_str, self.terrain.width, self.terrain.height);

                // Uplift (Magma/Growth)
                self.terrain.uplift(x, y, 5.0);

                // Erosion (Weather)
                // Rain falls where changes happen
                self.terrain.erode_droplet(x as f64, y as f64);
            }
        }
        self.commit_idx = end_idx;

        // Global Erosion (Time passes)
        // Occasional random rain?
        // self.terrain.erode(1);

        self.terrain.decay_water();
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init App
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            disable_raw_mode()?;
            execute!(std::io::stdout(), LeaveAlternateScreen)?;
            eprintln!("Error initializing: {}", e);
            return Err(e);
        }
    };

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
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('+') => app.speed = (app.speed * 2).min(1000),
                        KeyCode::Char('-') => app.speed = (app.speed / 2).max(1),
                        KeyCode::Char('r') => {
                            app.terrain = Terrain::new(120, 60);
                            app.commit_idx = 0;
                        }
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

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let w = app.terrain.width as f64;
    let h = app.terrain.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Code Erosion "),
        )
        .x_bounds([0.0, w])
        .y_bounds([0.0, h])
        .paint(|ctx| {
            // Draw terrain cells
            // We iterate over the terrain grid and draw rectangles (pixels)
            // Canvas resolution in Ratatui is effectively 2x4 per char if using Block markers?
            // Actually Canvas uses Braille or Block.
            // Let's rely on Canvas::paint to draw Rectangles which are quantized.

            // Optimization: Don't draw 0 height?
            for y in 0..app.terrain.height {
                for x in 0..app.terrain.width {
                    let height = app.terrain.get_height(x, y);
                    let water = app.terrain.water_map[y * app.terrain.width + x];

                    if height <= 0.1 && water <= 0.1 {
                        continue;
                    }

                    // Color mapping
                    // Water: Blue
                    // Low: Green
                    // Mid: Grey
                    // High: White
                    // Sediment: Yellow?

                    let color = if water > 0.5 {
                        Color::Blue
                    } else if height < 5.0 {
                        Color::Rgb(34, 139, 34) // Forest Green
                    } else if height < 20.0 {
                        Color::DarkGray
                    } else if height < 50.0 {
                        Color::Gray
                    } else {
                        Color::White
                    };

                    // Draw a 1x1 rect at x, y
                    // Since y in terrain is 0..H, and canvas is 0..H
                    // We need to invert Y if terrain 0 is top.
                    // Usually graphics 0 is bottom left.
                    // Terrain 0,0 is usually top left in array.
                    // So let's flip Y.

                    ctx.draw(&Rectangle {
                        x: x as f64,
                        y: (app.terrain.height - 1 - y) as f64,
                        width: 1.0,
                        height: 1.0,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status
    let progress = if !app.commits.is_empty() {
        (app.commit_idx as f64 / app.commits.len() as f64) * 100.0
    } else {
        0.0
    };

    let (current_hash, current_date) = if app.commit_idx > 0 && app.commit_idx <= app.commits.len()
    {
        let c = &app.commits[app.commit_idx - 1];
        let date = chrono::DateTime::from_timestamp(c.timestamp, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d");
        (&c.hash[..], date.to_string())
    } else {
        ("...", "Start".to_string())
    };

    let status = format!(
        "Commit: {} | Date: {} | Progress: {:.1}% | Speed: {} | Paused: {} | [Space] Pause [+/-] Speed [r] Reset [q] Quit",
        &current_hash[0..7.min(current_hash.len())],
        current_date,
        progress,
        app.speed,
        app.paused
    );

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
