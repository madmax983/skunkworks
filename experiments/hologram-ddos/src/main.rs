use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod hologram;
mod simulation;
use hologram::Hologram;
use simulation::{World, WORLD_SIZE};

struct App {
    world: World,
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let mut world = World::new();
        // Pre-warm the simulation or add some default firewalls
        world.add_firewall((WORLD_SIZE / 2.0 - 150.0, WORLD_SIZE / 2.0), 50.0);
        world.add_firewall((WORLD_SIZE / 2.0 + 150.0, WORLD_SIZE / 2.0), 50.0);
        world.add_firewall((WORLD_SIZE / 2.0, WORLD_SIZE / 2.0 - 150.0), 50.0);

        let mut app = Self {
            world,
            hologram: Hologram::new(256, 128), // Standard TUI hologram resolution
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            status_msg: "Arrow Keys: Angle | Space: Firewall | C: Clear | Esc: Quit".into(),
            reconstruction_data: vec![],
        };
        app.update_hologram();
        app
    }

    fn update_hologram(&mut self) {
        self.world.update();
        self.hologram = Hologram::from_world(&self.world, 256, 128);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>
    where std::io::Error: From<<B as Backend>::Error>
    {
        let mut last_tick = std::time::Instant::now();
        let tick_rate = Duration::from_millis(32); // ~30fps

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                                KeyCode::Left => {
                                    self.reconstruction_angle_x -= 1;
                                }
                                KeyCode::Right => {
                                    self.reconstruction_angle_x += 1;
                                }
                                KeyCode::Up => {
                                    self.reconstruction_angle_y += 1;
                                }
                                KeyCode::Down => {
                                    self.reconstruction_angle_y -= 1;
                                }
                                KeyCode::Char('c') | KeyCode::Char('C') => {
                                    self.world.clear_firewalls();
                                    self.status_msg = "Firewalls Cleared.".into();
                                }
                                KeyCode::Enter => {
                                    self.reconstruction_angle_x = -20;
                                    self.reconstruction_angle_y = -10;
                                    self.status_msg = "Reset to Recording Angle.".into();
                                }
                                KeyCode::Char(' ') => {
                                    // Add a random firewall
                                    use rand::Rng;
                                    let mut rng = rand::thread_rng();
                                    let fx = rng.gen_range(100.0..WORLD_SIZE - 100.0);
                                    let fy = rng.gen_range(100.0..WORLD_SIZE - 100.0);
                                    let fr = rng.gen_range(20.0..80.0);
                                    self.world.add_firewall((fx, fy), fr);
                                    self.status_msg = format!("Firewall added at ({:.0}, {:.0})", fx, fy);
                                }
                                _ => {}
                            }
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        if mouse_event.kind == MouseEventKind::Down(event::MouseButton::Left) {
                            // Map terminal click to world coords roughly
                            // We don't have exact canvas bounds here, so we estimate based on terminal size
                            // This is a rough approximation for interactivity
                            // For a robust implementation, we'd need to track the Canvas rect.
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let fx = rng.gen_range(100.0..WORLD_SIZE - 100.0);
                            let fy = rng.gen_range(100.0..WORLD_SIZE - 100.0);
                            let fr = rng.gen_range(30.0..70.0);
                            self.world.add_firewall((fx, fy), fr);
                            self.status_msg = format!("Firewall deployed via Mouse.");
                        }
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.update_hologram();
                last_tick = std::time::Instant::now();
            }
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(85),
                    Constraint::Percentage(15),
                ]
                .as_ref(),
            )
            .split(f.area());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);

        // Left Panel: Spatial Domain (Simulation Grid + Swarm)
        let canvas_spatial = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Spatial Domain (DDoS Swarm) "),
            )
            .marker(ratatui::symbols::Marker::Block)
            .x_bounds([0.0, WORLD_SIZE])
            .y_bounds([0.0, WORLD_SIZE])
            .paint(|ctx| {
                // Draw Target
                ctx.draw(&Points {
                    coords: &[(self.world.target.0, self.world.target.1)],
                    color: Color::Red,
                });

                // Draw Firewalls
                for (fw_pos, fw_radius) in &self.world.firewalls {
                    // Approximate circles with points
                    let steps = 16;
                    let mut fw_pts = Vec::new();
                    for i in 0..steps {
                        let angle = (i as f64) * std::f64::consts::TAU / (steps as f64);
                        let px = fw_pos.0 + (*fw_radius as f64) * angle.cos();
                        let py = fw_pos.1 + (*fw_radius as f64) * angle.sin();
                        fw_pts.push((px, py));
                    }
                    ctx.draw(&Points {
                        coords: &fw_pts,
                        color: Color::Magenta,
                    });
                }

                // Draw Agents (sample a subset for performance)
                let sample_rate = 100; // Only draw 1% of agents
                let mut agent_pts = Vec::with_capacity(self.world.agents.len() / sample_rate);
                for (i, agent) in self.world.agents.iter().enumerate() {
                    if i % sample_rate == 0 {
                        agent_pts.push((agent.pos.0, agent.pos.1));
                    }
                }
                ctx.draw(&Points {
                    coords: &agent_pts,
                    color: Color::Cyan,
                });
            });
        f.render_widget(canvas_spatial, main_chunks[0]);

        // Right Panel: Reconstruction (Frequency Domain / Hologram)
        let recon_mag = &self.reconstruction_data;
        let max_recon = recon_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_recon = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Spectral Signature (Hologram Recon) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let mut pts_high = vec![];
                let mut pts_med = vec![];
                let mut pts_low = vec![];

                if max_recon > 0.0 {
                    for y in 0..self.hologram.height {
                        for x in 0..self.hologram.width {
                            let idx = y * self.hologram.width + x;
                            let val = recon_mag[idx] / max_recon;
                            if val > 0.6 {
                                pts_high.push((x as f64, (self.hologram.height - 1 - y) as f64));
                            } else if val > 0.3 {
                                pts_med.push((x as f64, (self.hologram.height - 1 - y) as f64));
                            } else if val > 0.1 {
                                pts_low.push((x as f64, (self.hologram.height - 1 - y) as f64));
                            }
                        }
                    }
                }

                ctx.draw(&Points {
                    coords: &pts_low,
                    color: Color::DarkGray,
                });
                ctx.draw(&Points {
                    coords: &pts_med,
                    color: Color::Gray,
                });
                ctx.draw(&Points {
                    coords: &pts_high,
                    color: Color::White,
                });
            });

        f.render_widget(canvas_recon, main_chunks[1]);

        // Status / Controls
        let health_pct = (self.world.server_health / self.world.max_health).clamp(0.0, 1.0);
        let status = Paragraph::new(format!(
            "Server Health: {:.1}% | Angle: ({}, {}) | {}",
            health_pct * 100.0, self.reconstruction_angle_x, self.reconstruction_angle_y, self.status_msg
        ))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(status, chunks[1]);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
