use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};

pub mod boid;
pub mod phonology;
pub mod world;

use world::World;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    world: World,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        Self {
            world: World::new(width, height),
            running: true,
        }
    }

    fn on_tick(&mut self) {
        self.world.update();
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let size = terminal.size()?;
    // Map terminal size to world size (approximate)
    let world_width = size.width as f64 * 1.5;
    let world_height = size.height as f64 * 3.0; // Y axis is usually denser in chars

    let mut app = App::new(world_width, world_height);

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                // Check if key is 'q' to quit
                let is_q = key.code == KeyCode::Char('q');
                if key.kind == KeyEventKind::Press && is_q {
                    app.running = false;
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

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    // Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Phonetic Flock"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            // Draw Boids
            for boid in &app.world.boids {
                // Draw Y-inverted for canvas if needed, but here we just map directly
                // Note: Ratatui canvas has Y-up (0,0 bottom-left).
                // Boid sim usually assumes Y-down or just Cartesian.
                // We'll just draw it.
                ctx.print(
                    boid.position.x,
                    boid.position.y,
                    Span::styled(boid.dna.word.clone(), Style::default().fg(boid.dna.color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status bar
    let boid_count = app.world.boids.len();

    let status = format!(
        "Words: {} | Green: Growth, Red: Decay, Yellow: Shift | Press 'q' to quit",
        boid_count
    );

    let p = Paragraph::new(status).style(Style::default().fg(Color::White).bg(Color::Blue));
    f.render_widget(p, chunks[1]);
}
