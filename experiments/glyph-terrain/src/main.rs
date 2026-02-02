mod render;
mod sdf;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::Vec3;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame,
};
use rusttype::Font;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use tui_shared::Tui;

use render::{draw_terrain, Camera, Terrain};
use sdf::generate_glyph_sdf;

struct App {
    terrain: Terrain,
    camera: Camera,
    running: bool,
}

impl App {
    fn new() -> Result<Self> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets/Roboto-Regular.ttf");
        let font_data = std::fs::read(&d)
            .map_err(|e| anyhow::anyhow!("Failed to read font from {:?}: {}", d, e))?;
        let font = Font::try_from_vec(font_data)
            .ok_or_else(|| anyhow::anyhow!("Error constructing Font"))?;

        // Generate SDF for 'G' (Genesis)
        // High resolution for better terrain
        let sdf = generate_glyph_sdf(&font, 'G', 64);
        let terrain = Terrain::new(sdf);

        // Position camera to view the glyph
        // Glyph is generated at grid coordinates. Render maps grid Z to world -Z.
        // Grid width ~ 40-50 units (scale 0.5 -> 20-25 world units).

        let center_x = (terrain.sdf.width as f32 * terrain.scale) / 2.0;
        // let center_z = -(terrain.sdf.height as f32 * terrain.scale) / 2.0;

        let mut camera = Camera::new(Vec3::new(center_x, 20.0, 20.0));
        camera.yaw = -std::f32::consts::FRAC_PI_2; // Look -Z
        camera.pitch = -0.6; // Look down

        // Actually, render maps grid Z index to -Z world coord.
        // So glyph extends from Z=0 to Z=-Height*Scale.
        // Camera at Z=20 looking at Z=-20 is good.

        Ok(Self {
            terrain,
            camera,
            running: true,
        })
    }

    fn run(&mut self) -> Result<()> {
        let mut tui = Tui::init()?;
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

        while self.running {
            tui.terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => self.running = false,
                            KeyCode::Char('w') => self.move_camera(1.0, 0.0),
                            KeyCode::Char('s') => self.move_camera(-1.0, 0.0),
                            KeyCode::Char('a') => self.move_camera(0.0, -1.0),
                            KeyCode::Char('d') => self.move_camera(0.0, 1.0),
                            KeyCode::Char('e') => self.camera.pos.y += 1.0,
                            KeyCode::Char('c') => self.camera.pos.y -= 1.0,
                            KeyCode::Left => self.camera.yaw += 0.1,
                            KeyCode::Right => self.camera.yaw -= 0.1,
                            KeyCode::Up => self.camera.pitch += 0.1,
                            KeyCode::Down => self.camera.pitch -= 0.1,
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn move_camera(&mut self, fwd: f32, right: f32) {
        let f = self.camera.forward() * fwd;
        let r = self.camera.right() * right;
        // Move flat on XZ plane if desired, or free fly?
        // Free fly is easier.
        self.camera.pos += (f + r) * 1.0;
    }

    fn ui(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Glyph Terrain - 'G'"),
            )
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(|ctx| {
                // Pass a rough screen size aspect to the renderer?
                // Canvas resolution is high, but coordinate system is abstract.
                // We map NDC to 0..100.
                draw_terrain(ctx, &self.terrain, &self.camera, (100.0, 100.0));
            });

        frame.render_widget(canvas, chunks[0]);

        let info = format!(
            "Pos: {:.1}, {:.1}, {:.1} | Yaw: {:.2} | Pitch: {:.2}\nWASD: Move | Arrows: Rotate | E/C: Up/Down | Q: Quit",
            self.camera.pos.x, self.camera.pos.y, self.camera.pos.z, self.camera.yaw, self.camera.pitch
        );

        frame.render_widget(
            Paragraph::new(info).block(Block::default().borders(Borders::ALL)),
            chunks[1],
        );
    }
}

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}
