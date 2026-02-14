use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, MouseEvent, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

mod lbm;
use lbm::{FluidSim, HEIGHT, WIDTH};

struct App {
    sim: FluidSim,
    paused: bool,
    should_quit: bool,
    show_curl: bool,
}

impl App {
    fn new() -> Self {
        Self {
            sim: FluidSim::new(),
            paused: false,
            should_quit: false,
            show_curl: true,
        }
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.sim.step();
        }
    }

    fn on_mouse(&mut self, mouse: MouseEvent) {
        let _x = mouse.column as usize;
        let _y = mouse.row as usize;
        // Map terminal coordinates to simulation coordinates
        // Canvas usually fits to the rect.
        // We need to know the rect area to map correctly.
        // This is hard inside `on_mouse` without passing the area.
        // For now, let's just assume we can map roughly or pass the logic later.
        // Actually, let's handle mouse logic in the main loop where we might know area or just ignore precise mouse for now.
        // Or we can just use relative movement if we had `MouseMotion`.

        match mouse.kind {
            MouseEventKind::Drag(_button) | MouseEventKind::Down(_button) => {
                 // Simple interaction: Add density at random or center?
                 // We can't easily map mouse pos to canvas pos without layout info.
                 // Let's rely on keyboard for "Stir" and mouse for just "Activity".
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Char('p') => app.paused = !app.paused,
                    KeyCode::Char('c') => app.show_curl = !app.show_curl,
                    KeyCode::Char('r') => app.sim = FluidSim::new(),
                    KeyCode::Char(' ') => {
                        // Splash in center
                        app.sim.add_density(WIDTH / 2, HEIGHT / 2, 10.0);
                        app.sim.add_velocity(WIDTH / 2, HEIGHT / 2, 2.0, 1.0);
                    }
                     KeyCode::Char('w') => {
                        // Wind from left
                        for y in 20..40 {
                            app.sim.add_velocity(5, y, 0.5, 0.0);
                            app.sim.add_density(5, y, 0.5);
                        }
                    }
                    _ => {}
                },
                Event::Mouse(mouse) => app.on_mouse(mouse),
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Main
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    // Header
    let title = Paragraph::new(format!(
        "KLEIN-FLUID | {} | Space: Splash | W: Wind | C: Toggle Curl",
        if app.paused { "PAUSED" } else { "RUNNING" }
    ))
    .style(Style::default().fg(Color::Cyan).bg(Color::Black));
    f.render_widget(title, chunks[0]);

    // Canvas
    let _canvas_area = chunks[1];

    // We want to draw pixels.
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Twisted Manifold"))
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .paint(|ctx| {
            // Draw fluid
            // We iterate over the grid and draw points where density/curl is high.
            // Since ctx.draw is a closure, we can't easily iterate and draw efficiently if we do it one by one?
            // `ctx.draw` takes a `Shape`. `Points` is a shape.
            // We can collect points into a vector.

            // To visualize Curl (Vorticity): Blue/Red.
            // To visualize Density: White/Gray.

            let mut points_pos = Vec::new();
            let mut points_neg = Vec::new();
            let mut points_density = Vec::new();

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let idx = y * WIDTH + x;

                    // Flip Y for rendering because Canvas 0,0 is usually bottom-left?
                    // LBM (0,0) is top-left in our logic (y increases down).
                    // But Canvas y increases up.
                    // So we map y -> HEIGHT - 1 - y.
                    let draw_y = (HEIGHT - 1 - y) as f64;
                    let draw_x = x as f64;

                    if app.show_curl {
                        let curl = app.sim.curl[idx];
                        if curl > 0.1 {
                            points_pos.push((draw_x, draw_y));
                        } else if curl < -0.1 {
                            points_neg.push((draw_x, draw_y));
                        }
                    } else {
                        let rho = app.sim.density[idx];
                        if rho > 1.2 {
                            points_density.push((draw_x, draw_y));
                        }
                    }
                }
            }

            ctx.draw(&ratatui::widgets::canvas::Points {
                coords: &points_pos,
                color: Color::Red,
            });
            ctx.draw(&ratatui::widgets::canvas::Points {
                coords: &points_neg,
                color: Color::Blue,
            });
             ctx.draw(&ratatui::widgets::canvas::Points {
                coords: &points_density,
                color: Color::White,
            });

            // Visualize Twist Boundary (Left/Right)
            // Draw arrows or lines to show A -> A'
            ctx.print(0.0, HEIGHT as f64 / 2.0, "A");
            ctx.print(WIDTH as f64 - 2.0, HEIGHT as f64 / 2.0, "A'");
        });

    f.render_widget(canvas, chunks[1]);
}
