use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use platter::Platter;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub fn run() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal);
    drop(tui);
    res
}

struct App {
    dish: GrayScott,
    platter: Platter,
    running: bool,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let mut dish = GrayScott::new(width, height);
        // Seed the center
        dish.add_chemical(width / 2, height / 2, 1.0);
        dish.add_chemical(width / 2 - 1, height / 2 - 1, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2 + 1, 1.0);

        let platter = Platter::new(width, height);

        Self {
            dish,
            platter,
            running: true,
        }
    }

    fn on_tick(&mut self) {
        // Standard Spots parameters
        let f = 0.055;
        let k = 0.062;
        let dt = 1.0;

        self.dish.update(f, k, dt);

        // Bleed Turing patterns into Platter
        let width = self.dish.width();
        let height = self.dish.height();

        // Decay the entire memory field first
        self.platter.decay(0.95); // Fade memory out

        // Saturate Platter from Gray-Scott V chemical
        let v_grid = self.dish.v();
        for y in 0..height {
            for x in 0..width {
                let v = v_grid[y * width + x];
                if v > 0.1 {
                    // Bleed this value into the heatmap memory, mapping v directly as amount
                    // Saturate limits it to 1.0
                    self.platter.saturate(x, y, v as f64 * 0.5);
                }
            }
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let width = 100;
    let height = 100;
    let mut app = App::new(width, height);

    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app, width, height))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => {
                            // Reset
                            app = App::new(width, height);
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App, width: usize, height: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let gs_v = app.dish.v();
    let platter_mag = app.platter.magnetism();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("gray-platter: Chemical Heatmap Memory"),
        )
        .x_bounds([0.0, width as f64])
        .y_bounds([0.0, height as f64])
        .paint(|ctx| {
            // Draw Platter Heatmap Memory first (background fading trail)
            for y in 0..height {
                for x in 0..width {
                    let mem = platter_mag[y * width + x];
                    if mem > 0.05 {
                        let color = if mem > 0.8 {
                            Color::DarkGray
                        } else if mem > 0.4 {
                            Color::Rgb(50, 50, 50)
                        } else {
                            Color::Rgb(20, 20, 20)
                        };
                        ctx.print(
                            x as f64,
                            y as f64,
                            Span::styled("#", Style::default().fg(color)),
                        );
                    }
                }
            }

            // Draw sharp Turing patterns over the heatmap
            for y in 0..height {
                for x in 0..width {
                    let v = gs_v[y * width + x];
                    if v > 0.1 {
                        let color = if v > 0.5 {
                            Color::Cyan
                        } else if v > 0.3 {
                            Color::Blue
                        } else {
                            Color::LightBlue
                        };
                        ctx.print(
                            x as f64,
                            y as f64,
                            Span::styled(".", Style::default().fg(color)),
                        );
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = Paragraph::new("Press 'q' to quit | 'r' to reset")
        .style(Style::default().fg(Color::Black).bg(Color::Cyan));
    f.render_widget(status, chunks[1]);
}
