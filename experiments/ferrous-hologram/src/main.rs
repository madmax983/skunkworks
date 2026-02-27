mod physics;
mod spectral;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use physics::Universe;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

struct App {
    universe: Universe,
}

impl App {
    fn new() -> Self {
        Self {
            universe: Universe::new(100.0, 50.0), // TUI size roughly
        }
    }

    fn update(&mut self) {
        self.universe.update(0.016);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B::Error: std::marker::Send + std::marker::Sync + 'static,
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            // Header
            let title = Paragraph::new(" Ferrous Hologram: Spectral Feedback Loop ")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Left: Spatial (Particles)
            let canvas_spatial = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Spatial Domain "))
                .x_bounds([0.0, app.universe.width])
                .y_bounds([0.0, app.universe.height])
                .marker(ratatui::symbols::Marker::Braille)
                .paint(|ctx| {
                    let mut points = Vec::new();
                    for p in &app.universe.particles {
                        // Flip Y for TUI
                        points.push((p.pos.x, app.universe.height - p.pos.y));
                    }
                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Blue,
                    });
                });
            f.render_widget(canvas_spatial, main_chunks[0]);

            // Right: Spectral (Potential Field)
            // We visualize the "Ghost Potential" which is driving the particles
            let field = &app.universe.spectral_field;
            let canvas_spectral = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Spectral Potential (Ghost Field) "))
                .x_bounds([0.0, field.width as f64])
                .y_bounds([0.0, field.height as f64])
                .marker(ratatui::symbols::Marker::Braille)
                .paint(|ctx| {
                    let mut points = Vec::new();
                    let threshold = 0.5; // Visual threshold

                    // Normalize for display
                    let max_pot = field.potential.iter().fold(0.0f64, |a, &b| a.max(b.abs()));

                    if max_pot > 0.0 {
                         for (i, &val) in field.potential.iter().enumerate() {
                            let norm = val.abs() / max_pot;
                            if norm > threshold {
                                let x = (i % field.width) as f64;
                                let y = (i / field.width) as f64;
                                points.push((x, field.height as f64 - y));
                            }
                        }
                    }

                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Red,
                    });
                });
            f.render_widget(canvas_spectral, main_chunks[1]);

            // Footer
            let footer = Paragraph::new(" [Q] Quit | Particles generate Field -> FFT -> Filter -> IFFT -> Forces on Particles ")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
        }

        app.update();
    }
}
