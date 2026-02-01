#![allow(clippy::collapsible_if)]

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    time::{Duration, Instant},
};

use crate::swarm::Swarm;

pub struct App {
    pub swarm: Swarm,
    pub is_running: bool,
    pub tick_rate: Duration,
    pub last_tick: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        // High resolution swarm
        // 160 width, 80 height
        Self {
            swarm: Swarm::new(160, 80),
            is_running: true,
            tick_rate: Duration::from_millis(33), // ~30 FPS
            last_tick: Instant::now(),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.is_running {
            terminal.draw(|f| self.draw(f))?;

            let timeout = self.tick_rate
                .checked_sub(self.last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.is_running = false,
                            KeyCode::Up => {
                                self.swarm.coupling_strength = (self.swarm.coupling_strength + 0.001).min(1.0);
                            }
                            KeyCode::Down => {
                                self.swarm.coupling_strength = (self.swarm.coupling_strength - 0.001).max(0.0);
                            }
                             KeyCode::Right => {
                                // Increase natural freq variance? Or just dt?
                                // Let's just adjust coupling for now.
                            }
                            _ => {}
                        }
                    }
                }
            }

            if self.last_tick.elapsed() >= self.tick_rate {
                self.swarm.update();
                self.last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let grid_area = chunks[0];
        let status_area = chunks[1];

        // Draw Swarm
        // We use Canvas to render points.
        // We categorize fireflies by phase color to reduce draw calls

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Swarm"))
            .x_bounds([0.0, self.swarm.width as f64])
            .y_bounds([0.0, self.swarm.height as f64])
            .marker(ratatui::symbols::Marker::Braille) // Braille gives 2x4 resolution!
            .paint(|ctx| {
                // To optimize, we collect points for different colors
                let mut flash_points = Vec::new();
                let mut high_phase_points = Vec::new();
                let mut mid_phase_points = Vec::new();

                for (i, firefly) in self.swarm.fireflies.iter().enumerate() {
                    let x = (i % self.swarm.width) as f64;
                    // Y needs to be inverted for Canvas? Usually 0,0 is bottom-left.
                    // But in our vector 0 is top-left.
                    let y = (self.swarm.height - 1 - (i / self.swarm.width)) as f64;

                    if firefly.flashed || firefly.phase > 0.95 {
                        flash_points.push((x, y));
                    } else if firefly.phase > 0.7 {
                        high_phase_points.push((x, y));
                    } else if firefly.phase > 0.4 {
                        mid_phase_points.push((x, y));
                    }
                }

                ctx.draw(&Points {
                    coords: &mid_phase_points,
                    color: Color::DarkGray,
                });
                ctx.draw(&Points {
                    coords: &high_phase_points,
                    color: Color::Gray,
                });
                ctx.draw(&Points {
                    coords: &flash_points,
                    color: Color::Yellow,
                });
            });

        frame.render_widget(canvas, grid_area);

        // Draw Status
        let sync_index = self.swarm.synchronization_index();
        let status_text = vec![
            Line::from(vec![
                Span::styled("Coupling (K): ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{:.3}", self.swarm.coupling_strength)),
                Span::raw(" | "),
                Span::styled("Sync (R): ", Style::default().fg(Color::Magenta)),
                Span::raw(format!("{:.3}", sync_index)),
                Span::raw(" | "),
                Span::styled("Controls: ", Style::default().fg(Color::Green)),
                Span::raw("Up/Down (K), Q (Quit)"),
            ]),
        ];

        let status_block = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Status"));

        frame.render_widget(status_block, status_area);
    }
}
