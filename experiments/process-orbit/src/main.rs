use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Context},
        Block, Borders,
    },
    Frame,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use tui_shared::Tui;

struct ProcessBody {
    name: String,
    cpu_usage: f32,
    memory: u64,
    angle: f64,
    radius: f64,
    color: Color,
}

struct App {
    system: System,
    bodies: HashMap<sysinfo::Pid, ProcessBody>,
    running: bool,
}

impl App {
    fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
        );
        system.refresh_all(); // First refresh to populate
        Self {
            system,
            bodies: HashMap::new(),
            running: true,
        }
    }

    fn update(&mut self, dt: f64) {
        // Refresh processes
        self.system.refresh_processes();
        self.system.refresh_cpu();
        self.system.refresh_memory();

        // Update bodies
        for (pid, process) in self.system.processes() {
            let cpu = process.cpu_usage(); // usually 0.0 to 100.0+ per core
            let memory = process.memory();

            // Determine orbit radius based on CPU.
            let radius = calculate_orbit_radius(cpu);

            // Angular velocity
            // speed = 1.0 + cpu * 0.1
            let speed = 0.5 + (cpu as f64) * 0.05;

            // Integrate angle: new_angle = old_angle + speed * dt
            let angle = if let Some(old_body) = self.bodies.get(pid) {
                old_body.angle + speed * dt
            } else {
                pid.as_u32() as f64 // Random start angle based on PID
            };

            // Color based on User vs System?
            // Or Memory usage?
            // Let's use Memory for Color Intensity (Blue -> Red) or just a fixed color.
            // Let's pick a color from a palette based on PID hash.
            let color = match pid.as_u32() % 6 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Yellow,
                4 => Color::Magenta,
                5 => Color::Cyan,
                _ => Color::White,
            };

            self.bodies.insert(
                *pid,
                ProcessBody {
                    name: process.name().to_string(),
                    cpu_usage: cpu,
                    memory,
                    angle,
                    radius,
                    color,
                },
            );
        }

        // Remove dead processes
        let current_pids: Vec<sysinfo::Pid> = self.system.processes().keys().cloned().collect();
        self.bodies.retain(|pid, _| current_pids.contains(pid));
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    let mut tui = Tui::init()?;

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    app.running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            app.update(dt);
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    // Solar System Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Process Orbit "))
        .x_bounds([-120.0, 120.0])
        .y_bounds([-120.0, 120.0]) // Adjusted for standard aspect
        .paint(|ctx| {
            draw_system(ctx, app);
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let global_cpu = app.system.global_cpu_info().cpu_usage();
    let total_memory = app.system.total_memory();
    let used_memory = app.system.used_memory();

    let status = Line::from(vec![
        Span::raw(" CPU: "),
        Span::styled(format!("{:.1}%", global_cpu), Style::default().fg(if global_cpu > 80.0 { Color::Red } else { Color::Green })),
        Span::raw(" | MEM: "),
        Span::styled(format!("{}/{} MB", used_memory / 1024 / 1024, total_memory / 1024 / 1024), Style::default().fg(Color::Cyan)),
        Span::raw(" | Processes: "),
        Span::raw(app.bodies.len().to_string()),
        Span::raw(" | 'q' to Quit"),
    ]);

    f.render_widget(Block::default().borders(Borders::TOP).title(status), chunks[1]);
}

fn draw_system(ctx: &mut Context, app: &App) {
    // Draw Sun (Kernel/System Load)
    let global_cpu = app.system.global_cpu_info().cpu_usage();
    let sun_radius = 5.0 + (global_cpu as f64) * 0.1;
    let sun_color = if global_cpu > 80.0 {
        Color::Red
    } else if global_cpu > 40.0 {
        Color::Yellow
    } else {
        Color::White
    };

    ctx.draw(&Circle {
        x: 0.0,
        y: 0.0,
        radius: sun_radius,
        color: sun_color,
    });

    // Draw Planets (Processes)
    // We only draw top N processes to avoid clutter?
    // Or just all of them.
    for body in app.bodies.values() {
        let radius = body.radius;

        let x = radius * body.angle.cos();
        let y = radius * body.angle.sin();

        // Size based on memory?
        // Map memory (bytes) to size [0.5, 3.0]
        // 1GB = 1073741824 bytes
        let mem_mb = body.memory as f64 / 1024.0 / 1024.0;
        let size = (mem_mb / 500.0).clamp(0.5, 3.0);

        ctx.draw(&Circle {
            x,
            y,
            radius: size,
            color: body.color,
        });

        // If it's a big process, label it?
        if body.cpu_usage > 5.0 || mem_mb > 500.0 {
             ctx.print(x + size, y + size, Span::styled(body.name.clone(), Style::default().fg(body.color)));
        }
    }
}

fn calculate_orbit_radius(cpu: f32) -> f64 {
    // High CPU -> Close to center (Gravity).
    // CPU 0 -> Far away.
    // Map CPU [0, 100] to Radius [100.0, 10.0]
    // R = Base + (Max - Base) * exp(-k * cpu)
    10.0 + 90.0 * (-0.05 * (cpu as f64)).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_radius() {
        let r_zero = calculate_orbit_radius(0.0);
        let r_mid = calculate_orbit_radius(50.0);
        let r_high = calculate_orbit_radius(100.0);

        // Zero CPU should be furthest out (around 100.0)
        assert!(r_zero > 99.0);

        // Higher CPU should be closer
        assert!(r_mid < r_zero);
        assert!(r_high < r_mid);

        // High CPU should be close to 10.0
        assert!(r_high > 10.0);
    }
}
