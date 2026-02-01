#![allow(clippy::collapsible_if)]
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use crate::physics::{LorenzSystem, LorenzState};
use crate::monitor::SystemMonitor;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};

pub struct App {
    system: LorenzSystem,
    state: LorenzState,
    monitor: SystemMonitor,
    trail: Vec<(f64, f64, f64)>, // x, y, z
    max_trail: usize,
    rotation_angle: f64,
    current_rho: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            system: LorenzSystem::new(10.0, 28.0, 8.0/3.0),
            state: LorenzState::new(0.1, 0.0, 0.0),
            monitor: SystemMonitor::new(),
            trail: Vec::with_capacity(2000),
            max_trail: 2000,
            rotation_angle: 0.0,
            current_rho: 28.0,
        }
    }

    pub fn run(mut self, terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>) -> std::io::Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut last_monitor_update = Instant::now();

        // Initial monitor update
        self.update_monitor();

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                    if let KeyCode::Char('r') = key.code {
                        self.state = LorenzState::new(0.1, 0.0, 0.0);
                        self.trail.clear();
                    }
                    if let KeyCode::Left = key.code {
                        self.rotation_angle -= 0.1;
                    }
                    if let KeyCode::Right = key.code {
                        self.rotation_angle += 0.1;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }

            if last_monitor_update.elapsed() >= Duration::from_secs(1) {
                self.update_monitor();
                last_monitor_update = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        let dt = 0.005;
        // Run physics faster than render
        for _ in 0..5 {
            self.state = self.system.integrate(&self.state, dt);
            self.trail.push((self.state.x, self.state.y, self.state.z));
            if self.trail.len() > self.max_trail {
                self.trail.remove(0);
            }
        }
    }

    fn update_monitor(&mut self) {
        let rho = self.monitor.get_chaos_parameter();
        self.system.rho = rho;
        self.current_rho = rho;
    }

    fn ui(&self, frame: &mut Frame) {
        let area = frame.area();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        let main_area = layout[0];
        let status_area = layout[1];

        // Chaos Color based on rho
        // 10.0 -> Blue, 28.0 -> Green, 50.0 -> Red
        let color = if self.current_rho < 20.0 {
            Color::Blue
        } else if self.current_rho < 35.0 {
            Color::Green
        } else {
            Color::Red
        };

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("System Weather: Lorenz Attractor"))
            .x_bounds([-30.0, 30.0])
            .y_bounds([-30.0, 30.0])
            .paint(|ctx| {
                // Draw trailing
                for i in 0..self.trail.len().saturating_sub(1) {
                    let (x1, y1, z1) = self.trail[i];
                    let (x2, y2, z2) = self.trail[i+1];

                    // Project 3D to 2D
                    // Rotate around Z axis (vertical)
                    // This rotates the object in the XY plane.
                    // We plot Rotated X vs Z.
                    let (rot_x1, _rot_y1) = rotate_z(x1, y1, self.rotation_angle);
                    let (rot_x2, _rot_y2) = rotate_z(x2, y2, self.rotation_angle);

                    ctx.draw(&CanvasLine {
                        x1: rot_x1,
                        y1: z1 - 25.0, // Center Z around 25
                        x2: rot_x2,
                        y2: z2 - 25.0,
                        color,
                    });
                }
            });

        frame.render_widget(canvas, main_area);

        let status_text = format!(
            "Rho: {:.2} | CPU Impact: {:.1}% | Rotation: {:.2} | Controls: < Left/Right > [r]eset | Quit: q",
            self.current_rho,
            (self.current_rho - 10.0) / 40.0 * 100.0,
            self.rotation_angle
        );
        let status = Paragraph::new(status_text).style(Style::default().fg(Color::White).bg(Color::Black));
        frame.render_widget(status, status_area);
    }
}

fn rotate_z(x: f64, y: f64, angle: f64) -> (f64, f64) {
    let cos = angle.cos();
    let sin = angle.sin();
    (x * cos - y * sin, x * sin + y * cos)
}
