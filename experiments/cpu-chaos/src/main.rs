pub mod chaos;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{collections::VecDeque, time::{Duration, Instant}};
use sysinfo::System;
use tui_shared::Tui;

use crate::chaos::{LyapunovEstimator, TimeDelayEmbedding};

const MAX_HISTORY: usize = 500;
const REFRESH_RATE: Duration = Duration::from_millis(50);

struct App {
    system: System,
    cpu_history: VecDeque<f64>,

    // Analysis State
    embedding: TimeDelayEmbedding,
    estimator: LyapunovEstimator,
    current_lambda: f64,
    reconstructed_points: Vec<Vec<f64>>,

    // UI State
    paused: bool,
    rotation_y: f64,
}

impl App {
    fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu(); // Initial refresh

        Self {
            system,
            cpu_history: VecDeque::with_capacity(MAX_HISTORY),
            embedding: TimeDelayEmbedding::new(3, 10), // Dim 3, Delay 10
            estimator: LyapunovEstimator::new(10), // Window 10
            current_lambda: 0.0,
            reconstructed_points: vec![],
            paused: false,
            rotation_y: 0.0,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        self.system.refresh_cpu();
        let cpu_usage = self.system.global_cpu_info().cpu_usage();

        // Normalize 0-100 to 0-1
        let val = (cpu_usage as f64) / 100.0;

        if self.cpu_history.len() >= MAX_HISTORY {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(val);

        // Perform Analysis if we have enough data
        let data: Vec<f64> = self.cpu_history.iter().cloned().collect();
        self.reconstructed_points = self.embedding.embed(&data);

        if !self.reconstructed_points.is_empty() {
             self.current_lambda = self.estimator.estimate(&self.reconstructed_points);
        }

        // Auto-rotate
        self.rotation_y += 0.02;
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = REFRESH_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    KeyCode::Char('r') => {
                        app.cpu_history.clear();
                        app.reconstructed_points.clear();
                    }
                    KeyCode::Up => app.embedding.delay += 1,
                    KeyCode::Down => {
                        if app.embedding.delay > 1 {
                            app.embedding.delay -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= REFRESH_RATE {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    draw_attractor(f, chunks[0], app);
    draw_stats(f, chunks[1], app);
}

fn draw_attractor(f: &mut Frame, area: Rect, app: &App) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Phase Space Reconstruction (Time-Delay Embedding)"),
        )
        .x_bounds([-1.5, 1.5]) // Normalized centered
        .y_bounds([-1.5, 1.5])
        .paint(|ctx| {
            // Draw axes
            ctx.draw(&CanvasLine { x1: -1.0, y1: 0.0, x2: 1.0, y2: 0.0, color: Color::DarkGray });
            ctx.draw(&CanvasLine { x1: 0.0, y1: -1.0, x2: 0.0, y2: 1.0, color: Color::DarkGray });

            for (i, p) in app.reconstructed_points.iter().enumerate() {
                if p.len() < 3 { continue; }

                // Center data (0..1 -> -0.5..0.5)
                let x = p[0] - 0.5;
                let y = p[1] - 0.5;
                let z = p[2] - 0.5;

                // Simple 3D Rotation (Y-axis)
                let rx = x * app.rotation_y.cos() - z * app.rotation_y.sin();
                let rz = x * app.rotation_y.sin() + z * app.rotation_y.cos();
                // Perspective projection (weak)
                let scale = 1.0 / (2.0 - rz * 0.5);
                let sx = rx * scale;
                let sy = y * scale;

                let color = if i < app.reconstructed_points.len() / 3 {
                    Color::DarkGray
                } else if i < app.reconstructed_points.len() * 2 / 3 {
                    Color::Blue
                } else {
                    Color::Cyan
                };

                // Draw point
                ctx.print(sx, sy, Span::styled("•", Style::default().fg(color)));
            }
        });

    f.render_widget(canvas, area);
}

fn draw_stats(f: &mut Frame, area: Rect, app: &App) {
    let lambda_color = if app.current_lambda > 0.1 {
        Color::Red // CHAOS
    } else if app.current_lambda > 0.0 {
        Color::Yellow // UNSTABLE
    } else {
        Color::Green // STABLE
    };

    let text = vec![
        Line::from(Span::styled(
            "Chaos Monitor",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("Lyapunov Exponent: "),
            Span::styled(
                format!("{:.4}", app.current_lambda),
                Style::default().fg(lambda_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Status: "),
            if app.current_lambda > 0.1 {
                 Span::styled("CHAOTIC", Style::default().fg(Color::Red))
            } else {
                 Span::styled("STABLE", Style::default().fg(Color::Green))
            }
        ]),
        Line::from(""),
        Line::from("Parameters:"),
        Line::from(format!("Delay (tau): {}", app.embedding.delay)),
        Line::from(format!("Dimension: {}", app.embedding.dimension)),
        Line::from(format!("History: {}", app.cpu_history.len())),
        Line::from(""),
        Line::from("Controls:"),
        Line::from("Space: Pause"),
        Line::from("Up/Down: Adjust Delay"),
        Line::from("R: Reset"),
        Line::from("Q: Quit"),
    ];

    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Analysis"));

    f.render_widget(p, area);
}
