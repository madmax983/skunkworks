use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::canvas::{Canvas, Context},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::time::{Duration, Instant};

use crate::algo::bjorklund;
use crate::audio::AudioHandle;
use crate::monitor::SystemMonitor;

pub struct App {
    monitor: SystemMonitor,
    audio: AudioHandle,
    patterns: Vec<Vec<bool>>,
    current_step: usize,
    running: bool,
    core_usages: Vec<f32>,
}

impl App {
    pub fn new(monitor: SystemMonitor, audio: AudioHandle) -> Self {
        Self {
            monitor,
            audio,
            patterns: vec![],
            current_step: 0,
            running: true,
            core_usages: vec![],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut last_update = Instant::now();
        let update_interval = Duration::from_millis(100); // Update CPU stats 10 times a sec

        while self.running {
            // Check for audio sync
            while let Ok(step) = self.audio.step_rx.try_recv() {
                self.current_step = step;
            }

            // Update System logic
            if last_update.elapsed() >= update_interval {
                self.core_usages = self.monitor.update();

                // Generate patterns
                self.patterns = self
                    .core_usages
                    .iter()
                    .map(|usage| {
                        let pulses = (*usage / 100.0 * 16.0).round() as usize;
                        bjorklund(16, pulses)
                    })
                    .collect();

                // Send to audio
                let _ = self.audio.patterns_tx.try_send(self.patterns.clone());

                last_update = Instant::now();
            }

            terminal.draw(|frame| self.draw(frame))?;

            // Input
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        // Grid for cores
        let core_count = self.patterns.len();
        if core_count == 0 {
            let msg = Paragraph::new("Initializing Monitor...")
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(msg, chunks[0]);
            // Don't return, allow footer to render
        } else {
            let cols = (core_count as f64).sqrt().ceil() as usize;
            let rows = (core_count as f64 / cols as f64).ceil() as usize;

            let row_constraints: Vec<Constraint> = (0..rows)
                .map(|_| Constraint::Ratio(1, rows as u32))
                .collect();
            let row_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(row_constraints)
                .split(chunks[0]);

            for (r, row_chunk) in row_chunks.iter().enumerate() {
                let col_constraints: Vec<Constraint> = (0..cols)
                    .map(|_| Constraint::Ratio(1, cols as u32))
                    .collect();
                let col_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(col_constraints)
                    .split(*row_chunk);

                for c in 0..cols {
                    let idx = r * cols + c;
                    if let Some(pattern) = self.patterns.get(idx) {
                        let usage = self.core_usages.get(idx).unwrap_or(&0.0);
                        self.render_core_circle(frame, col_chunks[c], idx, pattern, *usage);
                    }
                }
            }
        }

        // Footer
        let footer = Paragraph::new("Press 'q' to Quit | CPU Pulse Generator")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(footer, chunks[1]);
    }

    fn render_core_circle(
        &self,
        frame: &mut Frame,
        area: Rect,
        idx: usize,
        pattern: &[bool],
        usage: f32,
    ) {
        let canvas = Canvas::default()
            .block(
                Block::default()
                    .title(format!("Core {} ({:.0}%)", idx, usage))
                    .borders(Borders::ALL),
            )
            .x_bounds([-1.2, 1.2])
            .y_bounds([-1.2, 1.2])
            .paint(|ctx: &mut Context| {
                let step_angle = 2.0 * std::f64::consts::PI / 16.0;
                for i in 0..16 {
                    // Clockwise from top
                    let angle = std::f64::consts::PI / 2.0 - (i as f64 * step_angle);
                    let x = angle.cos();
                    let y = angle.sin();

                    let is_hit = pattern.get(i).copied().unwrap_or(false);
                    let is_current = i == self.current_step;

                    let color = if is_current {
                        Color::Red
                    } else if is_hit {
                        Color::Cyan
                    } else {
                        Color::DarkGray
                    };

                    let symbol = if is_current || is_hit { "●" } else { "·" };
                    ctx.print(x, y, Span::styled(symbol, Style::default().fg(color)));
                }
            });
        frame.render_widget(canvas, area);
    }
}
