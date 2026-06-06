use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use quipu::{Quipu, Cord, Knot};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    env,
    io,
    time::{Duration, Instant},
};

const TICK_RATE: Duration = Duration::from_millis(50);

struct App {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
    quipu: Quipu,
    running: bool,
    width: usize,
    height: usize,
    ticks: usize,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let mut positions = Vec::new();
        let mut velocities = Vec::new();

        for i in 0..30 {
            positions.push(Vec2::new(
                (1234.5678 * (i as f64 + 1.0)).fract() * width as f64,
                (8765.4321 * (i as f64 + 1.0)).fract() * height as f64,
            ));
            velocities.push(Vec2::new(
                ((345.67 * (i as f64 + 1.0)).fract() - 0.5) * 5.0,
                ((765.43 * (i as f64 + 1.0)).fract() - 0.5) * 5.0,
            ));
        }

        let params = FlockingParams {
            view_radius: 15.0,
            separation_radius: 4.0,
            max_speed: 2.0,
            max_force: 0.3,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.2,
        };

        let mut quipu = Quipu::new();
        let mut c = Cord::new();
        c.clusters.push(Vec::new());
        quipu.add_cord(c);

        Self {
            positions,
            velocities,
            params,
            quipu,
            running: true,
            width,
            height,
            ticks: 0,
        }
    }

    fn update(&mut self) {
        let forces: Vec<Vec2> = (0..self.positions.len())
            .map(|i| compute_force(&self.positions, &self.velocities, i, &self.params))
            .collect();

        for (i, force) in forces.iter().enumerate() {
            self.velocities[i] += *force;
            self.velocities[i] = self.velocities[i].limit(self.params.max_speed);
            self.positions[i] += self.velocities[i];

            if self.positions[i].x < 0.0 {
                self.positions[i].x += self.width as f64;
            } else if self.positions[i].x >= self.width as f64 {
                self.positions[i].x -= self.width as f64;
            }
            if self.positions[i].y < 0.0 {
                self.positions[i].y += self.height as f64;
            } else if self.positions[i].y >= self.height as f64 {
                self.positions[i].y -= self.height as f64;
            }
        }

        self.ticks += 1;
        if self.ticks % 20 == 0 {
            // Count clusters (boids close to each other)
            let mut clusters = 0;
            for i in 0..self.positions.len() {
                let mut count = 0;
                for j in 0..self.positions.len() {
                    if i != j {
                        let dist = (self.positions[i] - self.positions[j]).magnitude();
                        if dist < self.params.view_radius {
                            count += 1;
                        }
                    }
                }
                if count > 3 {
                    clusters += 1;
                }
            }

            if clusters > 0 {
                let cord_idx = self.quipu.cords.len() - 1;
                if self.quipu.cords[cord_idx].clusters[0].len() > 10 {
                    let mut c = Cord::new();
                    c.clusters.push(Vec::new());
                    self.quipu.add_cord(c);
                }
                let cord_idx = self.quipu.cords.len() - 1;
                if let Some(cord) = self.quipu.cords.get_mut(cord_idx) {
                    let val = clusters as u32;
                    let num_knots = if val > 9 { 9 } else { val as u8 };
                    if num_knots == 1 {
                        cord.clusters[0].push(Knot::FigureEight);
                    } else if num_knots > 1 {
                        cord.clusters[0].push(Knot::Long(num_knots));
                    }
                }
            }
        }
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Swarm-Driven Knotted Data Storage 🦅🧵 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            for y in 0..self.height.min(inner_area.height as usize - 4) {
                let mut spans = Vec::new();
                for x in 0..self.width.min(inner_area.width as usize) {
                    let mut is_boid = false;
                    for p in &self.positions {
                        let bx = p.x as usize;
                        let by = p.y as usize;
                        if bx == x && by == y {
                            is_boid = true;
                            break;
                        }
                    }

                    let ch = if is_boid { "v" } else { " " };
                    let fg = if is_boid { Color::Cyan } else { Color::Reset };
                    spans.push(Span::styled(ch, Style::default().fg(fg)));
                }
                lines.push(Line::from(spans));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Quipu Data Structure:", Style::default().fg(Color::Yellow))));

            for (i, cord) in self.quipu.cords.iter().enumerate() {
                let mut cord_str = format!("Cord {}: ", i);
                for knot in &cord.clusters[0] {
                    match knot {
                        Knot::FigureEight => cord_str.push_str("8-"),
                        Knot::Long(n) => cord_str.push_str(&format!("L{}-", n)),
                        Knot::Simple => cord_str.push_str("S-"),
                    }
                }
                lines.push(Line::from(cord_str));
            }

            f.render_widget(Paragraph::new(lines), inner_area);
        })?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let width = 80;
    let height = 30;

    if args.contains(&"--headless".to_string()) {
        let mut app = App::new(width, height);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(width, height);
    let mut last_tick = Instant::now();

    while app.running {
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.update();
            last_tick = Instant::now();
        }

        app.render(&mut terminal)?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
