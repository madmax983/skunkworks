use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use flocking::{compute_force, FlockingParams};
use locus::{Topology, Vec2};
use rand::Rng;
use ratatui::{

    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct LocusFlockApp {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
    topology: Topology,
    width: i64,
    height: i64,
}

impl LocusFlockApp {
    fn new(count: usize, width: i64, height: i64, topology: Topology) -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(count);
        let mut velocities = Vec::with_capacity(count);

        for _ in 0..count {
            positions.push(Vec2::new(
                rng.gen_range(0.0..(width as f64)),
                rng.gen_range(0.0..(height as f64)),
            ));
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            velocities.push(Vec2::new(angle.cos() * 2.0, angle.sin() * 2.0));
        }

        let params = FlockingParams {
            view_radius: 15.0,
            separation_radius: 3.0,
            max_speed: 1.0,
            max_force: 0.05,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        Self {
            positions,
            velocities,
            params,
            topology,
            width,
            height,
        }
    }

    fn tick(&mut self) {
        let mut new_velocities = self.velocities.clone();

        for i in 0..self.positions.len() {
            // Apply topological wrapping to positions so agents see "ghosts" across borders
            // For true topological distance, we should ideally adjust compute_force,
            // but as an approximation, we create a wrapped copy of positions relative to this agent.

            // To make this simple, we just use standard compute_force for now.
            // The true hybrid trait is how they wrap around the edges.
            let mut virtual_positions = self.positions.clone();

            // Re-center virtual positions to mimic topology (Torus)
            if matches!(self.topology, Topology::Torus) {
               for j in 0..virtual_positions.len() {
                   if i == j { continue; }
                   let dx = virtual_positions[j].x - self.positions[i].x;
                   let dy = virtual_positions[j].y - self.positions[i].y;

                   let w = self.width as f64;
                   let h = self.height as f64;

                   if dx > w / 2.0 { virtual_positions[j].x -= w; }
                   if dx < -w / 2.0 { virtual_positions[j].x += w; }
                   if dy > h / 2.0 { virtual_positions[j].y -= h; }
                   if dy < -h / 2.0 { virtual_positions[j].y += h; }
               }
            }

            let force = compute_force(&virtual_positions, &self.velocities, i, &self.params);
            new_velocities[i] += force;

            // Cap speed
            let speed = new_velocities[i].magnitude();
            if speed > self.params.max_speed {
                new_velocities[i] = new_velocities[i].normalize() * self.params.max_speed;
            }
        }

        self.velocities = new_velocities;

        for i in 0..self.positions.len() {
            self.positions[i] += self.velocities[i];

            // Apply topological bounds
            let y_idx = self.positions[i].y.floor() as i64;
            let x_idx = self.positions[i].x.floor() as i64;

            if let Some((ny, nx)) = self.topology.normalize(y_idx, x_idx, self.width as usize, self.height as usize) {
                // Update coordinate to wrapped position, keeping fractional part
                let frac_x = self.positions[i].x - self.positions[i].x.floor();
                let frac_y = self.positions[i].y - self.positions[i].y.floor();
                self.positions[i].x = nx as f64 + frac_x;
                self.positions[i].y = ny as f64 + frac_y;
            } else {
                // If None, we hit a wall (e.g., Plane). Bounce.
                if x_idx < 0 {
                    self.positions[i].x = 0.0;
                    self.velocities[i].x *= -1.0;
                } else if x_idx >= self.width {
                    self.positions[i].x = (self.width - 1) as f64;
                    self.velocities[i].x *= -1.0;
                }

                if y_idx < 0 {
                    self.positions[i].y = 0.0;
                    self.velocities[i].y *= -1.0;
                } else if y_idx >= self.height {
                    self.positions[i].y = (self.height - 1) as f64;
                    self.velocities[i].y *= -1.0;
                }
            }
        }
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let title = format!(" 🧬 Splice: locus × flocking | Topology: {:?} ", self.topology);
        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                for pos in &self.positions {
                    // Map to 0-100 for canvas
                    let cx = (pos.x / self.width as f64) * 100.0;
                    let cy = (1.0 - (pos.y / self.height as f64)) * 100.0; // Invert Y for canvas
                    ctx.print(cx, cy, Span::styled("·", Style::default().fg(Color::Cyan)));
                }
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        frame.render_widget(canvas, frame.area());
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|arg| arg == "--headless");

    let mut tui = if headless { None } else { Some(Tui::init()?) };
    let mut app = LocusFlockApp::new(100, 120, 40, Topology::Torus);

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();
    let mut frame_count = 0;

    loop {
        if headless {
            app.tick();
            frame_count += 1;
            if frame_count > 50 {
                println!("🧬 locus-flock running in headless mode for 50 ticks.");
                break;
            }
            continue;
        }

        if let Some(tui) = &mut tui {
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
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab {
                        app.topology = match app.topology {
                            Topology::Plane => Topology::Torus,
                            Topology::Torus => Topology::Klein,
                            Topology::Klein => Topology::CylinderH,
                            Topology::CylinderH => Topology::CylinderV,
                            Topology::CylinderV => Topology::Mobius,
                            Topology::Mobius => Topology::Plane,
                            _ => Topology::Plane,
                        };
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.tick();
                last_tick = Instant::now();
            }
        }
    }

    Ok(())
}
