use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use platter::Platter;
use quipu::{Quipu, Cord};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct QuipuPlatterApp {
    platter: Platter,
    width: usize,
    height: usize,
    quipu: Quipu,
    time: f64,
}

impl QuipuPlatterApp {
    fn new(width: usize, height: usize) -> Self {
        let mut quipu = Quipu::new();
        quipu.add_cord(Cord::from(42));
        quipu.add_cord(Cord::from(1337));
        quipu.add_cord(Cord::from(255));
        Self {
            platter: Platter::new(width, height),
            width,
            height,
            quipu,
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;

        // The knots deposit heat onto the scalar field
        for (i, cord) in self.quipu.cords.iter().enumerate() {
            let x = (i * 20 + 10) % self.width;
            let val = cord.value();
            let y = (val as usize) % self.height;
            self.platter.accumulate(x, y, 0.5);
        }

        self.platter.decay(0.90);
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
    let mut app = QuipuPlatterApp::new(width, height);

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
                        .title(" Quipu Platter: Data Heatmap "),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let val = app.platter.get(x, y);
                            if val > 0.05 {
                                let color = if val > 0.8 {
                                    Color::Red
                                } else if val > 0.4 {
                                    Color::Yellow
                                } else {
                                    Color::Blue
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

            let stats = Paragraph::new(format!(
                "Cords: {} | [Q] Quit",
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
