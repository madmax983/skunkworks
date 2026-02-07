use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
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
use std::io::Stdout;
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod lattice;
mod render;
mod audio;
mod brain;

use lattice::LatticeType;
use render::Camera;
use brain::Brain;

struct App {
    brain: Brain,
    camera: Camera,
    running: bool,
    lattice_type: LatticeType,
    lattice_size: usize,
}

impl App {
    fn new() -> Result<Self> {
        let size = 2;
        let l_type = LatticeType::SimpleCubic;
        let brain = Brain::new(l_type, size)?;

        Ok(Self {
            brain,
            camera: Camera::new(),
            running: true,
            lattice_type: l_type,
            lattice_size: size,
        })
    }

    fn change_lattice(&mut self) {
        self.lattice_type = match self.lattice_type {
            LatticeType::SimpleCubic => LatticeType::BodyCenteredCubic,
            LatticeType::BodyCenteredCubic => LatticeType::FaceCenteredCubic,
            LatticeType::FaceCenteredCubic => LatticeType::SimpleCubic,
        };
        // Re-initialize brain with new lattice
        // But brain.change_lattice might fail if we had it return Result, but I made it void.
        // I need to implement `change_lattice` in `Brain` properly. I did.
        // It's void there.
        // Wait, `Brain::new` returns Result because of Audio.
        // `change_lattice` reuses audio engine.
        // But `Brain` struct has `lattice` field.
        // We need to update it.
        // In `brain.rs`, `change_lattice` creates new lattice.

        // Wait, `Brain::change_lattice` signature in `brain.rs` was:
        // pub fn change_lattice(&mut self, l_type: LatticeType, size: usize)

        self.brain.change_lattice(self.lattice_type, self.lattice_size);
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

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

            // Update brain state (receive snapshots)
            self.brain.update();

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
            KeyCode::Char('i') => {
                 // Inject current into random neuron
                 let idx = rand::random::<usize>() % self.brain.neurons.len();
                 self.brain.inject(idx, 20.0);
            }
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
            LatticePoint {
                x: f64,
                y: f64,
                z: f64,
                v: f32, // Voltage for color
            },
            Edge {
                x1: f64,
                y1: f64,
                x2: f64,
                y2: f64,
                z: f64,
                weight: f32,
            },
        }

        let mut items = Vec::new();

        // Project Lattice Points (Neurons)
        for (i, p) in self.brain.lattice.points.iter().enumerate() {
            if let Some((x, y, z)) = self.camera.project(p, width, height) {
                let v = if i < self.brain.neurons.len() {
                    self.brain.neurons[i].v
                } else {
                    -65.0
                };
                items.push((z, Item::LatticePoint { x, y, z, v }));
            }
        }

        // Project Edges (Synapses)
        // This might be too many edges.
        // Only draw if weight > threshold or only nearest neighbors?
        // `brain.adj` has adjacency.
        // Let's draw a subset or all? 1000 neurons * 6 neighbors = 6000 lines.
        // Might be slow in TUI.
        // Limit to first 2 neighbors?
        for (u, neighbors) in self.brain.adj.iter().enumerate() {
            let u_pos = &self.brain.lattice.points[u];
             if let Some((x1, y1, z1)) = self.camera.project(u_pos, width, height) {
                for (j_idx, &v) in neighbors.iter().enumerate() {
                    if u < v { // Undirected drawing
                        let v_pos = &self.brain.lattice.points[v];
                        if let Some((x2, y2, z2)) = self.camera.project(v_pos, width, height) {
                            let avg_z = (z1 + z2) / 2.0;
                            items.push((
                                avg_z,
                                Item::Edge {
                                    x1,
                                    y1,
                                    x2,
                                    y2,
                                    z: avg_z,
                                    weight: self.brain.weights[u][j_idx],
                                },
                            ));
                        }
                    }
                }
            }
        }

        // Sort Painter's Algorithm (Back to Front)
        items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Lattice Brain"),
            )
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(move |ctx| {
                for (_, item) in &items {
                    match item {
                        Item::LatticePoint { x, y, v, .. } => {
                            // Color based on voltage
                            // Resting -65, Spike 30.
                            // Map -70..30 to Blue..Red
                            let t = ((v + 70.0) / 100.0).clamp(0.0, 1.0);
                            let color = if t > 0.8 {
                                Color::Red // Spike
                            } else if t > 0.6 {
                                Color::Yellow
                            } else {
                                Color::Blue // Resting
                            };

                            ctx.print(
                                *x,
                                *y,
                                Span::styled("●", Style::default().fg(color)),
                            );
                        }
                        Item::Edge {
                            x1,
                            y1,
                            x2,
                            y2,
                            weight,
                            ..
                        } => {
                            let color = if *weight > 0.5 {
                                Color::Green
                            } else {
                                Color::DarkGray
                            };
                            ctx.draw(&Line {
                                x1: *x1,
                                y1: *y1,
                                x2: *x2,
                                y2: *y2,
                                color,
                            });
                        }
                    }
                }
            });

        f.render_widget(canvas, chunks[0]);

        let status = format!(
            "Lattice: {:?} | Neurons: {} | [Space]: Switch Lattice, [I]: Inject Current, [Arrows]: Rotate, [+/-]: Zoom, [Q]: Quit",
            self.lattice_type, self.brain.neurons.len()
        );

        f.render_widget(
            Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
            chunks[1],
        );
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let app = App::new();
    match app {
        Ok(app) => {
            if let Err(e) = app.run(&mut tui.terminal) {
                eprintln!("Error: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to init app: {:?}", e);
        }
    }

    tui.exit()?;
    Ok(())
}
