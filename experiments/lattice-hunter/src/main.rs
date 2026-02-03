use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

mod lattice;
mod network;
mod render;

use lattice::{Lattice, LatticeType};
use network::Network;
use render::Camera;

struct App {
    lattice: Lattice,
    network: Network,
    camera: Camera,
    running: bool,
    optimizing: bool,
    lattice_type: LatticeType,
    lattice_size: usize,
    temp: f64,
    iteration: usize,
}

impl App {
    fn new() -> Self {
        let size = 2;
        let l_type = LatticeType::SimpleCubic;
        let lattice = Lattice::new(l_type, size, 4.0);
        let mut rng = rand::thread_rng();
        let mut network = Network::new_random(30, 0.3, &mut rng);
        network.initialize_positions(&lattice, &mut rng);

        Self {
            lattice,
            network,
            camera: Camera::new(),
            running: true,
            optimizing: false,
            lattice_type: l_type,
            lattice_size: size,
            temp: 10.0,
            iteration: 0,
        }
    }

    fn reset_network(&mut self) {
        let mut rng = rand::thread_rng();
        self.network = Network::new_random(30, 0.3, &mut rng);
        self.network.initialize_positions(&self.lattice, &mut rng);
        self.temp = 10.0;
        self.iteration = 0;
    }

    fn change_lattice(&mut self) {
        self.lattice_type = match self.lattice_type {
            LatticeType::SimpleCubic => LatticeType::BodyCenteredCubic,
            LatticeType::BodyCenteredCubic => LatticeType::FaceCenteredCubic,
            LatticeType::FaceCenteredCubic => LatticeType::SimpleCubic,
        };
        self.lattice = Lattice::new(self.lattice_type, self.lattice_size, 4.0);

        // Re-initialize network positions to fit new lattice
        let mut rng = rand::thread_rng();
        self.network.initialize_positions(&self.lattice, &mut rng);
        self.temp = 10.0;
        self.iteration = 0;
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut rng = rand::thread_rng();

        while self.running {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                }
            }

            if self.optimizing {
                for _ in 0..10 {
                    if self.network.optimize_step(&self.lattice, self.temp, &mut rng) {
                        // Accepted
                    }
                    self.iteration += 1;
                }
                self.temp *= 0.995; // Cool down
                if self.temp < 0.001 {
                    self.optimizing = false;
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Left => self.camera.rotate_y(-0.1),
            KeyCode::Right => self.camera.rotate_y(0.1),
            KeyCode::Up => self.camera.rotate_x(-0.1),
            KeyCode::Down => self.camera.rotate_x(0.1),
            KeyCode::Char('+') => self.camera.zoom *= 1.1,
            KeyCode::Char('-') => self.camera.zoom /= 1.1,
            KeyCode::Char(' ') => self.change_lattice(),
            KeyCode::Char('r') => self.reset_network(),
            KeyCode::Char('o') => {
                self.optimizing = !self.optimizing;
                if self.optimizing { self.temp = 10.0; } // Reset temp on start
            },
            _ => {}
        }
    }

    fn ui(&self, f: &mut Frame) {
        let area = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let width = chunks[0].width as f64;
        let height = chunks[0].height as f64;

        // Render Lists
        enum Item {
            LatticePoint { x: f64, y: f64, z: f64 },
            Node { x: f64, y: f64, z: f64, color: Color },
            Edge { x1: f64, y1: f64, x2: f64, y2: f64, z: f64, color: Color },
        }

        let mut items = Vec::new();

        // Project Lattice (Ghost)
        for p in &self.lattice.points {
            if let Some((x, y, z)) = self.camera.project(p, width, height) {
                items.push((z, Item::LatticePoint { x, y, z }));
            }
        }

        // Project Network Nodes
        // (We don't really need to draw nodes if they overlap lattice points, but let's draw them brighter)
        for &idx in &self.network.node_positions {
            let p = &self.lattice.points[idx];
            if let Some((x, y, z)) = self.camera.project(p, width, height) {
                 // Z-bias slightly towards camera to draw on top of lattice points
                items.push((z + 0.1, Item::Node { x, y, z: z+0.1, color: Color::Cyan }));
            }
        }

        // Project Edges
        for (u, neighbors) in self.network.adj.iter().enumerate() {
            let u_pos = &self.lattice.points[self.network.node_positions[u]];
            if let Some((x1, y1, z1)) = self.camera.project(u_pos, width, height) {
                for &v in neighbors {
                    if u < v {
                        let v_pos = &self.lattice.points[self.network.node_positions[v]];
                        if let Some((x2, y2, z2)) = self.camera.project(v_pos, width, height) {
                            let avg_z = (z1 + z2) / 2.0;
                            items.push((avg_z, Item::Edge { x1, y1, x2, y2, z: avg_z, color: Color::Yellow }));
                        }
                    }
                }
            }
        }

        // Sort Painter's Algorithm (Back to Front)
        items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Lattice Hunter"))
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(move |ctx| {
                for (_, item) in &items {
                    match item {
                        Item::LatticePoint { x, y, .. } => {
                            ctx.print(*x, *y, Span::styled("·", Style::default().fg(Color::DarkGray)));
                        }
                        Item::Node { x, y, color, .. } => {
                             ctx.print(*x, *y, Span::styled("●", Style::default().fg(*color)));
                        }
                        Item::Edge { x1, y1, x2, y2, color, .. } => {
                            ctx.draw(&Line {
                                x1: *x1, y1: *y1, x2: *x2, y2: *y2, color: *color,
                            });
                        }
                    }
                }
            });

        f.render_widget(canvas, chunks[0]);

        let status = format!(
            "Lattice: {:?} | Temp: {:.3} | Iter: {} | Energy: {:.1} | [Arrows]: Rotate, [Space]: Lattice, [O]: Optimize, [R]: Reset, [Q]: Quit",
            self.lattice_type, self.temp, self.iteration, self.network.total_energy(&self.lattice)
        );

        f.render_widget(
            Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
            chunks[1]
        );
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}
