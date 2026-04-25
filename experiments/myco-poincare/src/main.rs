use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use poincare_disk::{hyperbolic_dist, mobius_add, Point};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{f64::consts::PI, io, time::Duration};

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 120;
const AGENT_COUNT: usize = 5000;

struct Agent {
    x: f64,
    y: f64,
    angle: f64,
}

struct World {
    width: usize,
    height: usize,
    trails: Vec<f64>,
    next_trails: Vec<f64>,
    agents: Vec<Agent>,
    center: Point,
}

fn to_poincare(x: f64, y: f64, width: usize, height: usize) -> Point {
    let nx = (x / width as f64) * 2.0 - 1.0;
    let ny = (y / height as f64) * 2.0 - 1.0;
    Point::new(nx, ny)
}

fn from_poincare(pt: Point, width: usize, height: usize) -> (f64, f64) {
    let x = (pt.re + 1.0) * 0.5 * width as f64;
    let y = (pt.im + 1.0) * 0.5 * height as f64;
    (x, y)
}

fn sense(
    agent: &Agent,
    angle_offset: f64,
    sensor_dist: f64,
    center: Point,
    width: usize,
    height: usize,
    trails: &[f64],
) -> f64 {
    let sensor_angle = agent.angle + angle_offset;

    let p_agent = to_poincare(agent.x, agent.y, width, height);

    // Calculate the step in hyperbolic space
    let r2 = p_agent.re * p_agent.re + p_agent.im * p_agent.im;
    if r2 >= 1.0 {
        return 0.0;
    }

    let h_dist = hyperbolic_dist(p_agent, center);

    // Warp the sensor distance based on hyperbolic distance
    // As we get further from center, the apparent distance stretches
    let warped_dist = sensor_dist / (1.0 + h_dist);

    let step_x = sensor_angle.cos() * warped_dist * 0.05; // Scale down for poincare
    let step_y = sensor_angle.sin() * warped_dist * 0.05;
    let step_pt = Point::new(step_x, step_y);

    let sensor_pt = mobius_add(step_pt, p_agent);

    let (sx, sy) = from_poincare(sensor_pt, width, height);

    let sx_idx = (sx.rem_euclid(width as f64)) as usize;
    let sy_idx = (sy.rem_euclid(height as f64)) as usize;

    if sx_idx < width && sy_idx < height {
        trails[sy_idx * width + sx_idx]
    } else {
        0.0
    }
}

impl World {
    fn new(width: usize, height: usize, agent_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = Vec::with_capacity(agent_count);
        for _ in 0..agent_count {
            agents.push(Agent {
                x: rng.gen_range(10.0..(width as f64 - 10.0)),
                y: rng.gen_range(10.0..(height as f64 - 10.0)),
                angle: rng.gen_range(0.0..PI * 2.0),
            });
        }
        Self {
            width,
            height,
            trails: vec![0.0; width * height],
            next_trails: vec![0.0; width * height],
            agents,
            center: Point::new(0.0, 0.0),
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let sensor_angle = std::f64::consts::FRAC_PI_4;
        let sensor_dist = 5.0;
        let turn_speed = 0.2;
        let move_speed = 1.0;

        for agent in &mut self.agents {
            let left = sense(
                agent,
                sensor_angle,
                sensor_dist,
                self.center,
                self.width,
                self.height,
                &self.trails,
            );
            let forward = sense(
                agent,
                0.0,
                sensor_dist,
                self.center,
                self.width,
                self.height,
                &self.trails,
            );
            let right = sense(
                agent,
                -sensor_angle,
                sensor_dist,
                self.center,
                self.width,
                self.height,
                &self.trails,
            );

            if forward > left && forward > right {
                // Keep straight
            } else if forward < left && forward < right {
                // Random turn
                if rng.gen_bool(0.5) {
                    agent.angle += turn_speed;
                } else {
                    agent.angle -= turn_speed;
                }
            } else if right > left {
                agent.angle -= turn_speed;
            } else if left > right {
                agent.angle += turn_speed;
            }

            let p_agent = to_poincare(agent.x, agent.y, self.width, self.height);
            let h_dist = hyperbolic_dist(p_agent, self.center);

            // Warp the move speed based on hyperbolic distance
            let warped_speed = move_speed / (1.0 + h_dist);

            let step_x = agent.angle.cos() * warped_speed * 0.02; // Scale down
            let step_y = agent.angle.sin() * warped_speed * 0.02;
            let step_pt = Point::new(step_x, step_y);

            let mut next_pt = mobius_add(step_pt, p_agent);

            let r2 = next_pt.re * next_pt.re + next_pt.im * next_pt.im;
            if r2 >= 0.99 {
                // Boundary, turn around
                agent.angle += std::f64::consts::PI;
                next_pt = p_agent; // stay
            }

            let (nx, ny) = from_poincare(next_pt, self.width, self.height);

            agent.x = nx;
            agent.y = ny;

            let ix = agent.x as usize;
            let iy = agent.y as usize;

            if ix < self.width && iy < self.height {
                self.trails[iy * self.width + ix] = 255.0;
            }
        }

        // Diffuse & Decay
        let decay = 0.9;
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                        sum += self.trails[ny * w + nx];
                    }
                }
                self.next_trails[y * w + x] = (sum / 9.0) * decay;
            }
        }

        std::mem::swap(&mut self.trails, &mut self.next_trails);
    }
}

