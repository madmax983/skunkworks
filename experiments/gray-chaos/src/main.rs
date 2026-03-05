use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use locus::Vec2;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod chaos;
use chaos::DoublePendulum;

struct App {
    gs: GrayScott,
    pendulum: DoublePendulum,
    width: usize,
    height: usize,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let mut gs = GrayScott::new(width, height);
        // Seed the center
        for dy in -5..=5 {
            for dx in -5..=5 {
                let x = (width as isize / 2 + dx).max(0).min(width as isize - 1) as usize;
                let y = (height as isize / 2 + dy).max(0).min(height as isize - 1) as usize;
                gs.add_chemical(x, y, 1.0);
            }
        }

        let origin = Vec2::new(width as f64 / 2.0, height as f64 / 2.0);
        let pendulum = DoublePendulum::new(origin);

        Self {
            gs,
            pendulum,
            width,
            height,
        }
    }

    fn update(&mut self) {
        // Double pendulum step
        self.pendulum.update(0.05);

        // Map pendulum tip to grid coordinates
        let p2 = self.pendulum.p2();
        let px = p2.x.round() as isize;
        let py = p2.y.round() as isize;

        // Add chemical V where the pendulum tip is
        if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
            self.gs.add_chemical(px as usize, py as usize, 0.5);

            // Interaction: get chemical gradient to perturb pendulum
            let idx = self.gs.get_index(px as usize, py as usize);
            let u_val = self.gs.u()[idx];
            let v_val = self.gs.v()[idx];

            // High V concentration slows down or deflects the pendulum
            if v_val > 0.3 {
                self.pendulum.a1_v -= (v_val as f64) * 0.05;
                self.pendulum.a2_v += (u_val as f64) * 0.05;
            }
        }

        // Standard GS update parameters
        let feed = 0.055;
        let kill = 0.062;
        self.gs.update(feed, kill, 1.0);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let size = terminal.size()?;
    // Compute drawing area width and height approximately (canvas block area)
    let _draw_width = (size.width - 2) as usize * 2; // canvas points x bounds
    let _draw_height = (size.height - 4) as usize * 2; // canvas points y bounds

    // We will fix the internal simulation resolution
    let sim_width = 160;
    let sim_height = 100;

    let app = App::new(sim_width, sim_height);
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>
where
    std::io::Error: From<<B as ratatui::backend::Backend>::Error>,
{
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('k') | KeyCode::Char('K') => {
                            // Kick the pendulum manually
                            app.pendulum.a1_v += 15.0;
                            app.pendulum.a2_v -= 15.0;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let title = Paragraph::new(format!(
        " 🧬 gray-chaos | Gray-Scott x Double Pendulum | Press 'k' to kick | Press 'q' to quit"
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let mut gs_points = vec![];
    let v_data = app.gs.v();

    for y in 0..app.height {
        for x in 0..app.width {
            let val = v_data[app.gs.get_index(x, y)];
            if val > 0.2 {
                gs_points.push((x as f64, y as f64));
            }
        }
    }

    let p1 = app.pendulum.p1();
    let p2 = app.pendulum.p2();
    let origin = app.pendulum.origin;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .marker(ratatui::symbols::Marker::Block)
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, app.height as f64])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &gs_points,
                color: Color::Magenta,
            });

            // Draw Pendulum Segments
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: origin.x,
                y1: origin.y,
                x2: p1.x,
                y2: p1.y,
                color: Color::Yellow,
            });

            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: p1.x,
                y1: p1.y,
                x2: p2.x,
                y2: p2.y,
                color: Color::White,
            });
        });

    f.render_widget(canvas, chunks[1]);
}
