use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use nalgebra::{Point3, Rotation3, Vector3};
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
use std::fs;
use std::io::Stdout;
use std::path::Path;
use std::time::{Duration, Instant};
use tui_shared::Tui;

// --- Data Structures ---

#[derive(Debug, Clone)]
struct Atom {
    position: Point3<f64>,
    color: Color,
    size: f64,
    name: String,
}

struct Bond {
    start_idx: usize,
    end_idx: usize,
    color: Color,
}

struct Crystal {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

impl Crystal {
    fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: Vec::new(),
        }
    }
}

// --- Scanner / Layout ---

fn build_crystal(path: &str) -> Result<Crystal> {
    let mut crystal = Crystal::new();
    let root_path = Path::new(path);

    // Add root atom
    crystal.atoms.push(Atom {
        position: Point3::new(0.0, 0.0, 0.0),
        color: Color::White,
        size: 1.0,
        name: root_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    });

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((root_path.to_path_buf(), 0usize, 10.0f64)); // path, parent_idx, scale

    // Breadth-first layout
    while let Some((path, parent_idx, scale)) = queue.pop_front() {
        if !path.is_dir() {
            continue;
        }

        let entries = match fs::read_dir(&path) {
            Ok(e) => e,
            Err(_) => continue,
        };

        let mut children = Vec::new();
        for entry in entries.flatten() {
            children.push(entry.path());
        }
        children.sort();

        let n = children.len();
        if n == 0 {
            continue;
        }

        let phi = std::f64::consts::PI * (3.0 - 5.0f64.sqrt());
        let parent_pos = crystal.atoms[parent_idx].position;

        for (i, child_path) in children.iter().enumerate() {
            let y = 1.0 - (i as f64 / (n as f64 - 1.0).max(1.0)) * 2.0;
            let radius = (1.0 - y * y).sqrt();
            let theta = phi * i as f64;

            let x = theta.cos() * radius;
            let z = theta.sin() * radius;

            let dist = scale.max(2.0);
            let offset = Vector3::new(x, y, z) * dist;
            let pos = parent_pos + offset;

            let is_dir = child_path.is_dir();
            let name = child_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let color = if is_dir {
                Color::Cyan
            } else if name.ends_with(".rs") {
                Color::Red
            } else if name.ends_with(".toml") {
                Color::Yellow
            } else if name.ends_with(".md") {
                Color::Green
            } else {
                Color::Gray
            };

            let atom_idx = crystal.atoms.len();
            crystal.atoms.push(Atom {
                position: pos,
                color,
                size: if is_dir { 0.4 } else { 0.2 },
                name,
            });

            crystal.bonds.push(Bond {
                start_idx: parent_idx,
                end_idx: atom_idx,
                color: Color::DarkGray,
            });

            if is_dir {
                queue.push_back((child_path.clone(), atom_idx, scale * 0.5));
            }
        }
    }

    Ok(crystal)
}

// --- Camera & Projection ---

#[allow(dead_code)]
struct Camera {
    position: Point3<f64>,
    target: Point3<f64>,
    up: Vector3<f64>,
    fov: f64,
    aspect: f64,
    rotation: Rotation3<f64>,
    zoom: f64,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, -100.0),
            target: Point3::origin(),
            up: Vector3::y(),
            fov: 60.0f64.to_radians(),
            aspect: 2.0, // Terminal characters are roughly 1:2
            rotation: Rotation3::identity(),
            zoom: 1.0,
        }
    }

    fn project(&self, point: &Point3<f64>, width: f64, height: f64) -> Option<(f64, f64, f64)> {
        // 1. Rotate
        let rotated = self.rotation * (point - self.target.coords) + self.target.coords;

        // 2. Translate to camera view (simplified: assume camera is at -Z looking at +Z)
        // Actually, let's just use the rotation to spin the world, and keep camera fixed.
        // World -> Camera
        let v = rotated - self.position.coords; // Vector from camera to point

        // 3. Perspective divide
        // If z is positive (in front of camera, assuming camera looks down +Z and is at negative Z)
        let z = v.z;
        if z <= 0.1 {
            return None; // Behind camera or clipping plane
        }

        let scale = self.zoom * 100.0; // Base scale factor

        let x = v.x / z * scale * self.aspect;
        let y = -v.y / z * scale; // Flip Y for screen coords

        // Center on screen
        let screen_x = x + width / 2.0;
        let screen_y = y + height / 2.0;

        Some((screen_x, screen_y, z))
    }
}

// --- App ---

struct App {
    crystal: Crystal,
    camera: Camera,
    slice_z: f64,
    slice_thickness: f64,
    tomography_enabled: bool,
    running: bool,
}

