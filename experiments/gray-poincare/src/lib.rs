use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use poincare_disk::{hyperbolic_dist, Point};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Context, Points},
        Block, Borders,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res.map_err(|e| anyhow::anyhow!(e))
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    // Create Gray-Scott grid
    let width = 100;
    let height = 100;
    let mut dish = GrayScott::new(width, height);

    // Seed the center
    dish.add_chemical(width / 2, height / 2, 1.0);
    dish.add_chemical(width / 2 - 1, height / 2 - 1, 1.0);
    dish.add_chemical(width / 2 + 1, height / 2 + 1, 1.0);

    loop {
        terminal.draw(|f| ui(f, &mut dish, width, height))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update Gray-Scott
            // Using standard "Spots" parameters
            let f = 0.055;
            let k = 0.062;
            let dt = 1.0;

            dish.update(f, k, dt);

            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, dish: &mut GrayScott, width: usize, height: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)].as_ref())
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("gray-poincare: Hyperbolic Reaction-Diffusion"),
        )
        .x_bounds([-1.5, 1.5])
        .y_bounds([-1.5, 1.5])
        .paint(|ctx: &mut Context| {
            // Map the grid to the disk
            for y in 0..height {
                for x in 0..width {
                    let v_val = dish.v()[dish.get_index(x, y)];
                    if v_val > 0.1 {
                        // Map grid coordinates to complex plane [-1, 1]
                        let cx = (x as f64 / width as f64) * 2.0 - 1.0;
                        let cy = (y as f64 / height as f64) * 2.0 - 1.0;

                        let point = Point::new(cx, cy);

                        // We scale the point by its hyperbolic distance to compress it towards the boundary
                        // The further from center, the denser it gets.
                        let dist = hyperbolic_dist(Point::new(0.0, 0.0), point);

                        let mapped = if dist.is_nan() || dist.is_infinite() {
                            point // Edge cases
                        } else {
                            // Scale by a function of distance to visually map to hyperbolic space
                            let scale = 1.0 - (-dist).exp();
                            Point::new(point.re * scale, point.im * scale)
                        };

                        // Color based on concentration
                        let color = if v_val > 0.5 {
                            Color::Red
                        } else if v_val > 0.3 {
                            Color::Yellow
                        } else {
                            Color::DarkGray
                        };

                        ctx.draw(&Points {
                            coords: &[(mapped.re, mapped.im)],
                            color,
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);
}
