use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::{
    io,
    time::{Duration, Instant},
};

mod physics;
use physics::Universe;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run
    let res = run(&mut terminal);

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut universe = Universe::new(80.0, 40.0);
    let mut paused = false;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Orbital Decay ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

            let inner = block.inner(chunks[0]);
            f.render_widget(block, chunks[0]);

            // Update universe dimensions to match terminal
            universe.width = inner.width as f64;
            universe.height = inner.height as f64;

            f.render_widget(UniverseWidget { universe: &universe }, inner);

            let status = format!(
                " [Q]uit | [R]eset | [Space] {} | Absorbed: {} ",
                if paused { "Resume" } else { "Pause" },
                universe.absorbed_count
            );
            let status_bar = Paragraph::new(status)
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(status_bar, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('r') => universe.reset(),
                        KeyCode::Char(' ') => paused = !paused,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                universe.tick();
            }
            last_tick = Instant::now();
        }
    }
}

struct UniverseWidget<'a> {
    universe: &'a Universe,
}

impl Widget for UniverseWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render trails first (dimmer)
        for particle in &self.universe.particles {
            for (i, &(tx, ty)) in particle.trail.iter().enumerate() {
                let x = tx.round() as u16;
                let y = ty.round() as u16;
                if x < area.width && y < area.height {
                    let intensity = (i as f32 / particle.trail.len() as f32 * 155.0) as u8 + 40;
                    if let Some(cell) = buf.cell_mut((area.left() + x, area.top() + y)) {
                        cell.set_char('·')
                            .set_fg(Color::Rgb(intensity / 2, intensity / 2, intensity));
                    }
                }
            }
        }

        // Render particles
        for particle in &self.universe.particles {
            let x = particle.x.round() as u16;
            let y = particle.y.round() as u16;
            if x < area.width && y < area.height {
                // Color based on speed
                let speed = (particle.vx * particle.vx + particle.vy * particle.vy).sqrt();
                let color = if speed > 0.8 {
                    Color::Red
                } else if speed > 0.4 {
                    Color::Yellow
                } else {
                    Color::White
                };
                if let Some(cell) = buf.cell_mut((area.left() + x, area.top() + y)) {
                    cell.set_char(particle.char).set_fg(color);
                }
            }
        }

        // Render gravity wells (brightest, on top)
        for well in &self.universe.wells {
            let x = well.x.round() as u16;
            let y = well.y.round() as u16;
            if x < area.width && y < area.height {
                // Draw well with "glow"
                let glow_radius = (well.mass / 50.0) as i16;
                for dy in -glow_radius..=glow_radius {
                    for dx in -glow_radius..=glow_radius {
                        let dist = ((dx * dx + dy * dy) as f64).sqrt();
                        if dist <= glow_radius as f64 && dist > 0.5 {
                            let gx = (x as i16 + dx) as u16;
                            let gy = (y as i16 + dy) as u16;
                            if gx < area.width && gy < area.height {
                                let intensity = ((1.0 - dist / glow_radius as f64) * 100.0) as u8;
                                if let Some(cell) = buf.cell_mut((area.left() + gx, area.top() + gy)) {
                                    cell.set_char('░')
                                        .set_fg(Color::Rgb(intensity + 50, intensity / 2, intensity + 80));
                                }
                            }
                        }
                    }
                }
                // Well center
                if let Some(cell) = buf.cell_mut((area.left() + x, area.top() + y)) {
                    cell.set_char(well.char)
                        .set_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));
                }
            }
        }
    }
}
