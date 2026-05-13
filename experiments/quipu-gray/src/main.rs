use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use quipu::{Cord, Knot, Quipu};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct QuipuGrayApp {
    gs: GrayScott,
    quipu: Quipu,
    width: usize,
    height: usize,
}

impl QuipuGrayApp {
    fn new(width: usize, height: usize) -> Self {
        let mut quipu = Quipu::new();
        quipu.add_cord(Cord::from(1048576));
        quipu.add_cord(Cord::from(8192));
        quipu.add_cord(Cord::from(256));
        quipu.add_cord(Cord::from(42));
        quipu.add_cord(Cord::from(65535));
        quipu.add_cord(Cord::from(2048));
        quipu.add_cord(Cord::from(314159));

        Self {
            gs: GrayScott::new(width, height),
            quipu,
            width,
            height,
        }
    }

    fn tick(&mut self) {
        // Drop chemicals based on Quipu knots
        let cord_spacing = self.width / (self.quipu.cords.len() + 1);

        for (i, cord) in self.quipu.cords.iter().enumerate() {
            let cx = cord_spacing * (i + 1);

            // Draw knots vertically
            // Clusters are reverse-ordered (highest place value first) in printing,
            // let's distribute them down the y-axis.
            if !cord.clusters.is_empty() {
                let cluster_spacing = self.height / (cord.clusters.len() + 2);
                for (j, cluster) in cord.clusters.iter().enumerate().rev() {
                    let cy = cluster_spacing * (cord.clusters.len() - j);

                    if !cluster.is_empty() {
                        let knot_spacing = 3;
                        let start_x = cx.saturating_sub((cluster.len() * knot_spacing) / 2);

                        for (k, knot) in cluster.iter().enumerate() {
                            let kx = start_x + k * knot_spacing;

                            // Amount of chemical based on knot value
                            let amount = match knot {
                                Knot::Simple => 0.2,
                                Knot::Long(v) => 0.1 * (*v as f32),
                                Knot::FigureEight => 0.3,
                            };

                            // Inject V (kill chemical) to spark Turing patterns
                            self.gs.add_chemical(kx, cy, amount);
                        }
                    }
                }
            }
        }

        // Update Gray-Scott reaction-diffusion
        // Run multiple steps per tick for visible growth
        for _ in 0..5 {
            self.gs.update(0.055, 0.062, 1.0);
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|arg| arg == "--headless");

    let mut tui = if headless { None } else { Some(Tui::init()?) };
    let width = 120;
    let height = 60;
    let mut app = QuipuGrayApp::new(width, height);

    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();
    let mut frame_count = 0;

    loop {
        if headless {
            app.tick();
            frame_count += 1;
            if frame_count > 50 {
                println!("🧬 quipu-gray running in headless mode for 50 ticks.");
                break;
            }
            continue;
        }

        if let Some(tui) = &mut tui {
            tui.terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(f.area());

                let canvas = Canvas::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" 🧬 Splice: quipu × gray-scott | Knotted Morphogenesis "),
                    )
                    .x_bounds([0.0, app.width as f64])
                    .y_bounds([0.0, app.height as f64])
                    .paint(|ctx| {
                        // Draw GrayScott V chemical concentration
                        let v_field = app.gs.v();
                        for y in 0..app.height {
                            for x in 0..app.width {
                                let idx = app.gs.get_index(x, y);
                                let v = v_field[idx];
                                if v > 0.1 {
                                    let color = if v > 0.4 {
                                        Color::Cyan
                                    } else if v > 0.2 {
                                        Color::Blue
                                    } else {
                                        Color::DarkGray
                                    };
                                    ctx.print(
                                        x as f64,
                                        // Invert Y for canvas drawing
                                        (app.height - 1 - y) as f64,
                                        ratatui::text::Span::styled(
                                            "█",
                                            Style::default().fg(color),
                                        ),
                                    );
                                }
                            }
                        }

                        // Overlay Quipu Knots
                        let cord_spacing = app.width / (app.quipu.cords.len() + 1);
                        for (i, cord) in app.quipu.cords.iter().enumerate() {
                            let cx = cord_spacing * (i + 1);

                            // Draw main cord line
                            for y in 0..app.height {
                                ctx.print(
                                    cx as f64,
                                    (app.height - 1 - y) as f64,
                                    ratatui::text::Span::styled(
                                        "|",
                                        Style::default().fg(Color::DarkGray),
                                    ),
                                );
                            }

                            if !cord.clusters.is_empty() {
                                let cluster_spacing = app.height / (cord.clusters.len() + 2);
                                for (j, cluster) in cord.clusters.iter().enumerate().rev() {
                                    let cy = cluster_spacing * (cord.clusters.len() - j);

                                    if !cluster.is_empty() {
                                        let knot_spacing = 3;
                                        let start_x =
                                            cx.saturating_sub((cluster.len() * knot_spacing) / 2);

                                        for (k, knot) in cluster.iter().enumerate() {
                                            let kx = start_x + k * knot_spacing;

                                            let symbol = match knot {
                                                Knot::Simple => "●",
                                                Knot::Long(_) => "≡",
                                                Knot::FigureEight => "∞",
                                            };

                                            ctx.print(
                                                kx as f64,
                                                (app.height - 1 - cy) as f64,
                                                ratatui::text::Span::styled(
                                                    symbol,
                                                    Style::default()
                                                        .fg(Color::White)
                                                        .bg(Color::Red),
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    });

                f.render_widget(canvas, chunks[0]);

                let stats = Paragraph::new(format!(
                    "Cords: {} | Injecting 'V' Catalyst | [Q] Quit",
                    app.quipu.cords.len(),
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
                            KeyCode::Char('q') | KeyCode::Esc => break,
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

    if let Some(mut tui) = tui {
        tui.exit()?;
    }

    Ok(())
}
