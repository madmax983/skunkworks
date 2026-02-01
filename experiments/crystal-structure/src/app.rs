use crate::lattice::Lattice;
use crate::math;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders,
    },
    DefaultTerminal, Frame,
};
use nalgebra::{Point3, Rotation3, Vector3};
use std::time::Duration;

pub struct App {
    lattice: Lattice,
    rotation: Vector3<f64>, // x, y, z rotation angles
    miller: Vector3<f64>,   // h, k, l
    plane_d: f64,
    show_plane: bool,
    exit: bool,
}

impl App {
    pub fn new(lattice: Lattice) -> Self {
        Self {
            lattice,
            rotation: Vector3::new(0.5, 0.5, 0.0),
            miller: Vector3::new(1.0, 0.0, 0.0), // Default 100 plane
            plane_d: 0.0,
            show_plane: false, // Start with full crystal visible
            exit: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let rotation = Rotation3::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);

        // Prepare atoms for rendering
        // 1. Rotate
        // 2. Filter (if plane is active)
        // 3. Project

        let center_x = 0.0;
        let center_y = 0.0;
        let scale = 40.0; // Zoom factor

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Crystal Structure"))
            .x_bounds([-100.0, 100.0])
            .y_bounds([-100.0, 100.0])
            .marker(Marker::Braille)
            .paint(|ctx| {
                // Draw Miller Plane if active
                if self.show_plane {
                   // Drawing the plane is hard in 2D canvas without polygon filling.
                   // Maybe just draw a line representing the normal?
                   // Or just let the filtering speak for itself.
                }

                for atom in &self.lattice.atoms {
                    let rotated_pos = math::rotate_point(atom.position, rotation);

                    if self.show_plane {
                        let dist = math::distance_to_plane(atom.position, self.miller.x, self.miller.y, self.miller.z, self.plane_d);
                        if dist > 0.5 { // Thickness
                            continue;
                        }
                    }

                    let (x, y) = math::project_point(rotated_pos, scale, center_x, center_y);

                    // Canvas bounds are [-100, 100]. Check if point is inside.
                    if x >= -100.0 && x <= 100.0 && y >= -100.0 && y <= 100.0 {
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: atom.color,
                        });
                    }
                }

                // Draw axes
                let origin = Point3::origin();
                let axes = [
                    (Point3::new(2.0, 0.0, 0.0), Color::Red),
                    (Point3::new(0.0, 2.0, 0.0), Color::Green),
                    (Point3::new(0.0, 0.0, 2.0), Color::Blue),
                ];

                let (ox, oy) = math::project_point(math::rotate_point(origin, rotation), scale, center_x, center_y);

                for (end_point, color) in axes {
                    let rotated_end = math::rotate_point(end_point, rotation);
                    let (ex, ey) = math::project_point(rotated_end, scale, center_x, center_y);
                    ctx.draw(&Line {
                        x1: ox,
                        y1: oy,
                        x2: ex,
                        y2: ey,
                        color,
                    });
                }

                // Info text
                let info = format!(
                    "Rot: ({:.1}, {:.1}, {:.1}) | Miller: ({:.0} {:.0} {:.0}) d={:.1} | Plane: {} | Files: {}",
                    self.rotation.x, self.rotation.y, self.rotation.z,
                    self.miller.x, self.miller.y, self.miller.z, self.plane_d,
                    if self.show_plane { "ON" } else { "OFF" },
                    self.lattice.atoms.len()
                );
                // Can't draw text easily on Canvas in ratatui without external widgets usually,
                // but Canvas paint has print? No, ctx.print.
                ctx.print(0.0, -90.0, Span::styled(info, Style::default().fg(Color::White)));
            });

        frame.render_widget(canvas, area);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        // Rotation
                        KeyCode::Left => self.rotation.y -= 0.1,
                        KeyCode::Right => self.rotation.y += 0.1,
                        KeyCode::Up => self.rotation.x -= 0.1,
                        KeyCode::Down => self.rotation.x += 0.1,
                        // Miller Indices
                        KeyCode::Char('h') => self.miller.x += 1.0,
                        KeyCode::Char('H') => self.miller.x -= 1.0,
                        KeyCode::Char('k') => self.miller.y += 1.0,
                        KeyCode::Char('K') => self.miller.y -= 1.0,
                        KeyCode::Char('l') => self.miller.z += 1.0,
                        KeyCode::Char('L') => self.miller.z -= 1.0,
                        // Plane Depth
                        KeyCode::Char(']') => self.plane_d += 0.2,
                        KeyCode::Char('[') => self.plane_d -= 0.2,
                        // Toggle Plane
                        KeyCode::Char('p') => self.show_plane = !self.show_plane,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
