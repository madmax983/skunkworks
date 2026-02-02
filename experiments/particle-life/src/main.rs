mod world;
use world::Universe;

use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    Frame,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, canvas::{Canvas, Points}},
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
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

struct App {
    universe: Universe,
    running: bool,
    colors: Vec<Color>,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        let n_species = 6;
        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Cyan,
            Color::Magenta,
        ];

        Self {
            universe: Universe::new(width, height, 400, n_species),
            running: true,
            colors,
        }
    }

    fn on_tick(&mut self) {
        self.universe.update(0.02);
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // 200x150 resolution for the simulation world
    let width = 200.0;
    let height = 150.0;

    let mut app = App::new(width, height);
    let tick_rate = Duration::from_millis(30);
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
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => {
                            // Reset
                            app.universe = Universe::new(width, height, 400, app.colors.len());
                        }
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

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Particle Life"))
        .x_bounds([0.0, app.universe.width])
        .y_bounds([0.0, app.universe.height])
        .marker(ratatui::symbols::Marker::Braille)
        .paint(|ctx| {
             for i in 0..app.colors.len() {
                let species_points: Vec<(f64, f64)> = app.universe.particles.iter()
                    .filter(|p| p.species == i)
                    .map(|p| (p.x, p.y))
                    .collect();

                ctx.draw(&Points {
                    coords: &species_points,
                    color: app.colors[i],
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    let info = format!("Particles: {} | R: Reset | Q: Quit", app.universe.particles.len());
    let status = Paragraph::new(info).style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(status, chunks[1]);
}
