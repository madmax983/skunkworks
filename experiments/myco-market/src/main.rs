use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod simulation;
use simulation::World;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;

struct App {
    world: World,
    tick: u64,
}

impl App {
    fn new() -> Self {
        Self {
            world: World::new(GRID_WIDTH, GRID_HEIGHT),
            tick: 0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Randomly spawn bids at bottom
        if rng.gen_bool(0.3) {
            let x = rng.gen_range(5..(GRID_WIDTH - 5));
            self.world.spawn_bid(x, GRID_HEIGHT - 2);
        }

        // Randomly spawn asks at top
        if rng.gen_bool(0.3) {
            let x = rng.gen_range(5..(GRID_WIDTH - 5));
            self.world.spawn_ask(x, 1);
        }

        self.world.update();
        self.tick += 1;
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + std::error::Error + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
        app.update();
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("🍄 Mycelial Market Simulator (Pheromone-Guided Liquidity) 📈"),
        )
        .x_bounds([0.0, GRID_WIDTH as f64])
        .y_bounds([0.0, GRID_HEIGHT as f64])
        .paint(|ctx| {
            // Draw Pheromone Trails
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    let trail = app.world.get_trail(x, y);
                    if trail > 10.0 {
                        // Invert Y for canvas
                        let cy = GRID_HEIGHT as f64 - 1.0 - (y as f64);

                        let color = if trail > 200.0 {
                            Color::Rgb(255, 255, 0) // Hot trade zone
                        } else if trail > 100.0 {
                            Color::Rgb(0, 255, 0) // Strong path
                        } else {
                            Color::Rgb(0, 50, 0) // Faint path
                        };

                        ctx.draw(&Points {
                            coords: &[(x as f64, cy)],
                            color,
                        });
                    }
                }
            }

            // Draw Agents
            for agent in &app.world.agents {
                let cy = GRID_HEIGHT as f64 - 1.0 - agent.y;
                let color = if agent.is_bid {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.draw(&Points {
                    coords: &[(agent.x, cy)],
                    color,
                });
            }

            // Draw Trades (Flashes)
            for trade in &app.world.trades {
                let cy = GRID_HEIGHT as f64 - 1.0 - (trade.y as f64);
                ctx.draw(&Points {
                    coords: &[(trade.x as f64, cy)],
                    color: Color::White,
                });

                if trade.age > 10 {
                    // Flash radius
                    ctx.draw(&ratatui::widgets::canvas::Circle {
                        x: trade.x as f64,
                        y: cy,
                        radius: 2.0,
                        color: Color::Yellow,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(chunks[1]);

    let agents_count = app.world.agents.len();
    let bids_count = app.world.agents.iter().filter(|a| a.is_bid).count();
    let asks_count = agents_count.saturating_sub(bids_count);

    let market_stats = Paragraph::new(vec![
        ratatui::text::Line::from(vec![
            Span::raw("Active Agents: "),
            Span::styled(format!("{}", agents_count), Style::default().fg(Color::Cyan)),
        ]),
        ratatui::text::Line::from(vec![
            Span::styled(format!("Bids: {} ", bids_count), Style::default().fg(Color::Green)),
            Span::raw("| "),
            Span::styled(format!("Asks: {} ", asks_count), Style::default().fg(Color::Red)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Market Stats"));
    f.render_widget(market_stats, stats_chunks[0]);

    let sim_info = Paragraph::new(vec![
        ratatui::text::Line::from(vec![
            Span::raw("Recent Trades: "),
            Span::styled(
                format!("{}", app.world.trades.len()),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        ratatui::text::Line::from(vec![
            Span::raw("Tick: "),
            Span::styled(
                format!("{}", app.tick),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Simulation Info"));
    f.render_widget(sim_info, stats_chunks[1]);

    let controls = Paragraph::new(vec![
        ratatui::text::Line::from(Span::styled(
            "Controls",
            Style::default().add_modifier(ratatui::style::Modifier::BOLD),
        )),
        ratatui::text::Line::from("Press 'q' or 'Esc' to quit"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls, stats_chunks[2]);
}
