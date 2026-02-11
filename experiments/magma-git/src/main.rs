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
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::{Duration, Instant}};

mod physics;
mod strata;
mod eruption;

use physics::{MagmaParticle, MagmaState};
use strata::StrataGrid;
use eruption::EruptionSource;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
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

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    let size = terminal.size()?;
    let width = size.width as usize;
    let height = size.height as usize;

    let mut grid = StrataGrid::new(width, height);
    let mut particles: Vec<MagmaParticle> = Vec::new();

    // Attempt to load git history from current directory, or fallback to parent if not a repo
    let mut eruption_source = EruptionSource::new(".", 100).or_else(|_| EruptionSource::new("..", 100))?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 FPS

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(size);

            // Render Grid & Particles
            let mut canvas = String::new();
            // Since we can't easily draw individual cells with standard widgets without a Canvas widget or buffer manipulation,
            // we'll construct a Paragraph representing the grid.
            // But modifying the buffer directly is better for performance here.

            // Actually, let's use the buffer directly via a custom widget or just a Paragraph with pre-rendered string.
            // For a particle system, a Canvas widget is best, but Ratatui Canvas uses Braille or Block characters which might not fit our 'char' based logic perfectly.
            // Let's try to construct a buffer of Lines.

            let mut lines: Vec<Line> = Vec::with_capacity(height);
            for y in 0..height {
                let mut spans: Vec<Span> = Vec::with_capacity(width);
                for x in 0..width {
                    // Check particles first (on top)
                    let particle = particles.iter().find(|p| p.x as usize == x && p.y as usize == y);

                    if let Some(p) = particle {
                        spans.push(Span::styled(p.char.to_string(), Style::default().fg(p.color)));
                    } else if let Some(cell) = grid.get(x, y) {
                        spans.push(Span::styled(cell.char.to_string(), Style::default().fg(cell.color)));
                    } else {
                        spans.push(Span::raw(" "));
                    }
                }
                lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
            f.render_widget(paragraph, chunks[0]);

            let status = Paragraph::new(format!("Particles: {} | Eruption: Active", particles.len()))
                .style(Style::default().fg(Color::White).bg(Color::Black));
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update logic
            // 1. Spawn
            if let Some(new_p) = eruption_source.update() {
                particles.extend(new_p);
            }

            // 2. Physics & Collision
            let gravity = 50.0; // Moderate gravity
            let dt = 0.033;

            // Retain only active particles
            particles.retain_mut(|p| {
                p.update(dt, gravity);

                // Boundary check
                if p.y >= height as f32 - 1.0 {
                    p.y = height as f32 - 1.0;
                    p.vx = 0.0;
                    p.vy = 0.0;
                    p.state = MagmaState::Solid;
                    // Solidify into grid
                    grid.solidify(p.x as usize, p.y as usize, p.commit_hash.clone(), p.temperature);
                    return false; // Remove from particle list
                }

                // Grid collision
                let gx = p.x as usize;
                let gy = p.y as usize;
                if grid.is_solid(gx, gy + 1) { // Hit something below
                     p.y = gy as f32;
                     p.vx = 0.0;
                     p.vy = 0.0;
                     p.state = MagmaState::Solid;
                     grid.solidify(gx, gy, p.commit_hash.clone(), p.temperature);
                     return false;
                }

                // Remove if out of bounds (width)
                if p.x < 0.0 || p.x >= width as f32 {
                    return false;
                }

                true
            });

            last_tick = Instant::now();
        }
    }
}
