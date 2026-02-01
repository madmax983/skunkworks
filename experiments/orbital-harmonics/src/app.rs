use crate::physics::{Universe, Body, AudioEvent, G};
use crate::audio::{AudioEngine, map_to_scale};
use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph, canvas::{Canvas, Context, Circle, Line as CanvasLine}},
    style::{Color},
    symbols::Marker,
};
use crossterm::event::{KeyCode, KeyEvent};

pub struct App {
    pub universe: Universe,
    pub audio: AudioEngine,
    pub paused: bool,
    pub zoom: f64,
    pub speed: f64,
}

impl App {
    pub fn new() -> Self {
        let mut universe = Universe::new();
        // Create Solar System
        universe.add_body(Body::new(0.0, 0.0, 5000.0, 5.0, Color::Yellow, "Sun".to_string()));

        // Add some planets
        // Earth-ish
        // V = sqrt(G*M/R)
        let v_earth = (G * 5000.0 / 100.0f64).sqrt();
        universe.add_body(Body::new(100.0, 0.0, 10.0, 2.0, Color::Blue, "Earth".to_string())
            .with_velocity(0.0, v_earth));

        // Mars-ish
        let v_mars = (G * 5000.0 / 150.0f64).sqrt();
        universe.add_body(Body::new(150.0, 0.0, 8.0, 1.5, Color::Red, "Mars".to_string())
             .with_velocity(0.0, v_mars));

        Self {
            universe,
            audio: AudioEngine::new(),
            paused: false,
            zoom: 1.0,
            speed: 1.0,
        }
    }

    pub fn on_tick(&mut self) {
        if !self.paused {
            let dt = 0.1 * self.speed;
            self.universe.tick(dt);

            // Handle audio events
            for event in &self.universe.events {
                match event {
                    AudioEvent::OrbitComplete { body_index: _, radius } => {
                        let freq = map_to_scale(*radius);
                        self.audio.play_freq(freq);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> bool {
         match key.code {
            KeyCode::Char('q') => return true, // exit
            KeyCode::Char(' ') => self.paused = !self.paused,
            KeyCode::Char('+') | KeyCode::Char('=') => self.speed *= 1.2,
            KeyCode::Char('-') => self.speed /= 1.2,
            KeyCode::Char('z') => self.zoom *= 1.1,
            KeyCode::Char('x') => self.zoom /= 1.1,
            KeyCode::Char('r') => {
                 // Reset
                 *self = Self::new();
            },
            KeyCode::Char('a') => {
                // Add random body
                 use rand::Rng;
                 let mut rng = rand::thread_rng();
                 let dist = rng.gen_range(50.0..250.0);
                 let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
                 let x = dist * angle.cos();
                 let y = dist * angle.sin();
                 let v_mag = (G * 5000.0 / dist).sqrt(); // Approx circ
                 let vx = -v_mag * angle.sin();
                 let vy = v_mag * angle.cos();

                 self.universe.add_body(Body::new(x, y, 5.0, 1.0, Color::Green, "Random".to_string())
                    .with_velocity(vx, vy));
            }
            _ => {}
        }
        false
    }

    pub fn draw(&self, f: &mut Frame) {
        // Layout
        // Canvas takes most space
        // Info at bottom
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(f.area());

        let canvas_area = chunks[0];

        let x_bound = 200.0 / self.zoom;
        let y_bound = 100.0 / self.zoom;

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Orbital Harmonics"))
            .marker(Marker::Braille)
            .x_bounds([-x_bound, x_bound])
            .y_bounds([-y_bound, y_bound])
            .paint(|ctx: &mut Context| {
                // Draw Trails
                for body in &self.universe.bodies {
                     for point in &body.trail {
                         ctx.draw(&CanvasLine {
                             x1: point.0,
                             y1: point.1,
                             x2: point.0,
                             y2: point.1,
                             color: Color::DarkGray,
                         });
                     }
                }

                // Draw Bodies
                for body in &self.universe.bodies {
                    ctx.draw(&Circle {
                        x: body.pos.x,
                        y: body.pos.y,
                        radius: body.radius,
                        color: body.color,
                    });
                }

                // Draw Star (special case 0) - ensure it's drawn last (on top)? Canvas draws in order.
                 if let Some(sun) = self.universe.bodies.first() {
                    ctx.draw(&Circle {
                        x: sun.pos.x,
                        y: sun.pos.y,
                        radius: sun.radius,
                        color: Color::Yellow,
                    });
                 }
            });

        f.render_widget(canvas, canvas_area);

        // Info
        let info = format!(
            "Zoom: {:.2} | Speed: {:.2} | Bodies: {} | [Space] Pause | [+/-] Speed | [z/x] Zoom | [a] Add | [r] Reset | [q] Quit",
            self.zoom, self.speed, self.universe.bodies.len()
        );
        let p = Paragraph::new(info).block(Block::default().borders(Borders::ALL));
        f.render_widget(p, chunks[1]);
    }
}
