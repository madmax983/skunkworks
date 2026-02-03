mod git;
mod terrain;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use git::GitHistory;
use terrain::Terrain;

struct App {
    history: GitHistory,
    terrain: Terrain,
    current_commit_idx: usize,
    is_paused: bool,
    speed: usize, // commits per tick
    should_quit: bool,
    viewport_width: usize,
    viewport_height: usize,
}

impl App {
    fn new(history: GitHistory, width: usize, height: usize) -> Self {
        Self {
            history,
            terrain: Terrain::new(width, height),
            current_commit_idx: 0,
            is_paused: false,
            speed: 1,
            should_quit: false,
            viewport_width: width,
            viewport_height: height,
        }
    }

    fn update(&mut self) {
        if self.is_paused {
            return;
        }

        self.terrain.decay_water();

        if self.current_commit_idx >= self.history.commits.len() {
            // Loop or stop? Let's just continue erosion without rain, or loop.
            // Let's loop for endless fun.
            self.current_commit_idx = 0;
            // Maybe clear terrain? No, let it build up.
            return;
        }

        // Process N commits
        for _ in 0..self.speed {
            if self.current_commit_idx >= self.history.commits.len() {
                break;
            }
            let commit = &self.history.commits[self.current_commit_idx];

            for file in &commit.files {
                let (x, y) = self.map_file_to_coord(file);
                // "Rain" intensity could be based on file extension or something?
                // For now, just 1.0
                self.terrain.add_rain(x as f64, y as f64);
            }

            self.current_commit_idx += 1;
        }

        // Always erode
        // Maybe do multiple erosion steps per frame if needed?
        // But `add_rain` already simulates the particle.
        // So we don't need explicit `erode` call unless we want continuous background erosion.
        // The terrain logic currently does "instant" erosion in `add_rain`.
    }

    fn map_file_to_coord(&self, path: &str) -> (usize, usize) {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let hash = hasher.finish();

        // Simple mapping
        let idx = (hash as usize) % (self.terrain.width * self.terrain.height);
        (idx % self.terrain.width, idx / self.terrain.width)
    }
}

#[allow(clippy::collapsible_if)]
fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load data
    let history = match GitHistory::load() {
        Ok(h) => h,
        Err(e) => {
            // Cleanup before panic
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
            eprintln!("Failed to load git history: {}", e);
            return Err(e);
        }
    };

    // Create App
    // We'll determine size dynamically or fix it.
    // Let's fix it to a reasonable size and scale later if needed.
    let mut app = App::new(history, 120, 40);

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char(' ') => app.is_paused = !app.is_paused,
                        KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Char('k') => {
                            app.speed = (app.speed * 2).min(100);
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Char('j') => {
                            app.speed = (app.speed / 2).max(1);
                        }
                        KeyCode::Char('r') => {
                            app.terrain = Terrain::new(app.viewport_width, app.viewport_height); // Reset terrain
                            app.current_commit_idx = 0;
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
    let size = f.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(size);

    // Render Terrain
    // We create a custom buffer rendering or just text
    // Since we have a `Terrain` with `width` and `height`, let's try to fit it.

    // Determine viewport offset? For now just 0,0
    let render_width = size.width as usize;
    let render_height = (size.height.saturating_sub(3)) as usize;

    // We simply clamp to min(terrain, screen)
    let w = render_width.min(app.terrain.width);
    let h = render_height.min(app.terrain.height);

    let mut canvas_str = String::with_capacity(w * h + h); // +h for newlines

    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    for y in 0..h {
        for x in 0..w {
            let height = app.terrain.get_height(x, y);
            let water = app.terrain.water_map[y * app.terrain.width + x];

            if water > 0.1 {
                // Water char
                canvas_str.push('~');
            } else {
                // Map height to char
                // Height range? Initial 5-10. Erosion can go lower, deposition higher.
                // Let's assume range 0-20
                let idx = (height.max(0.0) as usize).min(chars.len() - 1);
                canvas_str.push(chars[idx]);
            }
        }
        canvas_str.push('\n');
    }

    let terrain_widget = Paragraph::new(canvas_str).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Geologic Git Terrain"),
    );

    f.render_widget(terrain_widget, main_layout[0]);

    // Status Bar
    let current_commit = if app.current_commit_idx < app.history.commits.len() {
        &app.history.commits[app.current_commit_idx]
    } else {
        &app.history.commits[0] // fallback
    };

    let date = chrono::DateTime::from_timestamp(current_commit.timestamp, 0)
        .unwrap_or_default()
        .format("%Y-%m-%d %H:%M:%S");

    let status_text = format!(
        "Commit: {} | Date: {} | Speed: {}x | Pause: {} | Files: {}\nControls: [Space] Pause [+/-] Speed [r] Reset [q] Quit",
        &current_commit.hash[0..7],
        date,
        app.speed,
        app.is_paused,
        current_commit.files.len()
    );

    let status_bar = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));

    f.render_widget(status_bar, main_layout[1]);
}