struct App {
    world: World,
    tick: u64,
}

impl App {
    fn new() -> Self {
        Self {
            world: World::new(GRID_WIDTH, GRID_HEIGHT, AGENT_COUNT),
            tick: 0,
        }
    }
    fn update(&mut self) {
        self.world.update();
        self.tick += 1;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + std::error::Error + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
        app.update();
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("🍄 Myco Poincaré (Hyperbolic Foraging) 🌌"),
        )
        .x_bounds([0.0, GRID_WIDTH as f64])
        .y_bounds([0.0, GRID_HEIGHT as f64])
        .paint(|ctx| {
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    let val = app.world.trails[y * GRID_WIDTH + x];
                    if val > 10.0 {
                        // Invert Y for canvas
                        let cy = GRID_HEIGHT as f64 - 1.0 - (y as f64);

                        // Check if in disk
                        let nx = (x as f64 / GRID_WIDTH as f64) * 2.0 - 1.0;
                        let ny = (y as f64 / GRID_HEIGHT as f64) * 2.0 - 1.0;
                        if nx * nx + ny * ny < 1.0 {
                            let color = if val > 150.0 {
                                Color::Rgb(255, 255, 0)
                            } else if val > 50.0 {
                                Color::Rgb(0, 255, 0)
                            } else {
                                Color::Rgb(0, 100, 0)
                            };

                            ctx.draw(&Points {
                                coords: &[(x as f64, cy)],
                                color,
                            });
                        }
                    }
                }
            }

            // Draw unit disk boundary
            let r = GRID_WIDTH as f64 / 2.0;
            let cx = GRID_WIDTH as f64 / 2.0;
            let cy = GRID_HEIGHT as f64 / 2.0;
            ctx.draw(&ratatui::widgets::canvas::Circle {
                x: cx,
                y: cy,
                radius: r,
                color: Color::DarkGray,
            });
        });

    f.render_widget(canvas, chunks[0]);

    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let sim_info = Paragraph::new(vec![ratatui::text::Line::from(vec![
        Span::raw("Tick: "),
        Span::styled(
            format!("{}", app.tick),
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw(format!(" | Agents: {}", AGENT_COUNT)),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Simulation Info"),
    );
    f.render_widget(sim_info, stats_chunks[0]);

    let controls = Paragraph::new(vec![ratatui::text::Line::from(
        "Press 'q' or 'Esc' to quit",
    )])
    .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls, stats_chunks[1]);
}
