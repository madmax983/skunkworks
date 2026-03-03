use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};

use tui_shared::Tui;

mod boid;
mod git;
mod world;

use git::get_commit_history;
use world::World;

fn main() -> Result<()> {
    // 1. Gather git info
    let mut commits = get_commit_history()?;
    if commits.is_empty() {
        println!("No git commits found, or not in a git repository.");
        return Ok(());
    }

    // Only use up to 150 commits so it doesn't get too slow
    commits.truncate(150);

    let mut tui = Tui::init()?;

    let res = run_app(&mut tui.terminal, commits);

    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    world: World,
    running: bool,
    selected_boid: Option<usize>,
}

impl App {
    fn new(width: f64, height: f64, commits: Vec<git::Commit>) -> Self {
        Self {
            world: World::new(width, height, commits),
            running: true,
            selected_boid: None,
        }
    }

    fn on_tick(&mut self) {
        self.world.update();

        // Randomly change selected boid every once in a while to showcase commits
        if rand::random::<f64>() < 0.05 && !self.world.boids.is_empty() {
            self.selected_boid = Some(rand::random::<usize>() % self.world.boids.len());
        }
    }
}

#[allow(clippy::collapsible_if)]
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    commits: Vec<git::Commit>,
) -> Result<()> {
    // Canvas dimensions (virtual units)
    let world_width = 200.0;
    let world_height = 100.0;

    let mut app = App::new(world_width, world_height, commits);

    let tick_rate = Duration::from_millis(33); // ~30 FPS
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
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            return Ok(());
        }
    }
}

#[allow(clippy::collapsible_if)]
fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(f.area());

    let sync_index = app.world.synchronization_index();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Flock 🕊️: Git History as Boids"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            for (i, boid) in app.world.boids.iter().enumerate() {
                // Determine visual style based on flash state
                let (char_str, color) = if boid.flash_timer > 0 {
                    ("★".to_string(), Color::White) // Bright flash
                } else {
                    // Dimmed based on phase (pulsing effect) or just base color
                    let base_char = boid.dna.char_representation.to_string();
                    let color = if Some(i) == app.selected_boid {
                        Color::Green
                    } else {
                        boid.dna.color
                    };
                    (base_char, color)
                };

                ctx.print(
                    boid.position.x,
                    boid.position.y,
                    Span::styled(char_str, Style::default().fg(color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let mut selected_info = String::from("No commit selected");
    if let Some(idx) = app.selected_boid {
        if let Some(b) = app.world.boids.get(idx) {
            let msg = b.commit.message.chars().take(50).collect::<String>();
            let hash_str = if b.commit.hash.len() >= 7 {
                &b.commit.hash[0..7]
            } else {
                &b.commit.hash
            };
            selected_info = format!(
                "Featured Commit: {} by {} - {}",
                hash_str, b.commit.author, msg
            );
        }
    }

    let status = format!(
        "Sync Index: {:.3} | Population: {} | 'q': Quit\n{}",
        sync_index,
        app.world.boids.len(),
        selected_info
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Cyan));
    f.render_widget(p, chunks[1]);
}
