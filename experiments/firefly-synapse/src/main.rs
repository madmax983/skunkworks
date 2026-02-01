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
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};

mod firefly;
use firefly::Swarm;

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
    swarm: Swarm,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        Self {
            swarm: Swarm::new(200, width, height),
            running: true,
        }
    }

    fn on_tick(&mut self) {
        self.swarm.update();
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // 200x100 virtual units
    let world_width = 200.0;
    let world_height = 100.0;

    let mut app = App::new(world_width, world_height);

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
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => app.swarm.randomize_phases(),
                        KeyCode::Char('k') => app.swarm.coupling += 0.005,
                        KeyCode::Char('j') => app.swarm.coupling = (app.swarm.coupling - 0.005).max(0.0),
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

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let sync_index = app.swarm.synchronization_index();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Firefly Synapse (Kuramoto Model)"),
        )
        .x_bounds([0.0, app.swarm.width])
        .y_bounds([0.0, app.swarm.height])
        .paint(|ctx| {
             for fly in &app.swarm.fireflies {
                 let (char, color) = if fly.flash_timer > 0 {
                     ("★", Color::Yellow)
                 } else {
                     // Gradient based on phase
                     match (fly.phase * 4.0) as u8 {
                         0 => ("•", Color::DarkGray),
                         1 => ("•", Color::Gray),
                         2 => ("•", Color::White),
                         _ => ("•", Color::Cyan), // Almost ready
                     }
                 };

                 ctx.print(
                     fly.x,
                     fly.y,
                     Span::styled(char, Style::default().fg(color)),
                 );
             }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Sync (r): {:.3} | Coupling (K): {:.3} | 'r': Reset | 'k'/'j': +/- K | 'q': Quit",
        sync_index, app.swarm.coupling
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Cyan));
    f.render_widget(p, chunks[1]);
}
