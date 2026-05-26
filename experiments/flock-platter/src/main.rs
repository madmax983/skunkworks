use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use platter::Platter;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

/// 🧬 Lineage Documentation:
///
/// **Parent A (`crates/flocking`)**: Provides the swarm intelligence and boid rules.
/// **Parent B (`crates/platter`)**: Provides the continuous scalar field (heat/pheromone map).
///
/// **Emergence**: Boids deposit "heat" (pheromones) into the scalar field as they move.
/// The continuous field then decays over time. The boids sense the gradient of the heat field
/// around them and steer towards it, creating positive feedback loops that result in
/// self-organizing swarm highways and complex macroscopic pheromone trails.
///
/// This bridges discrete swarm intelligence with continuous topographical decay.

struct Boid {
    position: Vec2,
    velocity: Vec2,
}

struct App {
    boids: Vec<Boid>,
    platter: Platter,
    params: FlockingParams,
    width: u16,
    height: u16,
}

impl App {
    fn new(width: u16, height: u16) -> Self {
        let platter = Platter::new(width as usize, height as usize);
        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 3.0,
            max_speed: 1.5,
            max_force: 0.1,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let mut boids = Vec::new();
        // Spawn 50 boids randomly
        for i in 0..50 {
            let angle = (i as f64) * 0.5;
            let radius = 10.0 + (i as f64) % 10.0;
            let cx = (width as f64) / 2.0;
            let cy = (height as f64) / 2.0;

            boids.push(Boid {
                position: Vec2::new(cx + angle.cos() * radius, cy + angle.sin() * radius),
                velocity: Vec2::new(angle.cos() * params.max_speed, angle.sin() * params.max_speed),
            });
        }

        Self {
            boids,
            platter,
            params,
            width,
            height,
        }
    }

    fn step(&mut self) {
        // Collect positions and velocities
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        // Compute forces and update
        for i in 0..self.boids.len() {
            let flock_force = compute_force(&positions, &velocities, i, &self.params);

            // Platter Pheromone gradient force
            let px = positions[i].x as usize;
            let py = positions[i].y as usize;
            let mut gradient = Vec2::zero();

            // Steer towards heat
            if px > 0 && px < self.width as usize - 1 && py > 0 && py < self.height as usize - 1 {
                let heat_left = self.platter.get(px - 1, py);
                let heat_right = self.platter.get(px + 1, py);
                let heat_up = self.platter.get(px, py - 1);
                let heat_down = self.platter.get(px, py + 1);

                gradient.x = heat_right - heat_left;
                gradient.y = heat_down - heat_up; // Y grows down
                if gradient.magnitude_squared() > 0.001 {
                    gradient = gradient.normalize() * 0.05; // weak pull towards heat
                }
            }

            self.boids[i].velocity += flock_force + gradient;

            // Limit speed
            if self.boids[i].velocity.magnitude_squared() > self.params.max_speed * self.params.max_speed {
                self.boids[i].velocity = self.boids[i].velocity.normalize() * self.params.max_speed;
            }

            let vel = self.boids[i].velocity;
            self.boids[i].position += vel;

            // Wrap around boundaries
            if self.boids[i].position.x < 0.0 { self.boids[i].position.x += self.width as f64; }
            if self.boids[i].position.x >= self.width as f64 { self.boids[i].position.x -= self.width as f64; }
            if self.boids[i].position.y < 0.0 { self.boids[i].position.y += self.height as f64; }
            if self.boids[i].position.y >= self.height as f64 { self.boids[i].position.y -= self.height as f64; }

            // Deposit heat onto platter
            // Fix rounding out of bounds issue by modulo wrapping before casting to usize
            let nx = ((self.boids[i].position.x.trunc() as isize).rem_euclid(self.width as isize)) as usize;
            let ny = ((self.boids[i].position.y.trunc() as isize).rem_euclid(self.height as isize)) as usize;
            self.platter.saturate(nx, ny, 1.0); // max heat
        }

        // Decay platter
        self.platter.decay(0.95);
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    let width = size.width.min(app.width);
    let height = size.height.min(app.height);

    // Create a 2D grid for rendering
    let mut grid = vec![vec![' '; width as usize]; height as usize];
    let mut colors = vec![vec![Color::Black; width as usize]; height as usize];

    // Render Platter heat
    for y in 0..height {
        for x in 0..width {
            let heat = app.platter.get(x as usize, y as usize);
            if heat > 0.05 {
                grid[y as usize][x as usize] = '.';
                colors[y as usize][x as usize] = if heat > 0.8 {
                    Color::Red
                } else if heat > 0.4 {
                    Color::Yellow
                } else {
                    Color::DarkGray
                };
            }
        }
    }

    // Render boids
    for boid in &app.boids {
        let x = boid.position.x.round() as usize;
        let y = boid.position.y.round() as usize;
        if x < width as usize && y < height as usize {
            grid[y][x] = '>'; // Simplification, could rotate char based on vel
            colors[y][x] = Color::White;
        }
    }

    let mut lines = Vec::new();
    for y in 0..height {
        let mut spans = Vec::new();
        for x in 0..width {
            let mut style = Style::default().fg(colors[y as usize][x as usize]);
            if colors[y as usize][x as usize] == Color::Black {
                style = Style::default();
            }
            spans.push(Span::styled(grid[y as usize][x as usize].to_string(), style));
        }
        lines.push(ratatui::text::Line::from(spans));
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Flock Platter - Pheromone Swarming"));
    f.render_widget(p, size);
}

fn main() -> Result<(), io::Error> {
    // Check for headless mode (CI/tests)
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Simulation step successful.");
        let mut app = App::new(40, 20);
        app.step();
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(100, 40);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') | KeyCode::Esc = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.step();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
