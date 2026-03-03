use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod model;
use model::Platter;

struct App {
    platter: Platter,
    head_pos: (usize, usize),
    running: bool,
    last_tick: Instant,
    time_scale: f32,
}

impl App {
    fn new() -> Self {
        // 32x32 grid = 1024 sectors
        Self {
            platter: Platter::new(32, 32),
            head_pos: (0, 0),
            running: true,
            last_tick: Instant::now(),
            time_scale: 1.0,
        }
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        while self.running {
            tui.terminal.draw(|f| self.ui(f))?;

            let timeout = Duration::from_millis(16); // ~60fps
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                        KeyCode::Left => {
                            if self.head_pos.0 > 0 {
                                self.head_pos.0 -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if self.head_pos.0 < self.platter.width() - 1 {
                                self.head_pos.0 += 1;
                            }
                        }
                        KeyCode::Up => {
                            if self.head_pos.1 > 0 {
                                self.head_pos.1 -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.head_pos.1 < self.platter.height() - 1 {
                                self.head_pos.1 += 1;
                            }
                        }
                        KeyCode::Char(' ') => {
                            if let Some(sector) = self
                                .platter
                                .get_sector_mut(self.head_pos.0, self.head_pos.1)
                            {
                                sector.scrub();
                            }
                        }
                        KeyCode::Char('w') => {
                            if let Some(sector) = self
                                .platter
                                .get_sector_mut(self.head_pos.0, self.head_pos.1)
                            {
                                let mut rng = rand::thread_rng();
                                let mut data = [0u8; 64];
                                rand::Rng::fill(&mut rng, &mut data);
                                sector.write(data);
                            }
                        }
                        KeyCode::Char('+') => {
                            self.time_scale *= 2.0;
                        }
                        KeyCode::Char('-') => {
                            self.time_scale *= 0.5;
                        }
                        _ => {}
                    }
                }
            }

            let now = Instant::now();
            let dt = now.duration_since(self.last_tick).as_secs_f32();
            self.last_tick = now;

            self.platter.update(dt * self.time_scale);
        }
        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Help
            ])
            .split(f.area());

        // Title
        let title_text = format!(
            "MAGNETRON DECAY | Time Scale: {:.1}x | Head: ({}, {})",
            self.time_scale, self.head_pos.0, self.head_pos.1
        );
        f.render_widget(
            Paragraph::new(title_text)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .style(Style::default().fg(Color::Cyan)),
            main_layout[0],
        );

        // Content
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60), // Platter
                Constraint::Percentage(40), // Inspector
            ])
            .split(main_layout[1]);

        // Platter Rendering
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Platter Map"))
            .x_bounds([0.0, 32.0])
            .y_bounds([0.0, 32.0])
            .paint(|ctx| {
                for y in 0..self.platter.height() {
                    for x in 0..self.platter.width() {
                        if let Some(sector) = self.platter.get_sector(x, y) {
                            // Color based on magnetization
                            let color = if sector.magnetization > 0.8 {
                                Color::Green
                            } else if sector.magnetization > 0.4 {
                                Color::Yellow
                            } else {
                                Color::Red
                            };

                            // If it's the head position, use a distinct color (Cyan)
                            // We can overwrite the block with Cyan
                            let (color, ch) = if x == self.head_pos.0 && y == self.head_pos.1 {
                                (Color::Cyan, "▣")
                            } else {
                                (color, "■")
                            };

                            // Draw using print for character-based grid
                            // Canvas Y is 0 at bottom
                            ctx.print(
                                x as f64,
                                (31 - y) as f64,
                                Span::styled(ch, Style::default().fg(color)),
                            );
                        }
                    }
                }
            });

        f.render_widget(canvas, content_layout[0]);

        // Inspector
        let mut lines = vec![];
        if let Some(sector) = self.platter.get_sector(self.head_pos.0, self.head_pos.1) {
            lines.push(Line::from(vec![
                Span::raw("Address: "),
                Span::styled(
                    format!("0x{:04X}", sector.address),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("Magnetization: "),
                Span::styled(
                    format!("{:.1}%", sector.magnetization * 100.0),
                    Style::default().fg(if sector.magnetization < 0.5 {
                        Color::Red
                    } else {
                        Color::Green
                    }),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("Coercivity: "),
                Span::styled(
                    format!("{:.1}%", sector.coercivity * 100.0),
                    Style::default().fg(Color::Blue),
                ),
            ]));
            lines.push(Line::from(""));

            // Hex Dump
            for chunk in sector.data.chunks(8) {
                let hex: String = chunk.iter().map(|b| format!("{:02X} ", b)).collect();
                let ascii: String = chunk
                    .iter()
                    .map(|b| {
                        if *b >= 32 && *b <= 126 {
                            *b as char
                        } else {
                            '.'
                        }
                    })
                    .collect();
                lines.push(Line::from(format!("{:24} | {}", hex, ascii)));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Original vs Corrupt:",
                Style::default().fg(Color::Gray),
            )));
            // Show Diff count
            let diff_count = sector
                .data
                .iter()
                .zip(sector.original_data.iter())
                .filter(|(a, b)| a != b)
                .count();
            if diff_count > 0 {
                lines.push(Line::from(Span::styled(
                    format!("{} bytes corrupted", diff_count),
                    Style::default().fg(Color::Red),
                )));
            } else {
                lines.push(Line::from(Span::styled(
                    "Integrity Verified",
                    Style::default().fg(Color::Green),
                )));
            }
        }

        let inspector = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sector Inspector"),
        );
        f.render_widget(inspector, content_layout[1]);

        // Help
        let help = Paragraph::new(
            "Arrows: Move | Space: Scrub/Refresh | W: Write Noise | +/-: Time Scale | Q: Quit",
        )
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(help, main_layout[2]);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = app.run(&mut tui);
    tui.exit()?;
    res
}