impl App {
    fn new(path: String) -> Result<Self> {
        let crystal = build_crystal(&path)?;
        Ok(Self {
            crystal,
            camera: Camera::new(),
            slice_z: 0.0,
            slice_thickness: 10.0,
            tomography_enabled: false,
            running: true,
        })
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
            KeyCode::Left => {
                let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), -0.1);
                self.camera.rotation = rot * self.camera.rotation;
            }
            KeyCode::Right => {
                let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), 0.1);
                self.camera.rotation = rot * self.camera.rotation;
            }
            KeyCode::Up => {
                let rot = Rotation3::from_axis_angle(&Vector3::x_axis(), -0.1);
                self.camera.rotation = rot * self.camera.rotation;
            }
            KeyCode::Down => {
                let rot = Rotation3::from_axis_angle(&Vector3::x_axis(), 0.1);
                self.camera.rotation = rot * self.camera.rotation;
            }
            KeyCode::Char('+') | KeyCode::Char('=') => self.camera.zoom *= 1.1,
            KeyCode::Char('-') | KeyCode::Char('_') => self.camera.zoom /= 1.1,
            KeyCode::Char('t') => self.tomography_enabled = !self.tomography_enabled,
            KeyCode::Char('w') => self.slice_z += 2.0,
            KeyCode::Char('s') => self.slice_z -= 2.0,
            KeyCode::Char('a') => self.slice_thickness = (self.slice_thickness - 1.0).max(1.0),
            KeyCode::Char('d') => self.slice_thickness += 1.0,
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

        // Prepare points for rendering
        let mut draw_commands: Vec<(f64, RenderItem)> = Vec::new();

        // Filter and project atoms
        for atom in self.crystal.atoms.iter() {
            if self.tomography_enabled
                && (atom.position.z - self.slice_z).abs() > self.slice_thickness
            {
                continue;
            }

            if let Some((x, y, z)) = self.camera.project(&atom.position, width, height) {
                draw_commands.push((
                    z,
                    RenderItem::Atom {
                        x,
                        y,
                        color: atom.color,
                        label: atom.name.clone(),
                        size: atom.size,
                    },
                ));
            }
        }

        // Project bonds
        for bond in &self.crystal.bonds {
            let start = &self.crystal.atoms[bond.start_idx];
            let end = &self.crystal.atoms[bond.end_idx];

            if self.tomography_enabled
                && (start.position.z - self.slice_z).abs() > self.slice_thickness
                && (end.position.z - self.slice_z).abs() > self.slice_thickness
            {
                continue;
            }

            let p1 = self.camera.project(&start.position, width, height);
            let p2 = self.camera.project(&end.position, width, height);

            if let (Some((x1, y1, z1)), Some((x2, y2, z2))) = (p1, p2) {
                let avg_z = (z1 + z2) / 2.0;
                draw_commands.push((
                    avg_z,
                    RenderItem::Bond {
                        x1,
                        y1,
                        x2,
                        y2,
                        color: bond.color,
                    },
                ));
            }
        }

        // Sort by Z (Painter's Algorithm) - Furthest first (largest Z)
        draw_commands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Crystal Structure"),
            )
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(move |ctx| {
                for (_, item) in &draw_commands {
                    match item {
                        RenderItem::Atom {
                            x,
                            y,
                            color,
                            label: _,
                            size,
                        } => {
                            if *size > 0.3 {
                                ctx.print(*x, *y, Span::styled("●", Style::default().fg(*color)));
                            } else {
                                ctx.print(*x, *y, Span::styled("·", Style::default().fg(*color)));
                            }
                        }
                        RenderItem::Bond {
                            x1,
                            y1,
                            x2,
                            y2,
                            color,
                        } => {
                            ctx.draw(&Line {
                                x1: *x1,
                                y1: *y1,
                                x2: *x2,
                                y2: *y2,
                                color: *color,
                            });
                        }
                    }
                }
            });

        f.render_widget(canvas, chunks[0]);

        let status_text = format!(
            "Atoms: {} | Zoom: {:.1} | Tomography: {} (Z: {:.1}, Thk: {:.1}) | Arrows: Rotate | WASD: Slice | +/-: Zoom | Q: Quit",
            self.crystal.atoms.len(),
            self.camera.zoom,
            if self.tomography_enabled { "ON" } else { "OFF" },
            self.slice_z,
            self.slice_thickness
        );
        f.render_widget(
            Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
            chunks[1],
        );
    }
}

#[allow(dead_code)]
enum RenderItem {
    Atom {
        x: f64,
        y: f64,
        color: Color,
        label: String,
        size: f64,
    },
    Bond {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: Color,
    },
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    // Setup Terminal
    let mut tui = Tui::init()?;

    // Run App
    let app = App::new(path.to_string())?;
    let res = app.run(&mut tui.terminal);

    if let Err(e) = res {
        tui.exit()?;
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_point() {
        let camera = Camera::new();
        let p = Point3::new(0.0, 0.0, 0.0);

        let width = 100.0;
        let height = 50.0;
        let res = camera.project(&p, width, height);

        assert!(res.is_some());
        let (x, y, z) = res.unwrap();
        assert_eq!(x, 50.0);
        assert_eq!(y, 25.0);
        assert!(z > 0.0);
    }
}
