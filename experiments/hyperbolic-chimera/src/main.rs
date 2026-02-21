mod agent;
mod world;

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
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Terminal, Frame,
};
use std::time::{Duration, Instant};
use world::World;

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut world = World::new(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &world))?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        // Update
        if last_tick.elapsed() >= Duration::from_millis(50) {
            world.update();
            last_tick = Instant::now();
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, world: &World) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.size());

    let canvas_area = chunks[0];
    let status_area = chunks[1];

    // Status Bar
    let status = Paragraph::new(Line::from(vec![
        Span::styled("Hyperbolic Chimera", Style::default().fg(Color::Cyan)),
        Span::raw(format!(" | Agents: {} | Score: {}", world.agents.len(), world.score)),
        Span::raw(" | Press 'q' to quit"),
    ]));
    f.render_widget(status, status_area);

    // Main Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Poincaré Disk"))
        .x_bounds([-1.1, 1.1])
        .y_bounds([-1.1, 1.1])
        .paint(|ctx| {
            // Draw Disk Boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                color: Color::White,
            });

            // Draw Grid Lines (Hyperbolic Geodesics) - Optional visual flair
            // Draw some concentric circles (Euclidean circles centered at 0 correspond to hyperbolic circles)
            ctx.draw(&Circle { x: 0.0, y: 0.0, radius: 0.5, color: Color::DarkGray });
            ctx.draw(&Circle { x: 0.0, y: 0.0, radius: 0.8, color: Color::DarkGray });
            ctx.draw(&CanvasLine { x1: -1.0, y1: 0.0, x2: 1.0, y2: 0.0, color: Color::DarkGray });
            ctx.draw(&CanvasLine { x1: 0.0, y1: -1.0, x2: 0.0, y2: 1.0, color: Color::DarkGray });

            // Draw Food
            for food in &world.food {
                ctx.draw(&Circle {
                    x: food.re,
                    y: food.im,
                    radius: 0.02,
                    color: Color::Green,
                });
            }

            // Draw Agents
            for agent in &world.agents {
                ctx.draw(&Circle {
                    x: agent.pos.re,
                    y: agent.pos.im,
                    radius: 0.03, // Visual size
                    color: agent.color,
                });
            }
        });

    f.render_widget(canvas, canvas_area);
}
