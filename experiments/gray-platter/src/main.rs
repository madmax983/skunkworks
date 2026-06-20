use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

use gray_scott::GrayScott;
use platter::Platter;
use tui_shared::Tui;

struct GrayPlatterApp {
    gs: GrayScott,
    platter: Platter,
    width: usize,
    height: usize,
}

impl GrayPlatterApp {
    fn new(width: usize, height: usize) -> Self {
        let mut gs = GrayScott::new(width, height);

        // Seed the reaction-diffusion with some V chemical spores
        gs.add_chemical(width / 2, height / 2, 1.0);
        gs.add_chemical(width / 2 + 10, height / 2 - 5, 1.0);
        gs.add_chemical(width / 2 - 10, height / 2 + 5, 1.0);

        Self {
            gs,
            platter: Platter::new(width, height),
            width,
            height,
        }
    }

    fn tick(&mut self) {
        // Run gray-scott steps
        for _ in 0..10 {
            // "Spots" parameters: f = 0.03, k = 0.062
            self.gs.update(0.03, 0.062, 1.0);
        }

        // Project V chemical directly into platter heat field
        let v_slice = self.gs.v();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(idx) = self.gs.get_index(x, y) {
                    let v = v_slice[idx];
                    if v > 0.1 {
                        // Accumulate heat based on V chemical concentration
                        self.platter.accumulate(x, y, (v * 0.5) as f64);
                    }
                }
            }
        }

        // Decay the heat field
        self.platter.decay(0.95);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    let width = 80;
    let height = 80;
    let mut app = GrayPlatterApp::new(width, height);

    if headless {
        app.tick();
        println!("Headless check passed");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui, &mut app);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut GrayPlatterApp) -> Result<()> {
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
                        .title(" Thermodynamic Chemical Diffusion "),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let heat = app.platter.get(x, y);
                            if heat > 0.05 {
                                // Map heat to color
                                let color = if heat > 0.8 {
                                    Color::White
                                } else if heat > 0.5 {
                                    Color::LightRed
                                } else if heat > 0.2 {
                                    Color::Red
                                } else {
                                    Color::DarkGray
                                };

                                ctx.print(
                                    x as f64,
                                    y as f64,
                                    ratatui::text::Span::styled("█", Style::default().fg(color)),
                                );
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats =
                Paragraph::new("Gray-Platter: Reaction-Diffusion + Thermal Decay | [Q] Quit")
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
