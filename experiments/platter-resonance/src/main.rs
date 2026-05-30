use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use platter::Platter;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use resonance_audio::PhysicsGrid;
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct PlatterResonanceApp {
    platter: Platter,
    grid: PhysicsGrid,
    width: usize,
    height: usize,
    time: f32,
}

impl PlatterResonanceApp {
    fn new(width: usize, height: usize) -> Self {
        Self {
            platter: Platter::new(width, height),
            grid: PhysicsGrid::new(width, height),
            width,
            height,
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.1;

        // 1. Heat injection pattern
        let cx1 = (self.width as f32 / 2.0 + (self.time * 0.5).cos() * (self.width as f32 / 4.0))
            as usize;
        let cy1 = (self.height as f32 / 2.0 + (self.time * 0.7).sin() * (self.height as f32 / 4.0))
            as usize;
        let cx2 = (self.width as f32 / 2.0 + (self.time * 0.3).sin() * (self.width as f32 / 3.0))
            as usize;
        let cy2 = (self.height as f32 / 2.0 + (self.time * 0.4).cos() * (self.height as f32 / 3.0))
            as usize;

        self.platter.accumulate(cx1, cy1, 1.0);
        self.platter.accumulate(cx2, cy2, 1.0);

        // Diffuse and decay heat
        self.platter.decay(0.95);

        // 2. Thermodynamic Acoustic Excitation
        for y in 0..self.height {
            for x in 0..self.width {
                let heat = self.platter.get(x, y);
                if heat > 0.8 {
                    self.grid.pluck(x, y, ((heat - 0.8) * 0.5) as f32);
                }
            }
        }

        // 3. Step acoustic physics
        self.grid.step();
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let title = " 🧬 Splice: platter × resonance-audio (Thermodynamic Acoustic Excitation) ";
        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let heat = self.platter.get(x, y);
                        let wave = self.grid.get(x, y);

                        if heat > 0.2 {
                            let color = if heat > 0.8 {
                                Color::Red
                            } else if heat > 0.5 {
                                Color::LightRed
                            } else {
                                Color::Yellow
                            };
                            let render_x = (x as f64 / self.width as f64) * 100.0;
                            let render_y = (y as f64 / self.height as f64) * 100.0;
                            ctx.print(
                                render_x,
                                render_y,
                                ratatui::text::Span::styled("█", Style::default().fg(color)),
                            );
                        } else if wave.abs() > 0.05 {
                            let color = if wave > 0.5 {
                                Color::Cyan
                            } else if wave > 0.1 {
                                Color::LightBlue
                            } else {
                                Color::Blue
                            };
                            let render_x = (x as f64 / self.width as f64) * 100.0;
                            let render_y = (y as f64 / self.height as f64) * 100.0;
                            ctx.print(
                                render_x,
                                render_y,
                                ratatui::text::Span::styled("≈", Style::default().fg(color)),
                            );
                        }
                    }
                }
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        frame.render_widget(canvas, chunks[0]);

        let info =
            Paragraph::new("Injecting heat droplets... Critical heat boils into acoustic ripples.")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("🧬 platter-resonance running in headless mode for 10 ticks.");
        let mut app = PlatterResonanceApp::new(100, 100);
        for _ in 0..10 {
            app.tick();
        }
        println!("Finished headless run.");
        return Ok(());
    }

    let mut tui = Tui::init()?;

    let mut app = PlatterResonanceApp::new(100, 50);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
                {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
