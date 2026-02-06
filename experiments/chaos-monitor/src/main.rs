use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    collections::VecDeque,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use chaos_monitor::{LorenzSystem, SystemMonitor};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut system = LorenzSystem::default();
    let mut monitor = SystemMonitor::new();

    // History of points
    let mut history: VecDeque<(f64, f64, f64)> = VecDeque::with_capacity(10000);
    // Add initial point
    history.push_back((system.x, system.y, system.z));

    let mut angle: f64 = 0.0;
    let rotation_speed = 0.01;

    // Time tracking
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 fps

    // Metrics cache
    let mut last_monitor_update = Instant::now();

    // Force initial update
    monitor.refresh();
    let (mut cpu_load, mut mem_load, mut swap_load) = monitor.get_load_metrics();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.area());

            // Calculate projection
            // We want to reuse the allocated vector if possible, but for now strict allocation is fine for 10k points
            let projected_points: Vec<(f64, f64)> = history.iter().map(|(x, y, z)| {
                // Rotate around Y axis (which is vertical in screen space usually, but here z is up in Lorenz)
                // In Lorenz: z is up. x, y are horizontal.
                // Let's rotate around Z axis? No, we want to spin the attractor.
                // Rotate around Z (vertical axis in Lorenz terms)
                let x_rot = x * angle.cos() - y * angle.sin();

                // Map z to screen y?
                // Screen X = x_rot
                // Screen Y = z
                (x_rot, *z)
            }).collect();

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Lorenz Attractor (System Load)"))
                .x_bounds([-30.0, 30.0])
                .y_bounds([0.0, 60.0]) // Lorenz z goes 0 to 50ish
                .paint(|ctx| {
                    for i in 1..projected_points.len() {
                        let (p1x, p1y) = projected_points[i - 1];
                        let (p2x, p2y) = projected_points[i];

                        // Color gradient based on index (time)
                        let color = if i > projected_points.len().saturating_sub(100) {
                            Color::White
                        } else if i > projected_points.len().saturating_sub(1000) {
                            Color::Cyan
                        } else if i > projected_points.len().saturating_sub(3000) {
                            Color::Blue
                        } else {
                            Color::DarkGray
                        };

                        ctx.draw(&CanvasLine::new(p1x, p1y, p2x, p2y, color));
                    }
                });
            f.render_widget(canvas, chunks[0]);

            // Info Panel
            let info_text = vec![
                Line::from(Span::styled("Chaos Monitor", Style::default().fg(Color::Green))),
                Line::from(""),
                Line::from(format!("CPU Load: {:.1}%", cpu_load * 100.0)),
                Line::from(format!(" -> Rho: {:.2}", system.rho)),
                Line::from(""),
                Line::from(format!("Mem Load: {:.1}%", mem_load * 100.0)),
                Line::from(format!(" -> Sigma: {:.2}", system.sigma)),
                Line::from(""),
                Line::from(format!("Swap Load: {:.1}%", swap_load * 100.0)),
                Line::from(format!(" -> Beta: {:.2}", system.beta)),
                Line::from(""),
                Line::from(format!("Points: {}", history.len())),
                Line::from(format!("Coords: ({:.1}, {:.1}, {:.1})", system.x, system.y, system.z)),
                Line::from(""),
                Line::from("Controls:"),
                Line::from(" 'q': Quit"),
            ];

            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL).title("Metrics"));
            f.render_widget(info, chunks[1]);
        })?;

        // Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        // Updates
        let now = Instant::now();
        if now.duration_since(last_tick) >= tick_rate {
            last_tick = now;
            angle += rotation_speed;

            // Update Monitor
            if now.duration_since(last_monitor_update) > Duration::from_millis(200) {
                monitor.refresh();
                let (c, m, s) = monitor.get_load_metrics();
                cpu_load = c;
                mem_load = m;
                swap_load = s;

                // Map to Lorenz Params
                // Base: sigma=10, rho=28, beta=8/3 (2.66)
                system.rho = 28.0 + (cpu_load * 30.0); // 28 to 58
                system.sigma = 10.0 + (mem_load * 10.0); // 10 to 20
                // system.beta = (8.0/3.0) + (swap_load * 2.0);

                last_monitor_update = now;
            }

            // Update Physics
            let dt = 0.005;
            for _ in 0..10 {
                system.update_rk4(dt);
                history.push_back((system.x, system.y, system.z));
                if history.len() > 5000 { // Limit points
                    history.pop_front();
                }
            }
        }
    }
}
