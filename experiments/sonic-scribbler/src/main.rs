use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{canvas::{Canvas, Line, Context}, Block, Borders, Paragraph},
    Frame,
};
use std::{time::{Duration, Instant}, path::PathBuf};
use tui_shared::Tui;
use glam::Vec2;

mod font;
mod transform;

use font::{FontLoader, PathCommand};
use transform::{distort, flatten, AudioState};

struct App {
    audio: AudioState,
    text: String,
    running: bool,
    font_data: Vec<u8>,
}

impl App {
    fn new() -> Result<Self> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets/Roboto-Regular.ttf");
        let font_data = std::fs::read(&d)?;

        Ok(Self {
            font_data,
            audio: AudioState {
                amplitude: 1.0,
                frequency: 1.0,
                phase: 0.0,
            },
            text: "GENESIS".to_string(),
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
                    match key.code {
                        KeyCode::Char('q') => self.running = false,
                        KeyCode::Up => self.audio.amplitude += 0.2,
                        KeyCode::Down => self.audio.amplitude -= 0.2,
                        KeyCode::Right => self.audio.frequency += 0.2,
                        KeyCode::Left => self.audio.frequency -= 0.2,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                // Animate phase
                self.audio.phase += 0.1;
                // Add some random fluctuation to amplitude for "beat" effect
                // self.audio.amplitude = (self.audio.amplitude * 0.9) + (fastrand::f32() * 0.2);
                // No, manual control is better for testing.
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Sonic Scribbler"))
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, 12000.0])
            .y_bounds([-1000.0, 3000.0])
            .paint(|ctx| {
                self.draw_text(ctx);
            });

        frame.render_widget(canvas, chunks[0]);

        let info = format!(
            "Amp: {:.2} | Freq: {:.2} | Phase: {:.2} | Controls: Arrows (+/-), Q (Quit)",
            self.audio.amplitude, self.audio.frequency, self.audio.phase
        );
        frame.render_widget(
            Paragraph::new(info).block(Block::default().borders(Borders::ALL)),
            chunks[1]
        );
    }

    fn draw_text(&self, ctx: &mut Context) {
        if let Some(loader) = FontLoader::new(&self.font_data) {
             let mut cursor_x = 500.0;
             let baseline_y = 500.0;

             for c in self.text.chars() {
                 let cmds = loader.get_glyph_path(c);

                 // Shift to position
                 let shifted_cmds: Vec<PathCommand> = cmds.iter().map(|cmd| {
                     shift_cmd(cmd, Vec2::new(cursor_x, baseline_y))
                 }).collect();

                 // Distort
                 let distorted = distort(&shifted_cmds, &self.audio);

                 // Flatten
                 let lines = flatten(&distorted);

                 for (p1, p2) in lines {
                     ctx.draw(&Line {
                         x1: p1.x as f64,
                         y1: p1.y as f64,
                         x2: p2.x as f64,
                         y2: p2.y as f64,
                         color: Color::Cyan,
                     });
                 }

                 cursor_x += loader.get_glyph_advance(c);
             }
        }
    }
}

fn shift_cmd(cmd: &PathCommand, offset: Vec2) -> PathCommand {
    match cmd {
        PathCommand::MoveTo(p) => PathCommand::MoveTo(*p + offset),
        PathCommand::LineTo(p) => PathCommand::LineTo(*p + offset),
        PathCommand::QuadTo(c, e) => PathCommand::QuadTo(*c + offset, *e + offset),
        PathCommand::CurveTo(c1, c2, e) => PathCommand::CurveTo(*c1 + offset, *c2 + offset, *e + offset),
        PathCommand::Close => PathCommand::Close,
    }
}

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}
