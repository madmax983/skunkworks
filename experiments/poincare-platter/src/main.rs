use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use platter::Platter;
use poincare_disk::{Mobius, Point};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct PoincarePlatterApp {
    platter: Platter,
    width: usize,
    height: usize,
    // Add point sources
    sources: Vec<Point>,
    time: f64,
}

impl PoincarePlatterApp {
    fn new(width: usize, height: usize) -> Self {
        Self {
            platter: Platter::new(width, height),
            width,
            height,
            sources: vec![
                Point::new(0.0, 0.0),
                Point::new(0.5, 0.0),
                Point::new(-0.3, 0.4),
            ],
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;

        // Move sources in hyperbolic space
        let transform1 = Mobius::rotation(0.05);
        let transform2 = Mobius::translation(Point::new(0.01, 0.0)).then(&Mobius::rotation(0.02));

        for (i, source) in self.sources.iter_mut().enumerate() {
            if i % 2 == 0 {
                *source = transform1.apply(*source);
            } else {
                *source = transform2.apply(*source);
            }
        }

        // Project platter to poincare space and accumulate heat
        for y in 0..self.height {
            for x in 0..self.width {
                // Map screen (x, y) to Euclidean (-1, 1)
                let nx = (x as f64 / self.width as f64) * 2.0 - 1.0;
                let ny = (y as f64 / self.height as f64) * 2.0 - 1.0;
                let p = Point::new(nx, ny);

                // Only consider points inside the disk
                if p.norm() < 1.0 {
                    for source in &self.sources {
                        // Hyperbolic distance squared approximation (Möbius transform trick)
                        let mobius = Mobius::inverse_translation(*source);
                        let mapped = mobius.apply(p);
                        let dist_sq = mapped.norm_sqr();

                        if dist_sq < 0.05 {
                            // Radius of heat source
                            // Accumulate heat
                            self.platter.accumulate(x, y, 0.1);
                        }
                    }
                }
            }
        }

        // Decay the field
        self.platter.decay(0.95);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 100;
    let height = 50;
    let mut app = PoincarePlatterApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Poincaré Platter - Hyperbolic Heatmap "),
                )
                .x_bounds([-1.1, 1.1])
                .y_bounds([-1.1, 1.1])
                .paint(|ctx| {
                    // Draw Disk boundary
                    for i in 0..100 {
                        let angle = (i as f64) / 100.0 * 2.0 * std::f64::consts::PI;
                        ctx.print(
                            angle.cos(),
                            angle.sin(),
                            ratatui::text::Span::styled(".", Style::default().fg(Color::DarkGray)),
                        );
                    }

                    // Draw Platter Heatmap
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let val = app.platter.get(x, y);
                            if val > 0.05 {
                                let nx = (x as f64 / app.width as f64) * 2.0 - 1.0;
                                let ny = (y as f64 / app.height as f64) * 2.0 - 1.0;
                                let p = Point::new(nx, ny);

                                if p.norm() < 1.0 {
                                    // Map val (0.0..1.0) to color
                                    let color = if val > 0.8 {
                                        Color::Red
                                    } else if val > 0.4 {
                                        Color::Yellow
                                    } else {
                                        Color::Blue
                                    };
                                    ctx.print(
                                        nx,
                                        ny,
                                        ratatui::text::Span::styled(
                                            "█",
                                            Style::default().fg(color),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Heat Sources: {} | Hyperbolic Decay Active | [Q] Quit",
                app.sources.len(),
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}
