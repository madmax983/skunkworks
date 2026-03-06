mod blame;
mod simulation;

use anyhow::Result;
use blame::BlameAnalyzer;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
    Terminal,
};
use simulation::{Agent, World};
use std::{env, io, path::Path, time::{Duration, Instant}};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chron-mycelium <file_path>");
        return Ok(());
    }
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);

    // 1. Analyze Git Blame
    println!("Analyzing {}...", file_path_str);
    let analyzer = BlameAnalyzer::new(".");
    let blame_info = analyzer.analyze(file_path)?;
    if blame_info.is_empty() {
        eprintln!("No blame info found or file is empty.");
        return Ok(());
    }

    // Set up world based on text lines
    let width = 120;
    let height = blame_info.len().max(20).min(100);

    // Instead of cities, we map hot/cold areas from Git Blame
    let mut food_sources = Vec::new();
    for info in &blame_info {
        // High age_score means NEWEST code (Hot)
        // Let's create food sources at lines that are newly modified
        if info.age_score > 0.8 {
            let y = info.line_number.saturating_sub(1) as f64; // 0-indexed
            // Distribute food across the line width
            food_sources.push((60.0, y)); // Centered approx
        }
    }
    if food_sources.is_empty() {
        // Fallback
        food_sources.push((width as f64 / 2.0, height as f64 / 2.0));
    }

    let num_agents = 2000;
    let (mut world, mut agents) = World::with_food_and_agents(width, height, num_agents, &food_sources);

    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut world, &mut agents, width, height, file_path_str);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    world: &mut World,
    agents: &mut Vec<Agent>,
    width: usize,
    height: usize,
    file_name: &str,
) -> Result<()> {
    let tick_rate = Duration::from_millis(32);
    let mut last_tick = Instant::now();

    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);
    let mut agent_points = Vec::with_capacity(agents.len());

    loop {
        // Prepare rendering arrays
        trails_low.clear();
        trails_med.clear();
        trails_high.clear();
        agent_points.clear();

        for y in 0..world.height {
            for x in 0..world.width {
                let val = world.get_trail(x, y);
                if val > 50.0 {
                    trails_high.push((x as f64, y as f64));
                } else if val > 20.0 {
                    trails_med.push((x as f64, y as f64));
                } else if val > 5.0 {
                    trails_low.push((x as f64, y as f64));
                }
            }
        }

        for agent in agents.iter() {
            agent_points.push((agent.x, agent.y));
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let title = format!(" 🧬 Chron-Mycelium: Slime Mold vs Git Blame [{}] ", file_name);

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(title))
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: &trails_low,
                        color: Color::DarkGray,
                    });
                    ctx.draw(&Points {
                        coords: &trails_med,
                        color: Color::Cyan,
                    });
                    ctx.draw(&Points {
                        coords: &trails_high,
                        color: Color::LightCyan,
                    });
                    ctx.draw(&Points {
                        coords: &agent_points,
                        color: Color::Yellow,
                    });
                });

            f.render_widget(canvas, chunks[0]);

            let status = Line::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" to quit. Agents: "),
                Span::styled(format!("{}", agents.len()), Style::default().fg(Color::Cyan)),
            ]);
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update_agents_parallel(agents);
            world.diffuse_and_decay();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
