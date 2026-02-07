mod agent;
mod renderer;
mod topology;
mod world;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

use renderer::draw_world;
use world::World;

struct App {
    world: World,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            world: World::new(100.0, 50.0, 20), // 20 agents
            should_quit: false,
        }
    }

    fn on_tick(&mut self) {
        self.world.update();
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Char('r') => app.world = World::new(100.0, 50.0, 20),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new(format!(
        "KLEIN-CHIMERA | Agents: {} | Topology: Klein Bottle (Twist Y)",
        app.world.agents.len()
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"))
    .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Manifold"))
        .x_bounds([-50.0, 50.0])
        .y_bounds([-25.0, 25.0])
        .paint(|ctx| {
            draw_world(ctx, &app.world);
        });
    f.render_widget(canvas, chunks[1]);

    let help = Paragraph::new("Q: Quit | R: Reset | L: Left Chiral (Cyan) | R: Right Chiral (Magenta)")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
